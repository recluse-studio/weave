import type { DashboardSnapshot } from './types'

export function formatDate(value?: string | null): string {
  if (!value) {
    return 'No due date'
  }

  return new Intl.DateTimeFormat('en-US', {
    month: 'short',
    day: 'numeric',
    year: 'numeric',
  }).format(new Date(value))
}

export function deriveWorkspaceStats(dashboard: DashboardSnapshot) {
  return [
    { label: 'Pages', value: dashboard.workspace.pages },
    { label: 'Posts', value: dashboard.workspace.posts },
    { label: 'Docs', value: dashboard.workspace.documents },
    { label: 'Courses', value: dashboard.workspace.courses },
  ]
}

export function hashtags(tags: string[]): string {
  return tags.map((tag) => `#${tag}`).join(' ')
}
