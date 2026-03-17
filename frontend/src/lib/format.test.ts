import { describe, expect, it } from 'vitest'

import { deriveWorkspaceStats, hashtags } from './format'

describe('format helpers', () => {
  it('builds concise workspace stats', () => {
    const stats = deriveWorkspaceStats({
      workspace: {
        id: 'weave',
        name: 'WEAVE',
        tagline: 'quiet intranet',
        pages: 3,
        posts: 4,
        documents: 2,
        videos: 1,
        courses: 2,
      },
      communities: [],
      featured_pages: [],
      promoted_posts: [],
      featured_people: [],
      featured_projects: [],
      featured_documents: [],
      featured_videos: [],
      featured_courses: [],
      agents: [],
      live_sessions: [],
      exports: [],
      notifications: [],
      automations: [],
      google_previews: [],
      sync_health: {
        workspace_root: '/tmp',
        drive_mode: 'mirrored',
        ownership_mode: 'my_drive',
        last_local_scan: '2026-03-16T10:00:00Z',
        last_replay: '2026-03-16T10:00:00Z',
        unresolved_conflicts: 0,
        stale_cache_count: 0,
        lost_and_found_items: 0,
        workspace_audit_issue_count: 0,
        export_queue_backlog: 0,
        decryption_state: 'not_enabled',
        relay_connectivity: 'desktop_only',
        last_cache_rebuild: null,
      },
    })

    expect(stats[0]).toEqual({ label: 'Pages', value: 3 })
    expect(stats[3]).toEqual({ label: 'Courses', value: 2 })
  })

  it('formats hashtags as visible pills', () => {
    expect(hashtags(['launch', 'learning'])).toBe('#launch #learning')
  })
})
