export type Community = {
  id: string
  slug: string
  title: string
  description: string
  accent: string
  home_page_id: string
}

export type PageMeta = {
  id: string
  slug: string
  title: string
  excerpt: string
  community_id: string
  template: string
  updated_at: string
  featured: boolean
}

export type PageBlock = {
  kind: string
  title?: string
  body?: string
  items?: string[]
}

export type PageRecord = {
  meta: PageMeta
  published_revision: {
    title: string
    summary: string
    author: string
    updated_at: string
    blocks: PageBlock[]
  }
  revisions: Array<{
    id: string
    title: string
    summary: string
    author: string
    updated_at: string
    blocks: PageBlock[]
  }>
  drafts: PageDraft[]
}

export type PageDraft = {
  page_id: string
  author: string
  title: string
  summary: string
  updated_at: string
  blocks: PageBlock[]
}

export type FeedPost = {
  id: string
  community_id: string
  author_id: string
  author_name: string
  title: string
  body: string
  hashtags: string[]
  likes: number
  comments: number
  promoted: boolean
  published_at: string
}

export type DirectoryEntity = {
  id: string
  kind: string
  name: string
  title: string
  location: string
  summary: string
  tags: string[]
  featured: boolean
}

export type LibraryItem = {
  id: string
  kind: string
  title: string
  description: string
  tags: string[]
  updated_at: string
  citation_hint?: string
}

export type Course = {
  id: string
  slug: string
  title: string
  summary: string
  duration_minutes: number
  assignment_due?: string | null
  status: string
  featured: boolean
  lessons: Array<{ id: string; title: string; minutes: number }>
}

export type AutomationRecipe = {
  id: string
  name: string
  description: string
  trigger: string
}

export type AgentRecord = {
  id: string
  name: string
  bio: string
  communities: string[]
  schedule: string
  preferred_model: string
  allowed_tools: string[]
  posting_rules: string[]
}

export type LiveSessionRecord = {
  id: string
  title: string
  course_id: string
  community_id: string
  starts_at: string
  duration_minutes: number
  meet_enabled: boolean
  calendar_id: string
  status: string
}

export type ExportRecord = {
  id: string
  kind: string
  source_object_id: string
  title: string
  status: string
  destination_hint: string
  last_run_at: string
}

export type NotificationRecord = {
  id: string
  channel: string
  title: string
  body: string
  created_at: string
  state: string
}

export type RecipePreview = {
  id: string
  name: string
  command_preview: string
  payload_preview: string
  required_scopes: string[]
}

export type GoogleActionPreview = {
  id: string
  title: string
  surface: string
  summary: string
  object_id: string
  command_preview: string
  required_scopes: string[]
}

export type SyncHealth = {
  workspace_root: string
  drive_mode: string
  ownership_mode: string
  unresolved_conflicts: number
  stale_cache_count: number
}

export type DashboardSnapshot = {
  workspace: {
    id: string
    name: string
    tagline: string
    pages: number
    posts: number
    documents: number
    videos: number
    courses: number
  }
  communities: Community[]
  featured_pages: PageMeta[]
  promoted_posts: FeedPost[]
  featured_people: DirectoryEntity[]
  featured_projects: DirectoryEntity[]
  featured_documents: LibraryItem[]
  featured_videos: LibraryItem[]
  featured_courses: Course[]
  agents: AgentRecord[]
  live_sessions: LiveSessionRecord[]
  exports: ExportRecord[]
  notifications: NotificationRecord[]
  automations: AutomationRecipe[]
  google_previews: GoogleActionPreview[]
  sync_health: SyncHealth
}

export type SearchResponse = {
  query: string
  results: Array<{
    id: string
    kind: string
    title: string
    snippet: string
    path: string
    citation: string
    score: number
  }>
  answer: {
    mode: string
    summary: string
    citations: Array<{ label: string; target: string }>
  }
}
