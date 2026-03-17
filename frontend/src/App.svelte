<script lang="ts">
  import { onMount } from 'svelte'

  import { fetchJson, pickWorkspaceRoot, postJson } from './lib/api'
  import { deriveWorkspaceStats, formatDate, hashtags } from './lib/format'
  import type {
    AgentRecord,
    AutomationRecipe,
    BootstrapStatus,
    CacheRebuildReport,
    Course,
    DashboardSnapshot,
    DirectoryEntity,
    ExportRecord,
    FeedPost,
    GoogleActionPreview,
    LibraryItem,
    LiveSessionRecord,
    NotificationRecord,
    PageMeta,
    PageRecord,
    RecipePreview,
    SearchResponse,
    WorkspaceAuditReport,
  } from './lib/types'

  let dashboard: DashboardSnapshot | null = null
  let pages: PageMeta[] = []
  let feedPosts: FeedPost[] = []
  let employees: DirectoryEntity[] = []
  let projects: DirectoryEntity[] = []
  let documents: LibraryItem[] = []
  let videos: LibraryItem[] = []
  let courses: Course[] = []
  let automations: AutomationRecipe[] = []
  let agents: AgentRecord[] = []
  let googlePreviews: GoogleActionPreview[] = []
  let liveSessions: LiveSessionRecord[] = []
  let exports: ExportRecord[] = []
  let notifications: NotificationRecord[] = []
  let automationPreview: RecipePreview | null = null
  let bootstrapStatus: BootstrapStatus | null = null
  let cacheRebuild: CacheRebuildReport | null = null
  let selectedPageId = ''
  let selectedPage: PageRecord | null = null
  let searchQuery = 'launch'
  let searchResponse: SearchResponse | null = null
  let workspaceAudit: WorkspaceAuditReport | null = null
  let loading = true
  let working = false
  let error = ''
  let workspaceRootInput = ''

  let composer = {
    author_id: 'agent-herald',
    author_name: 'Herald',
    community_id: 'home',
    title: 'Studio update',
    body: '',
    hashtags: '#studio #launch',
    promoted: false,
  }

  let editor = {
    author: 'agent-editor',
    title: '',
    summary: '',
    body: '',
    bullets: '',
  }

  const loadPage = async (pageId: string) => {
    selectedPageId = pageId
    selectedPage = await fetchJson<PageRecord>(`/api/pages/${pageId}`)
    editor = {
      author: 'agent-editor',
      title: selectedPage.published_revision.title,
      summary: selectedPage.published_revision.summary,
      body: selectedPage.published_revision.blocks[0]?.body ?? '',
      bullets: (selectedPage.published_revision.blocks[1]?.items ?? []).join('\n'),
    }
  }

  const saveDraft = async () => {
    if (!selectedPageId) {
      return
    }

    working = true
    error = ''

    try {
      await postJson(`/api/pages/${selectedPageId}/drafts`, {
        author: editor.author,
        title: editor.title,
        summary: editor.summary,
        blocks: [
          {
            kind: 'heading',
            title: editor.title,
            body: editor.body,
            items: [],
          },
          {
            kind: 'callout',
            title: 'Draft focus',
            body: editor.summary,
            items: editor.bullets
              .split('\n')
              .map((entry) => entry.trim())
              .filter(Boolean),
          },
        ],
      })

      await loadWorkspace()
    } catch (draftError) {
      error = draftError instanceof Error ? draftError.message : 'Failed to save draft.'
    } finally {
      working = false
    }
  }

  const submitSearch = async () => {
    if (!searchQuery.trim()) {
      searchResponse = null
      return
    }

    searchResponse = await fetchJson<SearchResponse>(
      `/api/search?q=${encodeURIComponent(searchQuery.trim())}`,
    )
  }

  const loadWorkspace = async () => {
    loading = true
    error = ''

    try {
      const [
        dashboardResponse,
        bootstrapResponse,
        pageResponse,
        feedResponse,
        employeeResponse,
        projectResponse,
        documentResponse,
        videoResponse,
        courseResponse,
        automationResponse,
        auditResponse,
      ] = await Promise.all([
        fetchJson<DashboardSnapshot>('/api/dashboard'),
        fetchJson<BootstrapStatus>('/api/bootstrap/status'),
        fetchJson<PageMeta[]>('/api/pages'),
        fetchJson<FeedPost[]>('/api/feed'),
        fetchJson<DirectoryEntity[]>('/api/directories/employees'),
        fetchJson<DirectoryEntity[]>('/api/directories/projects'),
        fetchJson<LibraryItem[]>('/api/libraries/documents'),
        fetchJson<LibraryItem[]>('/api/libraries/videos'),
        fetchJson<Course[]>('/api/courses'),
        fetchJson<AutomationRecipe[]>('/api/automations'),
        fetchJson<WorkspaceAuditReport>('/api/sync/audit'),
      ])

      dashboard = dashboardResponse
      bootstrapStatus = bootstrapResponse
      workspaceRootInput = bootstrapResponse.workspace_root
      pages = pageResponse
      feedPosts = feedResponse
      employees = employeeResponse
      projects = projectResponse
      documents = documentResponse
      videos = videoResponse
      courses = courseResponse
      automations = automationResponse
      agents = dashboardResponse.agents
      googlePreviews = dashboardResponse.google_previews
      liveSessions = dashboardResponse.live_sessions
      exports = dashboardResponse.exports
      notifications = dashboardResponse.notifications
      workspaceAudit = auditResponse

      if (pages[0]) {
        await loadPage(selectedPageId || pages[0].id)
      }

      if (automationResponse[0]) {
        automationPreview = await fetchJson<RecipePreview>(
          `/api/automations/${automationResponse[0].id}/preview`,
        )
      }

      await submitSearch()
    } catch (loadError) {
      error = loadError instanceof Error ? loadError.message : 'Failed to load WEAVE.'
    } finally {
      loading = false
    }
  }

  const rebuildCache = async () => {
    working = true
    error = ''

    try {
      cacheRebuild = await postJson<CacheRebuildReport, Record<string, never>>('/api/sync/rebuild', {})
      await loadWorkspace()
    } catch (rebuildError) {
      error = rebuildError instanceof Error ? rebuildError.message : 'Failed to rebuild cache.'
    } finally {
      working = false
    }
  }

  const selectWorkspaceRoot = async (path: string) => {
    if (!path.trim()) {
      return
    }

    working = true
    error = ''

    try {
      bootstrapStatus = await postJson<BootstrapStatus, { path: string }>(
        '/api/bootstrap/workspace-root',
        {
          path: path.trim(),
        },
      )
      workspaceRootInput = bootstrapStatus.workspace_root
      await loadWorkspace()
    } catch (rootError) {
      error = rootError instanceof Error ? rootError.message : 'Failed to switch workspace root.'
    } finally {
      working = false
    }
  }

  const pickAndSelectWorkspaceRoot = async () => {
    const pickedRoot = await pickWorkspaceRoot()
    if (!pickedRoot) {
      return
    }

    await selectWorkspaceRoot(pickedRoot)
  }

  const createPost = async () => {
    working = true
    error = ''

    try {
      await postJson<FeedPost, unknown>('/api/feed', {
        ...composer,
        hashtags: composer.hashtags
          .split(/\s+/)
          .map((tag) => tag.replace(/^#/, '').trim())
          .filter(Boolean),
      })

      composer.body = ''
      await loadWorkspace()
    } catch (postError) {
      error = postError instanceof Error ? postError.message : 'Failed to post to feed.'
    } finally {
      working = false
    }
  }

  const publishPage = async () => {
    if (!selectedPageId) {
      return
    }

    working = true
    error = ''

    try {
      await postJson<PageRecord, unknown>(`/api/pages/${selectedPageId}/publish`, {
        author: editor.author,
        title: editor.title,
        summary: editor.summary,
        blocks: [
          {
            kind: 'heading',
            title: editor.title,
            body: editor.body,
            items: [],
          },
          {
            kind: 'callout',
            title: 'Focus',
            body: editor.summary,
            items: editor.bullets
              .split('\n')
              .map((entry) => entry.trim())
              .filter(Boolean),
          },
        ],
      })

      await loadWorkspace()
    } catch (publishError) {
      error = publishError instanceof Error ? publishError.message : 'Failed to publish page.'
    } finally {
      working = false
    }
  }

  onMount(loadWorkspace)

  $: stats = dashboard ? deriveWorkspaceStats(dashboard) : []
</script>

<svelte:head>
  <title>WEAVE</title>
</svelte:head>

{#if loading}
  <main class="shell">
    <section class="loading-card">
      <p class="eyebrow">WEAVE</p>
      <h1>Loading the studio workspace…</h1>
    </section>
  </main>
{:else if dashboard}
  <main class="shell">
    <header class="masthead">
      <div>
        <p class="eyebrow">WEAVE</p>
        <h1>{dashboard.workspace.name}</h1>
        <p class="lede">{dashboard.workspace.tagline}</p>
      </div>

      <div class="health-pill">
        <span>{dashboard.sync_health.drive_mode}</span>
        <span>{dashboard.sync_health.ownership_mode}</span>
      </div>
    </header>

    <section class="hero-grid">
      <article class="hero-card statement">
        <p class="eyebrow">North Star</p>
        <h2>Pages, social rhythm, search, and Google-side actions in one quiet surface.</h2>
        <p>
          The demo workspace is file-backed, locally searchable, and already wired for promoted
          posts, editorial revisions, learning assignments, and automation previews.
        </p>
        <div class="stat-row">
          {#each stats as stat}
            <div class="stat-chip">
              <span>{stat.value}</span>
              <small>{stat.label}</small>
            </div>
          {/each}
        </div>
      </article>

      <article class="hero-card search-card">
        <div class="section-head">
          <div>
            <p class="eyebrow">AI Search</p>
            <h3>Grounded answers with citations</h3>
          </div>
          <button class="ghost" type="button" on:click={submitSearch}>Refresh</button>
        </div>
        <label class="field">
          <span>Search the workspace</span>
          <input bind:value={searchQuery} placeholder="Search pages, posts, people, and courses" />
        </label>
        {#if searchResponse}
          <div class="answer-pane">
            <p>{searchResponse.answer.summary}</p>
            <div class="citation-row">
              {#each searchResponse.answer.citations as citation}
                <span>{citation.label}</span>
              {/each}
            </div>
          </div>
          <ul class="search-list stack compact-stack">
            {#each searchResponse.results.slice(0, 6) as result}
              <li class="mini-card">
                <strong>{result.title}</strong>
                <span>{result.kind}</span>
                <p>{result.snippet}</p>
              </li>
            {/each}
          </ul>
        {/if}
      </article>

      {#if bootstrapStatus}
        <article class="hero-card search-card">
          <div class="section-head">
            <div>
              <p class="eyebrow">Bootstrap</p>
              <h3>Operator setup visibility</h3>
            </div>
          </div>

          <ul class="stack compact-stack">
            <li class="mini-card">
              <strong>{bootstrapStatus.gws_installed ? 'gws found' : 'gws missing'}</strong>
              <span>{bootstrapStatus.gws_version ?? 'Install and auth are still pending.'}</span>
              <p>The cold-path Google control plane is checked here before export and delivery work.</p>
            </li>
            <li class="mini-card">
              <strong>{bootstrapStatus.gemini_configured ? 'Gemini ready' : 'Gemini not configured'}</strong>
              <span>{bootstrapStatus.gemini_source}</span>
              <p>The broker can stay local-first until a secured Gemini credential is present.</p>
            </li>
            <li class="mini-card">
              <strong>Workspace root</strong>
              <span>{bootstrapStatus.workspace_root}</span>
              <p>Demo root: {bootstrapStatus.demo_workspace_root}</p>
            </li>
          </ul>

          <div class="editor-grid">
            <label class="field compact">
              <span>Selected workspace root</span>
              <input bind:value={workspaceRootInput} placeholder="/path/to/WEAVE" />
            </label>
            <div class="button-row">
              <button
                class="ghost"
                type="button"
                disabled={working}
                on:click={() => selectWorkspaceRoot(bootstrapStatus?.demo_workspace_root ?? '')}
              >
                Use demo root
              </button>
              <button class="ghost" type="button" disabled={working} on:click={pickAndSelectWorkspaceRoot}>
                Choose folder
              </button>
              <button
                class="accent"
                type="button"
                disabled={working}
                on:click={() => selectWorkspaceRoot(workspaceRootInput)}
              >
                Persist root
              </button>
            </div>
            <div class="preview-box">
              <strong>Bootstrap config</strong>
              <p>{bootstrapStatus.config_path}</p>
              <p>{bootstrapStatus.workspace_root_persisted ? 'Persisted across restarts' : 'Using in-memory default only'}</p>
            </div>
          </div>
        </article>
      {/if}
    </section>

    {#if error}
      <p class="error-banner">{error}</p>
    {/if}

    <section class="content-grid">
      <article class="panel feed-panel">
        <div class="section-head">
          <div>
            <p class="eyebrow">Feed</p>
            <h3>Promoted activity and current rhythm</h3>
          </div>
          <span class="count-pill">{feedPosts.length} posts</span>
        </div>

        <div class="composer">
          <label class="field compact">
            <span>Title</span>
            <input bind:value={composer.title} />
          </label>
          <label class="field compact">
            <span>Body</span>
            <textarea bind:value={composer.body} rows="3"></textarea>
          </label>
          <label class="field compact">
            <span>Hashtags</span>
            <input bind:value={composer.hashtags} />
          </label>
          <label class="toggle">
            <input bind:checked={composer.promoted} type="checkbox" />
            <span>Promote this post</span>
          </label>
          <button class="accent" type="button" disabled={working} on:click={createPost}>
            Publish post
          </button>
        </div>

        <ul class="stack">
          {#each feedPosts.slice(0, 5) as post}
            <li class="stream-item">
              <div class="item-head">
                <strong>{post.title}</strong>
                {#if post.promoted}
                  <span class="pill warm">Promoted</span>
                {/if}
              </div>
              <p>{post.body}</p>
              <div class="meta-row">
                <span>{post.author_name}</span>
                <span>{hashtags(post.hashtags)}</span>
                <span>{post.comments} comments</span>
                <span>{post.likes} likes</span>
              </div>
            </li>
          {/each}
        </ul>
      </article>

      <article class="panel editorial-panel">
        <div class="section-head">
          <div>
            <p class="eyebrow">Editorial</p>
            <h3>Publish directly into canonical page revisions</h3>
          </div>
        </div>

        <div class="page-tabs">
          {#each pages as page}
            <button
              class:active={page.id === selectedPageId}
              type="button"
              on:click={() => loadPage(page.id)}
            >
              {page.title}
            </button>
          {/each}
        </div>

        {#if selectedPage}
          <div class="page-preview">
            <h4>{selectedPage.published_revision.title}</h4>
            <p>{selectedPage.published_revision.summary}</p>
            <small>Published by {selectedPage.published_revision.author}</small>
          </div>

          <div class="editor-grid">
            <label class="field compact">
              <span>Title</span>
              <input bind:value={editor.title} />
            </label>
            <label class="field compact">
              <span>Summary</span>
              <textarea bind:value={editor.summary} rows="3"></textarea>
            </label>
            <label class="field compact">
              <span>Lead block</span>
              <textarea bind:value={editor.body} rows="4"></textarea>
            </label>
            <label class="field compact">
              <span>Callout bullets</span>
              <textarea bind:value={editor.bullets} rows="4"></textarea>
            </label>
            <div class="button-row">
              <button class="ghost" type="button" disabled={working} on:click={saveDraft}>
                Save draft
              </button>
              <button class="accent" type="button" disabled={working} on:click={publishPage}>
                Publish revision
              </button>
            </div>
          </div>

          {#if selectedPage.drafts.length}
            <div class="draft-panel">
              <p class="eyebrow">Drafts</p>
              <ul class="stack compact-stack">
                {#each selectedPage.drafts as draft}
                  <li class="mini-card">
                    <strong>{draft.title}</strong>
                    <span>{draft.author} · {formatDate(draft.updated_at)}</span>
                    <p>{draft.summary}</p>
                  </li>
                {/each}
              </ul>
            </div>
          {/if}
        {/if}
      </article>

      <article class="panel">
        <div class="section-head">
          <div>
            <p class="eyebrow">Directories</p>
            <h3>People and project memory</h3>
          </div>
        </div>

        <div class="card-grid">
          {#each employees.slice(0, 3) as person}
            <div class="mini-card">
              <strong>{person.name}</strong>
              <span>{person.title}</span>
              <p>{person.summary}</p>
            </div>
          {/each}
        </div>

        <div class="card-grid">
          {#each projects.slice(0, 2) as project}
            <div class="mini-card soft">
              <strong>{project.name}</strong>
              <span>{project.location}</span>
              <p>{project.summary}</p>
            </div>
          {/each}
        </div>
      </article>

      <article class="panel">
        <div class="section-head">
          <div>
            <p class="eyebrow">Libraries</p>
            <h3>Featured docs and video</h3>
          </div>
        </div>
        <ul class="stack compact-stack">
          {#each [...documents.slice(0, 2), ...videos.slice(0, 2)] as item}
            <li class="asset-row">
              <div>
                <strong>{item.title}</strong>
                <p>{item.description}</p>
              </div>
              <small>{item.kind}</small>
            </li>
          {/each}
        </ul>
      </article>

      <article class="panel">
        <div class="section-head">
          <div>
            <p class="eyebrow">Learning</p>
            <h3>Courses and due work</h3>
          </div>
        </div>
        <ul class="stack compact-stack">
          {#each courses as course}
            <li class="course-row">
              <div>
                <strong>{course.title}</strong>
                <p>{course.summary}</p>
              </div>
              <small>{formatDate(course.assignment_due)}</small>
            </li>
          {/each}
        </ul>
      </article>

      <article class="panel automation-panel">
        <div class="section-head">
          <div>
            <p class="eyebrow">Automation</p>
            <h3>Dry-run command shadows</h3>
          </div>
        </div>
        <ul class="stack compact-stack">
          {#each automations as recipe}
            <li class="mini-card">
              <strong>{recipe.name}</strong>
              <span>{recipe.trigger}</span>
              <p>{recipe.description}</p>
            </li>
          {/each}
        </ul>
        {#if automationPreview}
          <div class="preview-box">
            <strong>{automationPreview.name}</strong>
            <code>{automationPreview.command_preview}</code>
            <p>{automationPreview.payload_preview}</p>
          </div>
        {/if}
      </article>

      <article class="panel">
        <div class="section-head">
          <div>
            <p class="eyebrow">Agents</p>
            <h3>First-class workspace members</h3>
          </div>
        </div>
        <ul class="stack compact-stack">
          {#each agents as agent}
            <li class="mini-card">
              <strong>{agent.name}</strong>
              <span>{agent.preferred_model}</span>
              <p>{agent.bio}</p>
            </li>
          {/each}
        </ul>
      </article>

      <article class="panel automation-panel">
        <div class="section-head">
          <div>
            <p class="eyebrow">Google Coordination</p>
            <h3>Operator-visible secondary surfaces</h3>
          </div>
        </div>
        <ul class="stack compact-stack">
          {#each googlePreviews as preview}
            <li class="mini-card">
              <strong>{preview.title}</strong>
              <span>{preview.surface}</span>
              <p>{preview.summary}</p>
              <code>{preview.command_preview}</code>
            </li>
          {/each}
        </ul>
      </article>

      <article class="panel">
        <div class="section-head">
          <div>
            <p class="eyebrow">Live Learning</p>
            <h3>Calendar-backed sessions</h3>
          </div>
        </div>
        <ul class="stack compact-stack">
          {#each liveSessions as session}
            <li class="mini-card">
              <strong>{session.title}</strong>
              <span>{formatDate(session.starts_at)} · {session.duration_minutes} min</span>
              <p>{session.meet_enabled ? 'Meet enabled' : 'Calendar only'} · {session.status}</p>
            </li>
          {/each}
        </ul>
      </article>

      <article class="panel">
        <div class="section-head">
          <div>
            <p class="eyebrow">Exports</p>
            <h3>Docs, Sheets, and Slides satellites</h3>
          </div>
        </div>
        <ul class="stack compact-stack">
          {#each exports as exportRecord}
            <li class="mini-card">
              <strong>{exportRecord.title}</strong>
              <span>{exportRecord.kind} · {exportRecord.status}</span>
              <p>{exportRecord.destination_hint}</p>
            </li>
          {/each}
        </ul>
      </article>

      <article class="panel">
        <div class="section-head">
          <div>
            <p class="eyebrow">Notifications</p>
            <h3>Cross-surface queue state</h3>
          </div>
        </div>
        <ul class="stack compact-stack">
          {#each notifications as notification}
            <li class="mini-card">
              <strong>{notification.title}</strong>
              <span>{notification.channel} · {notification.state}</span>
              <p>{notification.body}</p>
            </li>
          {/each}
        </ul>
      </article>

      <article class="panel automation-panel">
        <div class="section-head">
          <div>
            <p class="eyebrow">Workspace Engine</p>
            <h3>Sync health, cache rebuilds, and repair visibility</h3>
          </div>
          <button class="ghost" type="button" disabled={working} on:click={rebuildCache}>
            Rebuild cache
          </button>
        </div>

        <div class="card-grid">
          <div class="mini-card">
            <strong>{dashboard.sync_health.unresolved_conflicts}</strong>
            <span>Conflict copies</span>
            <p>Canonical files stay intact while merge work stays visible.</p>
          </div>
          <div class="mini-card">
            <strong>{dashboard.sync_health.stale_cache_count}</strong>
            <span>Stale cache families</span>
            <p>Counts drift until the local cache is rebuilt from canonical files.</p>
          </div>
          <div class="mini-card">
            <strong>{dashboard.sync_health.lost_and_found_items}</strong>
            <span>Lost and Found items</span>
            <p>Anything stranded outside the expected layout surfaces here.</p>
          </div>
          <div class="mini-card">
            <strong>{dashboard.sync_health.workspace_audit_issue_count}</strong>
            <span>Audit issues</span>
            <p>Reference gaps, orphaned drafts, and missing paths are counted together.</p>
          </div>
        </div>

        <div class="preview-box">
          <strong>Current local state</strong>
          <p>Workspace root: {dashboard.sync_health.workspace_root}</p>
          <p>Decryption: {dashboard.sync_health.decryption_state}</p>
          <p>Relay: {dashboard.sync_health.relay_connectivity}</p>
          <p>
            Cache rebuilt:
            {#if dashboard.sync_health.last_cache_rebuild}
              {formatDate(dashboard.sync_health.last_cache_rebuild)}
            {:else}
              Not yet
            {/if}
          </p>
        </div>

        {#if cacheRebuild}
          <div class="preview-box">
            <strong>Last rebuild</strong>
            <p>{formatDate(cacheRebuild.rebuilt_at)} · {cacheRebuild.issue_count} audit issue(s)</p>
            <code>{cacheRebuild.sqlite_path}</code>
          </div>
        {/if}

        {#if workspaceAudit}
          <ul class="stack compact-stack">
            {#each [
              ...workspaceAudit.invalid_paths,
              ...workspaceAudit.missing_references,
              ...workspaceAudit.orphaned_drafts,
              ...workspaceAudit.conflict_copies,
              ...workspaceAudit.lost_and_found_items,
            ].slice(0, 6) as syncIssue}
              <li class="mini-card">
                <strong>{syncIssue.code}</strong>
                <span>{syncIssue.severity}</span>
                <p>{syncIssue.message}</p>
              </li>
            {/each}
          </ul>
        {/if}
      </article>
    </section>
  </main>
{/if}
