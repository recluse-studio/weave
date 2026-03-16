# WEAVE SPEC

## 1. Purpose

This document is the exhaustive build specification for WEAVE.

WEAVE is a Google Workspace-native, local-first social intranet. It combines structured editorial content, a real social layer, metadata-rich libraries and directories, integrated learning, citation-rich search, and first-class automation and agent workflows.

This specification is written so that a capable long-running implementation model can build the entire project end to end without pausing for routine clarification. If a detail is underspecified, the implementing model must choose the most conservative option that preserves the product principles, document the choice, and continue.

`weave.md` is the concise product brief and directional canon. This file is the implementation canon. If the two differ, preserve the product direction in `weave.md` and the execution detail in `SPEC.md`.

## 2. Execution Contract For An Autonomous Builder

The implementing model must:

- build the repository from an empty or near-empty starting state
- create the monorepo structure, scripts, CI, fixtures, docs, and initial application code
- work milestone by milestone without stopping at the end of each one
- prefer vertical slices that leave the product runnable after each increment
- make reasonable implementation choices without waiting for approval unless a choice is destructive, irreversible, or credential-bound
- keep the demo workspace usable throughout the build
- update documentation when decisions materially affect architecture or operating rules
- create frequent micro commits with descriptive, Calvino-inflected mythopoetic commit messages
- run quality, security, and visual checks repeatedly instead of deferring them to the end

The implementing model must not:

- introduce a mandatory hosted backend
- move canonical truth into SQL, a hosted document store, or Google-native document types
- replace structured blocks with generic HTML blobs
- shell out to `gws` on hot paths
- leave secret values hard-coded in the repository
- stop after planning when implementation is feasible

If external credentials are missing, the model must still implement the local architecture, UI flows, mocks, adapters, fixtures, and activation hooks, then leave one precise activation note for the missing credential step.

## 3. Non-Negotiables

- Canonical truth lives in ordinary files inside Google Drive.
- Google Drive is the source of truth for content, settings, automation recipes, agent definitions, and workspace metadata.
- SQLite is cache only.
- Search indexes are cache only.
- Embeddings are cache only.
- Thumbnails, extracted text, transcripts, and object caches are cache only.
- Google Docs, Sheets, and Slides are secondary surfaces, never canonical page objects.
- Gmail is the outbound mail surface.
- Google Calendar is the event surface.
- Google Chat is the mirror and alert surface, not the canonical feed.
- `gws` is the official operator shell and control-plane adapter.
- Gemini is the runtime model layer.
- Every Google-side UI action must have an equivalent command shadow or recipe representation.
- Every automation must support dry-run preview.
- The desktop app is primary. Browser mode is secondary.
- Tier-1 end-user platforms are macOS and Windows.
- My Drive must have a first-class experience.
- Shared drives must have a first-class experience where available.
- The relay is optional and never canonical.

## 4. Product Definition

WEAVE must feel like three systems folded into one calm product:

- an editorial workspace for handbooks, guides, standards, policies, knowledge bases, and project documentation
- a social workspace for posts, comments, hashtags, celebrations, promoted updates, and weekly rhythm
- a Google Workspace cockpit for exports, mirrors, reminders, scheduling, routing, and governance

The app should open like a bright, orderly studio. The home page should feel alive before the user clicks anything, but never noisy. Search should feel indispensable. The right rail should earn its presence. Agents should be useful and legible, not mysterious.

## 5. Supported Account And Ownership Profiles

WEAVE must work well in these profiles:

### 5.1 Personal Google account

- Workspace lives in My Drive.
- Shared drives and Admin APIs may be unavailable.
- Gmail, Calendar, Docs, Sheets, Slides, and Gemini may still be available.
- The solo experience must feel complete rather than degraded.

### 5.2 Google Workspace user

- Workspace can live in My Drive or a shared drive.
- Groups, Chat, richer domain sharing, and organizational collaboration features may be available.
- Shared drives are team-owned and stream-only.

### 5.3 Google Workspace admin

- Workspace can use shared drives, Groups, Admin Directory sync, service-account-backed relay jobs, and admin-managed label and governance features.
- Drive Labels are optional org-mode enhancements, not a baseline dependency.

## 6. Runtime Modes

### 6.1 Personal workspace

- Mirrored My Drive root.
- Full desktop-first local experience.
- Best for one person or a very small team.

### 6.2 Shared My Drive workspace

- My Drive folder shared to collaborators.
- Useful for lightweight teams and mixed internal or external groups.

### 6.3 Team-owned shared drive workspace

- Shared drive provides continuity and team ownership.
- Requires stream-aware local caching for index and offline behavior.

### 6.4 Managed relay workspace

- Optional relay receives Drive watch notifications, runs scheduled recipes, sends digests, mirrors to Chat, handles heavy jobs, and supports browser mode.
- Relay never becomes the source of truth.

## 7. Architecture

### 7.1 Core stack

- Rust workspace for storage, sync, search, job orchestration, Google clients, `gws` adapter, exports, agent tools, Gemini broker, analytics, privacy, and automation runtime
- Tauri 2 desktop shell
- Axum local API and service boundary
- Svelte 5 frontend
- SQLite for rebuildable metadata cache, queues, lightweight materialized views, and local analytics storage
- Tantivy for lexical and faceted search
- local vector index for semantic retrieval
- `ffmpeg` for media extraction
- local ASR such as `whisper.cpp` for default transcription
- Gemini for reasoning, summarization, tagging, drafting, classification, answer synthesis, translation, chaptering, and enrichment
- optional Wasmtime-backed plugin runtime for connectors and controlled extensions

### 7.2 Hot path rules

The following paths must not shell out to `gws`:

- app startup
- page open
- feed open and scroll
- local search
- local indexing
- Drive change-log replay
- typing in editors
- rendering guides, libraries, profiles, and directories

Use direct APIs or local state for hot paths. Use `gws` for operator flows, recipes, exports, delivery, debugging, support bundles, and command shadows.

### 7.3 Desktop-first rule

The desktop app is the primary product surface. The relay and browser mode exist to extend the product, not to redefine it.

## 8. Canonical Storage Model

Use ordinary files. Keep them legible. Keep them stable. Keep them reconstructable.

Use:

- immutable revisions where history matters
- append-only JSONL segments where volume and concurrency matter
- tiny mutable pointers where navigation and publication state matter

Suggested workspace layout:

```text
WEAVE/
  weave.toml
  AGENTS.md
  workspace/
    identity.json
    google.json
    settings.json
  communities/
    home.json
    hr.json
    learning.json
  members/
    drew.json
    agent-archivist.json
    agent-herald.json
  pages/
    page-01/
      meta.json
      published.ref
      revisions/
        2026-03-16T09-00-12Z_drew.json
        2026-03-17T14-22-09Z_editor.json
      drafts/
        drew.json
        editor.json
  guides/
    handbook.json
    benefits.json
  templates/
    home.json
    policy.json
    learning-center.json
  directories/
    employees/
      schema.json
      entities/
        emp-001.json
        emp-002.json
    projects/
      schema.json
      entities/
        proj-001.json
  libraries/
    documents/
      library.json
      items/
        doc-001.json
    videos/
      library.json
      items/
        vid-001.json
    images/
      library.json
      items/
        img-001.json
  assets/
    documents/
    videos/
    images/
  courses/
    catalog.json
    course-001.json
    lesson-001.json
    learning-path-001.json
    assignments/
      assign-001.json
  feed/
    segments/
      2026/
        03/
          16/
            feed-0001.jsonl
  analytics/
    segments/
      2026/
        03/
          16/
            events-0001.jsonl
  agents/
    archivist/
      agent.json
      prompt.md
    herald/
      agent.json
      prompt.md
    coach/
      agent.json
      prompt.md
  automations/
    recipes/
      weekly-digest.yaml
      post-promotion-chat-mirror.yaml
    schedules.json
  exports/
    docs/
    sheets/
    slides/
  themes/
    theme.json
    dark.json
  attachments/
    external-links.json
  connectors/
    imports/
    mappings/
```

Rebuildable local state must stay outside Drive:

```text
appdata/
  workspaces/<workspace-id>/
    cache.sqlite
    search/
      tantivy/
      vectors/
    object_cache/
    extracted_text/
    thumbs/
    transcripts/
    jobs/
    logs/
```

Never place indexes, lock files, temp files, or machine-specific caches in the synced Drive workspace.

### 8.1 Canonical file contract

Every canonical JSON or YAML object file must carry a stable metadata envelope. At minimum, each canonical object must include:

- `schema_version`
- `object_type`
- `id`
- `slug` where human-readable routing matters
- `title` where human-facing naming matters
- `created_at`
- `updated_at`
- `created_by`
- `updated_by`
- `permission_scope`
- `provenance` for imported or connector-derived records

Rules:

- `schema_version` must be explicit and monotonically increasing per object family
- readers must be tolerant across at least one previous schema version where practical
- migrations must be additive first and destructive only with explicit repair tooling
- timestamps must use a stable, sortable format
- IDs must be durable and never rewritten during ordinary edits
- file names may change; object IDs may not

### 8.2 Migration and repair strategy

The repository must include code paths and scripts for:

- full workspace rebuild from canonical files
- targeted cache rebuild for one workspace, community, page, library, or directory
- schema migration preview
- schema migration execution
- orphan detection
- missing-reference detection
- conflict-copy enumeration
- Lost and Found recovery

If a migration fails partway through, the system must fail safely, preserve pre-migration files, and leave a resumable repair path.

## 9. Canonical Objects And Satellite Objects

Canonical objects include:

- page revisions and drafts
- guide definitions
- templates
- community definitions
- directory schemas and entity records
- profiles and mapped profile extensions
- library metadata and item metadata
- course, lesson, learning-path, assignment, transcript, and cohort objects
- feed events
- analytics event segments
- agent definitions and prompts
- automation recipes and schedules
- theme definitions
- workspace identity and settings
- connector mappings and imported normalized records

Satellite objects include:

- Gmail messages and drafts
- Calendar events
- Meet links
- Chat messages
- Google Doc exports and imports
- Google Sheet exports and imports
- Google Slide exports
- Drive Labels
- Drive Activity views
- local indexes and embeddings
- generated thumbnails and transcripts
- relay queues and relay-only job state

If a machine is replaced or local caches are deleted, WEAVE must reconstruct working state from canonical files plus fresh Google API lookups where necessary.

## 10. Filesystem And Google Drive Constraints

Design for real Drive semantics, not database semantics.

- File changes may arrive out of order.
- Rename events may be noisy.
- Conflict copies may appear.
- Some content may exist only as placeholders until locally hydrated.
- Shared-drive content is stream-oriented.
- My Drive can be mirrored or streamed.
- Shared drives can only be streamed.
- Drive folder paths may move under macOS File Provider behavior.
- The app must track the workspace by Drive file ID and local file identity, not by guessed path strings.

Google-native Docs, Sheets, and Slides file pointers must never be edited as canonical content. Use them as exports, mirrors, attachments, and collaborative satellites only.

High-churn events must not become one-file-per-like or one-file-per-view. Use JSONL segment files for feed events, notifications, analytics deltas, audit events, and similar write-heavy streams.

## 11. Sync Engine

The sync engine has three layers:

### 11.1 Local watcher

- Watch the chosen local root for materialized changes.
- Update cache, indexes, and UI incrementally.
- Debounce noisy change bursts.

### 11.2 Drive reconciliation

- Use Drive change-log replay based on start page tokens.
- Replay both user and shared-drive change logs where required.
- Detect missed local events and recover from change-log replays.

### 11.3 Relay wake-up

- In relay mode, use Drive watch channels as signals that new changes exist.
- Always pull actual changes after a watch notification.

### 11.4 Sync rules

- social activity is append-only
- page publishing writes immutable revisions
- drafts are user-scoped
- editing leases are advisory, not absolute
- conflict copies are preserved
- merge flows are explicit
- local indexes rebuild from canonical files
- streamed shared-drive content may be copied into the local object cache for indexing and offline reads

### 11.4.1 Sync correctness requirements

- local watch handling must be idempotent
- change-log replay must be idempotent
- replay after partial failure must be safe
- a replayed object must not duplicate analytics, feed, or notification events
- the cache layer must be disposable without affecting canonical content
- missing local materialization of streamed content must degrade search and preview gracefully rather than corrupting state
- merge screens must preserve both competing drafts until a human or explicit merge tool resolves them

### 11.5 Sync-health panel

The app must expose:

- chosen workspace root
- Drive mode: mirrored or streamed
- ownership mode: My Drive or shared drive
- last local scan
- last change-log replay
- stale cache count
- unresolved conflict copies
- Lost and Found items
- decryption state for private scopes
- export queue backlog
- relay connectivity state when relay mode is enabled

## 12. Permissions, Roles, Privacy, And Encryption

Use Google sharing as the coarse permission plane and WEAVE roles as the fine-grained behavior plane.

### 12.1 Roles

- owner
- workspace admin
- community manager
- editor
- member
- viewer
- agent

### 12.2 Scoped admin flags

- branding admin
- stream admin
- search admin
- learning admin
- template admin
- analytics admin

### 12.3 Permission sources in precedence order

1. Google account identity
2. Google Group membership
3. folder or shared-drive ACL
4. limited-access subfolder rules
5. app-layer private-scope encryption
6. app-local behavioral overrides

### 12.4 Privacy model

- public or ordinary workspace content may remain unencrypted
- private communities, restricted assets, and private agent memory must support encryption at rest inside the Drive workspace
- unauthorized users may sync encrypted blobs but must not decrypt or index them
- search indexes only decrypted content on authorized machines
- agents inherit explicit permission scopes and may not roam outside them

### 12.5 Encryption implementation

- use per-scope symmetric content keys
- wrap scope keys to authorized user public keys
- rotate scope keys lazily on membership changes
- separate transport credentials from content encryption keys
- never write plaintext secrets into canonical content files

## 13. Authentication, Credentials, And Scopes

Developer and operator prerequisites:

- Google Drive for desktop on macOS or Windows
- `gws` installed and pinned to a tested version or commit
- Google OAuth configured through `gws auth setup` or manual setup
- Gemini credential path configured
- chosen Drive workspace root

Rules:

- request Google scopes progressively rather than all at once
- start with Drive and identity scopes
- add Gmail, Calendar, Chat, Docs, Sheets, Slides, Admin, People, Groups, or Picker scopes only when the corresponding feature is enabled
- store secrets in OS keychain or encrypted credential storage, never in repo files
- default Gemini setup to `GEMINI_API_KEY` backed by the OS keychain or secure local secret store

## 14. Google Workspace Control Plane

Treat `gws` as the official operator shell.

Use `gws` for:

- auth bootstrap
- operator actions
- Gmail delivery and draft creation
- Calendar writes
- Chat mirroring
- Docs, Sheets, and Slides exports
- Group and directory sync
- admin recipes
- debugging and support bundles
- reproducible CLI shadows of UI actions

The UI must expose the equivalent `gws` command preview, recipe preview, or structured action summary for each Google-side operation that matters to operators.

## 15. Google Suite Mapping

### 15.1 Drive

- canonical storage
- sharing and ownership boundaries
- asset hosting
- change tracking
- attachment source

### 15.2 Gmail

- immediate notifications
- moderated announcements
- weekly digests
- approval requests
- optional reply-by-email comment flows

### 15.3 Calendar and Meet

- live learning sessions
- office hours
- onboarding schedules
- due-date overlays
- optional agent reminders

### 15.4 Chat

- promoted post mirroring
- urgent notices
- agent notices
- learning reminders

### 15.5 Docs

- export page to Doc
- export guide sections
- collaborative burst mode
- import Doc to draft

### 15.6 Sheets

- directory exports
- library exports
- assignment rosters
- taxonomy audits
- analytics tabs
- bulk staging tables for controlled imports

### 15.7 Slides

- training decks
- onboarding decks
- benefits decks
- guide-to-deck exports

### 15.8 People, Groups, And Admin

- profile hydration
- avatars and titles
- org structure
- managers and office locations
- team membership
- permission cohorts

### 15.9 Drive Labels And Drive Activity

- optional mirrored metadata in managed environments
- governance, sensitivity, and lifecycle hints
- operational and audit signal enrichment

### 15.10 Google Picker

- browser-mode file selection
- Drive-native attachment UX

## 16. Automation Recipes

Every cross-suite workflow must be representable as a versioned recipe in `automations/recipes/*.yaml`.

Recipes are canonical objects. They may run locally or through the relay.

Supported trigger types:

- page published
- page updated
- post promoted
- hashtag followed
- course assigned
- course due
- user onboarded
- community digest window
- calendar event created
- directory record changed
- manual run
- schedule

Supported action types:

- send Gmail
- create Gmail draft
- post Chat message
- create Calendar event
- update Calendar event
- export page to Doc
- export view to Sheet
- export lesson deck to Slides
- apply Drive label
- create limited-access folder
- sync Group membership
- ask Gemini to summarize
- ask Gemini to draft
- publish WEAVE post
- comment as agent
- update profile field

Recipe editor requirements:

- schema validation
- dry-run preview
- rendered payload preview
- required-scope preview
- audit log
- last-run status
- replay support
- secret indirection rather than literal secret entry

Example:

```yaml
name: promoted-post-broadcast
on:
  event: post.promoted
filters:
  communities: [home, leadership]
steps:
  - type: gemini.summarize
    input: post
    output: summary
  - type: gws.chat.spaces.messages.create
    params:
      parent: spaces/SPACE_ID
    body:
      text: "{{ summary.short }}"
  - type: gws.gmail.users.messages.send
    body_template: promoted-post-email
  - type: weave.notification.enqueue
    channel: in_app
```

## 17. Core Information Model

Top-level product entities:

- workspace
- community
- member
- page
- guide
- template
- library
- library item
- directory
- profile
- feed post
- feed comment
- hashtag
- course
- lesson
- learning path
- assignment
- transcript entry
- live session
- automation recipe
- agent
- notification
- export record
- analytics event
- connector mapping

Everything needs:

- stable internal IDs
- human-readable slugs where appropriate
- timestamps
- author or actor identity
- permission scope
- revision semantics
- audit trail linkage

## 18. Page And Block Model

Pages are structured block documents, never arbitrary HTML blobs.

Every block requires:

- JSON schema
- renderer
- editor config
- drag-and-drop behavior
- duplication behavior
- visibility rules
- export rules
- mobile behavior
- theme hooks
- validation

Required core blocks:

- rich text
- heading
- collapsible section
- divider
- table
- callout
- quote
- image
- gallery
- table of contents
- basic navigation
- smart navigation
- stream block
- post block
- hashtag block
- document block
- video block
- employee block
- project block
- metric block
- course block
- smart library block
- embed block

Required Google-native blocks:

- Drive file block
- Google Doc embed block
- Google Sheet range block
- Google Slide deck block
- Calendar agenda block
- Meet session block
- Gmail archive or search block
- Chat mirror block
- Group membership block
- Drive label badge block

Smart blocks resolve queries rather than storing static HTML snapshots.

## 19. Editorial Workflows And Templates

Required templates:

- Home
- Community Home
- Benefit Center
- Style Guide
- Learning Center
- Policy Page
- Directory Landing
- Guide Landing
- Tool Guide
- Blank Two-Column
- Blank Full Width

Editorial workflows:

- draft
- review
- publish
- promote
- export to Doc
- export to Slide deck
- archive
- clone to template
- translate via Gemini
- compare revisions
- reconcile Doc import

A guide is a page tree with a persistent right-side outline and strong in-page navigation. Guides must remain a first-class content form.

## 20. Navigation And Home Experience

Required home modules:

- live stream
- trending hashtags
- quick links
- popular resources
- birthdays
- anniversaries
- upcoming events
- learning assignments
- champion or help contact
- promoted posts
- featured documents
- featured videos
- recently updated guides
- agent cards

Required navigation features:

- mega menu with at least two levels
- breadcrumbs
- community switcher
- content outliner
- pinned resources
- recently visited
- best bets
- child-page smart grids
- right-rail related content
- open-in-Google affordances when appropriate

The UI language should remain light, airy, card-based, calm, and editorial.

## 21. Libraries, Directories, Profiles, And Connectors

### 21.1 Libraries

Libraries support documents, videos, and images with:

- Drive-backed asset storage
- custom metadata
- saved views
- sort, filter, group
- thumbnails and previews
- download and related-content linking
- version history where meaningful
- smart-block embedding
- batch import
- Sheet export and import
- linked Google file indexing

For linked Google-native files, use export endpoints to derive transient text or PDF extracts for indexing.

### 21.2 Directories

Directories support:

- employees
- projects
- companies
- contacts
- opportunities
- courses

Directory capabilities:

- list, grid, and map modes where relevant
- saved views
- hover cards
- filters and grouping
- configurable display fields
- editable custom fields
- relationship links
- mapped connector fields

### 21.3 Profiles

Employee profiles may hydrate from People or Admin sources and then extend locally. Project profiles must support image-rich cards, map view, linked people, linked pages, linked assets, and learning relationships.

### 21.4 Optional connectors

Support optional connectors and imports for:

- CSV employees, projects, courses, contacts, and companies
- filesystem watch import for assets
- generic REST pull connector
- webhook listener in relay mode
- DAM importers
- help center search connectors
- ERP and CRM mapping adapters

Connectors must emit normalized entities and events into canonical workspace structures. The rest of the product should not need to know where a record originated.

## 22. Feed, Hashtags, Notifications, And Digests

Required feed features:

- posts
- comments
- likes
- mentions
- hashtags
- hashtag pages
- promoted posts
- system posts
- community streams
- global stream
- moderation
- agent posting
- analytics
- pinning
- scheduling

Notification channels:

- in-app
- desktop OS
- Gmail
- Google Chat
- weekly digest
- optional Calendar reminders for learning items

Digest rules:

- a promoted post appears only if still promoted at send time
- a promoted post appears once per promotion window
- most-active ordering is comments first, then likes, then recency
- digest generation must be reproducible and auditable

Per-user notification preferences are required.

If Gmail delivery is disabled or unavailable, the system must still render the digest accurately in-app and allow operator-controlled manual sending.

## 23. Search And AI Search

Retrieval is local-first.

Search must index:

- pages
- guide sections
- feed posts
- comments
- document extracts
- video transcripts
- image metadata
- profile fields
- directory metadata
- course and lesson content
- library metadata
- linked Google Docs
- linked Google Sheets
- linked Google Slides
- tags and hashtags
- calendar-linked learning records
- external link metadata

Search ranking must blend:

- BM25
- semantic similarity
- entity boosting
- freshness
- best bets
- community affinity
- permission-aware filtering
- asset quality signals

The search UI must expose:

- fast result list
- filters and tabs
- answer pane with citations
- best-answer mode
- exhaustive mode labeled clearly as show-me-everything

Result tabs must include:

- All
- Pages
- Documents
- Videos
- People
- Projects
- Posts
- Hashtags
- Courses
- Libraries

Citations must deep-link to:

- page heading anchors
- specific post and comment IDs
- document page numbers
- video timestamps
- profile sections
- course lessons
- Google file IDs where relevant

Gemini File Search, or any hosted indexing path, must be explicit opt-in only and never replace the default local-first retrieval system.

## 24. Gemini Runtime

Gemini is the reasoning layer. It is not the storage layer and not the retrieval layer.

The Rust Gemini broker owns:

- model aliases
- tool declarations
- conversation history
- thought-signature preservation
- caching
- usage accounting
- safety and scope filters
- streaming responses
- retry behavior

Recommended aliases:

- `interactive_model`
- `deep_model`
- `high_throughput_model`
- `experimental_agent_model`
- `embedding_model`

Do not hard-code raw model IDs throughout the codebase.

Gemini usage requirements:

- use function calling for tool orchestration
- preserve thought signatures exactly when manual history management is in use
- support context caching for hot corpora such as the handbook, style guide, benefits guide, taxonomy rules, and learning catalog
- support long-context flows when the task materially benefits from them
- chunk documents and media locally before embedding
- separate retrieval from answer synthesis
- keep the answer layer citation-grounded

## 25. Agent System

Agents are first-class members of the workspace.

Each agent must have:

- name
- avatar
- bio
- communities
- permission scope
- schedule
- preferred model alias
- memory policy
- thread state
- allowed tools
- posting rules

Default agents:

- Archivist
- Librarian
- Connector
- Herald
- Coach
- Editor
- Scout
- Scheduler

Agent actions:

- draft page
- publish page
- comment
- tag
- promote
- summarize
- export Doc
- export Sheet
- create Calendar event
- create Gmail draft
- send digest
- mirror to Chat
- update profile metadata
- answer search query
- assign course
- schedule office hour

Agent behavior rules:

- every action leaves a visible trace
- agent-authored feed items require clear badging
- agent-created Gmail or Chat actions must link back to the source object
- permission scope must be enforced at tool level, not only at prompt level
- private memory must obey the same encryption and access rules as private human content

## 26. Learning, Video, And Live Learning

Required learning objects:

- course
- lesson
- course catalog
- learning center
- learning path
- assignment
- transcript entry
- dashboard
- continuing education record
- certificate
- external course wrapper
- cohort
- live session

Required video features:

- Drive-backed upload
- thumbnails
- captions
- transcripts
- chapters
- full-text search
- watch analytics
- branded player
- lesson linking
- transcript-to-quote extraction
- completion tracking

Google-native learning enhancements:

- live lessons create Calendar events
- synchronous sessions can include Meet links
- learners get Gmail reminders
- managers can receive digest summaries
- learning rosters can export to Sheets
- transcript and certificate summaries can appear on employee profiles
- Coach can generate nudges and office hours

## 27. Docs, Sheets, And Slides Workflows

### 27.1 Docs

- export page to Doc
- export guide section to Doc
- import Doc to draft
- collaborative burst mode
- diff Doc import against current draft
- maintain source link
- mark export stale when canonical page changes

### 27.2 Sheets

- export directory view
- export library view
- export assignment roster
- import edited Sheet with preview diff
- use Sheet as staging table for bulk updates
- push analytics summary tabs
- support taxonomy audit sheets

### 27.3 Slides

- export training deck
- export benefits deck
- export onboarding deck
- auto-build deck from learning path or guide
- keep deck links on course pages

Exports are collaboration amplifiers, not truth.

## 28. Branding, Accessibility, And Extension Surfaces

Required branding controls:

- logos
- favicon
- light and dark mode
- typography
- accent colors
- navigation styles
- heading styles
- card spacing and density
- thumbnail treatments
- icon style
- placeholder imagery
- community branding
- last-updated and last-updated-by display

Accessibility requirements:

- contrast checks
- WCAG AA thresholds
- keyboard navigation
- heading and landmark integrity
- caption review workflows
- transcript review workflows
- dark-mode audits
- reduced-motion option

Extension surfaces:

- custom CSS
- carefully scoped custom JavaScript
- page context object
- page-navigation event hooks
- controlled embed code execution in scoped blocks
- plugin API before raw code injection whenever possible

Theme exports should influence Docs, Slides, and digest email templates so brand feel survives when content leaves the core app.

## 29. Analytics, Audit, Governance, And Data Export

Track:

- page views
- search queries
- citation clicks
- digest sends and opens
- feed engagement
- hashtag follows
- Chat mirrors
- Calendar event creation
- course starts
- course completions
- video watch percentages
- assignment completion
- stale content candidates
- no-answer clusters

Default posture:

- raw interaction events stay local
- shared analytics are aggregated
- named analytics are opt-in
- audit trails may incorporate Drive Activity
- governance metadata may incorporate Drive Labels

Provide:

- read-only local analytics API
- JSON export
- tabular export suitable for BI

Use Drive Labels only as mirrored metadata where available, never as the only copy of governance state.

## 30. Browser Mode And Optional Relay

Relay responsibilities:

- browser UI
- Drive webhook receiver
- scheduled recipe execution
- Gmail delivery
- Chat mirroring
- Calendar writes
- import and export jobs
- heavy background transcription
- operations dashboard

Relay deployment options:

- local always-on machine
- small server
- Cloud Run

If the relay disappears, the Drive workspace and desktop app must continue to function.

## 31. Performance Targets

Hard targets:

- cold start under 2 seconds on a modern laptop
- first render under 150 ms after startup
- local search result list under 200 ms for 50k indexed items
- answer-pane first token under 3 seconds after retrieval begins
- feed scroll at 60 fps
- page open under 100 ms from warm cache
- incremental indexing within 2 seconds of file change
- Gmail, Chat, and Calendar recipe enqueue under 1 second
- demo workspace usable without relay
- mirrored-mode demo workspace usable offline except for Gemini and Google satellites

## 32. Monorepo Layout

Target repository layout:

```text
weave/
  AGENTS.md
  SPEC.md
  README.md
  Cargo.toml
  package.json
  apps/
    desktop/
    relay/
  crates/
    api/
    workspace/
    drive_sync/
    google_auth/
    gws_adapter/
    google_exports/
    google_people/
    google_mail/
    google_calendar/
    google_chat/
    pages/
    feed/
    libraries/
    profiles/
    lms/
    search/
    analytics/
    automations/
    gemini_broker/
    crypto/
    connectors/
    ui_contracts/
  frontend/
    src/
    components/
    routes/
    editor/
    theme/
  skills/
    workspace-engine/
    pages/
    feed/
    search/
    google-suite/
    lms/
    release-demo/
  fixtures/
    demo-workspace/
  scripts/
    dev/
    release/
    import/
    ci/
```

## 33. Required Repository Scaffolding

The repository must include:

- root `AGENTS.md`
- nested `AGENTS.md` in critical subtrees such as `crates/drive_sync`, `crates/gws_adapter`, `crates/gemini_broker`, `crates/automations`, `frontend/editor`, and `fixtures/demo-workspace`
- `SPEC.md`
- `README.md`
- golden screenshot fixtures
- demo workspace fixtures
- task-oriented scripts
- CI configuration
- security scanning configuration
- release and packaging scripts

Required scripts and entrypoints:

- bootstrap or doctor script for local environment verification
- dev script for desktop and API startup
- fixture loader for the demo workspace
- rebuild script for caches and indexes
- migration preview and apply scripts
- screenshot capture script
- release packaging script
- CI wrapper script or task runner entrypoint

## 34. Testing Strategy

Test at these layers:

### 34.1 Storage and sync

- canonical file behavior
- change-log replay
- shared-drive replay correctness
- conflict copies
- Lost and Found repair
- mirrored versus streamed behavior
- macOS File Provider path behavior
- permission propagation

### 34.2 Product behavior

- page publishing
- feed posting
- notifications
- digest logic
- Docs, Sheets, and Slides exports
- import reconciliation
- Calendar creation
- Chat mirroring
- agent trace creation

### 34.3 Search and AI

- lexical ranking
- semantic ranking
- citation correctness
- exhaustive mode
- permission filtering
- answer grounding
- tool-call audit logs
- thought-signature preservation

### 34.4 Visual parity

- golden screenshots
- template rendering
- right-rail composition
- directory cards
- library grids
- feed density
- dark mode
- mobile breakpoints

### 34.5 Privacy and security

- encryption and membership changes
- key rotation
- unauthorized index prevention
- secret redaction
- relay auth boundaries

Use recorded fixtures whenever possible. For operator workflows, use `gws --dry-run` in smoke tests before live integration tests.

## 35. Code Quality Gates

The implementing model must set up and use the following checks as early as possible:

### 35.1 Rust

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features`

### 35.2 Frontend

- `pnpm lint`
- `pnpm typecheck`
- `pnpm test`
- `pnpm test:e2e`

### 35.3 Cross-surface

- golden screenshot comparisons
- fixture migration checks
- demo workspace smoke test
- docs link and markdown lint

The builder must create repository-level wrapper commands or scripts so these checks are easy to run repeatedly.

CI expectations:

- run the core matrix on macOS and Windows
- run Rust checks on every pull request or merge candidate
- run frontend lint, typecheck, and tests on every pull request or merge candidate
- run golden or screenshot smoke tests on UI-affecting changes
- run security scans on dependency, auth, relay, and integration changes

## 36. Security Gates

Security checks must be installed and run regularly, not only at release time.

Required gates:

- `cargo deny check`
- `cargo audit`
- `pnpm audit --audit-level high`
- `gitleaks detect --source . --no-git`
- dependency update review before lockfile bumps
- secret handling review for any auth, export, relay, or Gemini-related code

Security implementation rules:

- never log tokens, refresh tokens, access tokens, or raw secrets
- redact PII in debug and support output by default
- require explicit opt-in for named analytics
- scope relay credentials to the minimum permissions necessary
- preserve permission filtering all the way through search and answer synthesis

## 37. Micro-Commit Protocol

The implementation process must use frequent micro commits.

Commit after:

- each completed vertical slice
- each successfully integrated subsystem
- each meaningful refactor that preserves behavior
- each test or security hardening pass that changes repository state
- at least every 30 to 90 minutes of active coding

Before each micro commit:

1. Run the narrowest relevant checks for the changed surface.
2. Run the security gate relevant to changed auth, integration, or dependency code.
3. Confirm the repo still builds or that unfinished work is safely hidden behind flags.
4. Confirm docs remain accurate for the changed behavior.
5. Update the changelog or milestone notes if the change is visible.

Commit message rules:

- messages must be descriptive first and mythopoetic second
- messages must include the technical scope
- messages must draw on an Italo Calvino tone or image without becoming vague
- messages must stay comprehensible in `git log --oneline`

Recommended format:

`<scope>: <clear technical action> beneath a Calvino-inflected image`

Acceptable examples:

- `bootstrap: wire Drive root setup under a winter traveler sky`
- `sync: replay shared-drive deltas through invisible cities`
- `search: ground answer-pane citations in a cosmicomic index`
- `agents: trace Scheduler actions in the castle of crossed tools`
- `lms: thread live-session reminders through the baron in the trees`

Do not use whimsical messages that obscure the actual technical change.

## 38. Autonomous Delivery Loop

The implementing model must follow this loop continuously:

1. Read the relevant sections of `SPEC.md` and `AGENTS.md`.
2. Pick the next milestone task that unblocks the most downstream work.
3. Implement a vertical slice that leaves the app more real, not merely more scaffolded.
4. Run targeted code-quality and security checks.
5. Fix failures immediately.
6. Update docs, scripts, and fixtures as needed.
7. Capture screenshots or evidence for visible changes.
8. Create a micro commit.
9. Move directly to the next slice.

Each milestone checkpoint should leave behind:

- committed code
- passing relevant checks
- updated docs
- any needed fixture changes
- at least one concrete proof artifact for visible work, such as a screenshot, golden, or recorded dry-run output

The model should not stop to ask for confirmation unless:

- a destructive operation is required
- credentials or external permissions are unavailable
- two incompatible product directions both satisfy the spec and the choice is irreversible

## 39. Build Order

### Milestone 0: Bootstrap

- desktop shell
- Drive root picker
- `gws` version check and auth wizard
- Gemini setup
- theme tokens
- demo workspace loader

### Milestone 1: Workspace engine

- canonical file engine
- revision store
- draft and publish flow
- local watchers
- local cache rebuilds
- sync-health panel
- user and shared-drive change replay

### Milestone 2: Pages and navigation

- block renderer
- page editor
- templates
- guides
- content outliner
- smart navigation
- Google-native blocks

### Milestone 3: Libraries and profiles

- document, video, and image libraries
- employees and projects
- profiles
- saved views
- hover cards
- map mode
- Sheet export and import

### Milestone 4: Feed and notifications

- posts, comments, likes, hashtags
- promoted posts
- moderation
- in-app notifications
- Gmail digests
- Chat mirrors

### Milestone 5: Search and AI

- BM25
- vectors
- citations
- answer pane
- exhaustive mode
- linked Google-file indexing
- Gemini tool layer

### Milestone 6: Learning and video

- native video
- transcripts
- chapters
- analytics
- courses
- learning paths
- assignments
- transcript records
- Calendar and Meet integration

### Milestone 7: Google suite coordination

- Docs export and import
- Sheets operational flows
- Slides export
- Groups sync
- profile hydration
- admin recipes
- operator command shadows

### Milestone 8: Privacy, connectors, relay, and governance

- encrypted scopes
- limited-access helpers
- connector runtime
- relay mode
- webhook receiver
- Drive-label mirroring
- audit surfaces

## 40. Demo Workspace

The demo workspace is a product requirement, not decoration.

It must include:

- a home page with live stream, trending hashtags, quick links, birthdays, anniversaries, and upcoming events
- a benefits center
- a style guide
- a learning center
- an employee directory
- a project directory with map mode
- multiple promoted posts
- multiple videos with transcripts and chapters
- multiple courses and transcript entries
- a Gmail digest recipe
- a Chat mirror recipe
- a Calendar-backed live session
- a Doc export example
- a Sheet bulk-edit example
- a Slide deck export example
- at least two active agents

The demo workspace is the acceptance test for feel, not just functionality.

## 41. Definition Of Done

WEAVE is done when a new user can:

1. install the desktop app
2. authenticate `gws`
3. configure Gemini
4. point the app at a My Drive folder or shared drive root
5. have the storage profile selected correctly
6. open a polished demo workspace
7. create and publish a page
8. export that page to a Google Doc
9. upload and index documents and videos
10. browse employee and project directories
11. bulk-edit a directory view through a Sheet workflow
12. post to the feed with hashtags
13. receive a Gmail digest
14. mirror a promoted post to Chat
15. create a live learning session in Calendar with a Meet link
16. ask a natural-language question and get a cited answer
17. let an agent post or schedule a useful update
18. keep a private community private
19. move the workspace to a new Drive root and reconnect without losing truth
20. rebuild local cache and search from canonical files alone

## 42. Final Instruction To The Implementing Model

Read `SPEC.md` and `AGENTS.md`, then build WEAVE all the way through. Do not stop at architectural notes, partial scaffolds, or milestone summaries. Ship the repo in successive working slices, keep the demo alive, keep the file model stable, keep the Google surfaces secondary to canonical files, and leave behind a repository that another strong engineer or model can continue without reinterpreting the project.
