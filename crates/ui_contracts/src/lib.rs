use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type Timestamp = DateTime<Utc>;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct WorkspaceSettings {
    pub schema_version: u16,
    pub object_type: String,
    pub id: String,
    pub name: String,
    pub tagline: String,
    pub storage_mode: String,
    pub ownership_mode: String,
    pub default_community_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ThemeDefinition {
    pub schema_version: u16,
    pub object_type: String,
    pub id: String,
    pub name: String,
    pub accent: String,
    pub accent_soft: String,
    pub canvas: String,
    pub surface: String,
    pub heading_font: String,
    pub body_font: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct CommunityRecord {
    pub schema_version: u16,
    pub object_type: String,
    pub id: String,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub accent: String,
    pub home_page_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct PageMeta {
    pub schema_version: u16,
    pub object_type: String,
    pub id: String,
    pub slug: String,
    pub title: String,
    pub excerpt: String,
    pub community_id: String,
    pub template: String,
    pub updated_at: Timestamp,
    pub featured: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct PageRevision {
    pub schema_version: u16,
    pub object_type: String,
    pub id: String,
    pub page_id: String,
    pub title: String,
    pub summary: String,
    pub author: String,
    pub updated_at: Timestamp,
    pub blocks: Vec<PageBlock>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct PageBlock {
    pub kind: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub items: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct DirectoryEntity {
    pub schema_version: u16,
    pub object_type: String,
    pub id: String,
    pub slug: String,
    pub kind: String,
    pub name: String,
    pub title: String,
    pub location: String,
    pub summary: String,
    pub communities: Vec<String>,
    pub tags: Vec<String>,
    pub featured: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct LibraryItem {
    pub schema_version: u16,
    pub object_type: String,
    pub id: String,
    pub slug: String,
    pub kind: String,
    pub title: String,
    pub description: String,
    pub community_id: String,
    pub updated_at: Timestamp,
    pub tags: Vec<String>,
    #[serde(default)]
    pub citation_hint: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct CourseRecord {
    pub schema_version: u16,
    pub object_type: String,
    pub id: String,
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub community_id: String,
    pub duration_minutes: u32,
    #[serde(default)]
    pub assignment_due: Option<Timestamp>,
    pub status: String,
    pub lessons: Vec<LessonRecord>,
    pub featured: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct LessonRecord {
    pub id: String,
    pub title: String,
    pub minutes: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct FeedPost {
    pub schema_version: u16,
    pub object_type: String,
    pub id: String,
    pub community_id: String,
    pub author_id: String,
    pub author_name: String,
    pub title: String,
    pub body: String,
    pub hashtags: Vec<String>,
    pub likes: u32,
    pub comments: u32,
    pub promoted: bool,
    pub published_at: Timestamp,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct AutomationRecipe {
    pub schema_version: u16,
    pub id: String,
    pub name: String,
    pub description: String,
    pub trigger: String,
    pub steps: Vec<AutomationStep>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct AutomationStep {
    #[serde(rename = "type")]
    pub step_type: String,
    #[serde(default)]
    pub channel: Option<String>,
    #[serde(default)]
    pub body_template: Option<String>,
    #[serde(default)]
    pub parent: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct SyncHealth {
    pub workspace_root: String,
    pub drive_mode: String,
    pub ownership_mode: String,
    pub last_local_scan: Timestamp,
    pub last_replay: Timestamp,
    pub unresolved_conflicts: usize,
    pub stale_cache_count: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct SearchResult {
    pub id: String,
    pub kind: String,
    pub title: String,
    pub snippet: String,
    pub path: String,
    pub citation: String,
    pub score: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct SearchAnswer {
    pub mode: String,
    pub summary: String,
    pub citations: Vec<SearchCitation>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct SearchCitation {
    pub label: String,
    pub target: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct SearchResponse {
    pub query: String,
    pub results: Vec<SearchResult>,
    pub answer: SearchAnswer,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct RecipePreview {
    pub id: String,
    pub name: String,
    pub command_preview: String,
    pub payload_preview: String,
    pub required_scopes: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct WorkspaceSnapshot {
    pub settings: WorkspaceSettings,
    pub theme: ThemeDefinition,
    pub communities: Vec<CommunityRecord>,
    pub pages: Vec<PageRecord>,
    pub people: Vec<DirectoryEntity>,
    pub projects: Vec<DirectoryEntity>,
    pub documents: Vec<LibraryItem>,
    pub videos: Vec<LibraryItem>,
    pub courses: Vec<CourseRecord>,
    pub feed: Vec<FeedPost>,
    pub automations: Vec<AutomationRecipe>,
    pub sync_health: SyncHealth,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct PageRecord {
    pub meta: PageMeta,
    pub published_revision: PageRevision,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct DashboardSnapshot {
    pub workspace: WorkspaceSummary,
    pub communities: Vec<CommunityRecord>,
    pub featured_pages: Vec<PageMeta>,
    pub promoted_posts: Vec<FeedPost>,
    pub featured_people: Vec<DirectoryEntity>,
    pub featured_projects: Vec<DirectoryEntity>,
    pub featured_documents: Vec<LibraryItem>,
    pub featured_videos: Vec<LibraryItem>,
    pub featured_courses: Vec<CourseRecord>,
    pub automations: Vec<AutomationRecipe>,
    pub sync_health: SyncHealth,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct WorkspaceSummary {
    pub id: String,
    pub name: String,
    pub tagline: String,
    pub pages: usize,
    pub posts: usize,
    pub documents: usize,
    pub videos: usize,
    pub courses: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct PublishPageRequest {
    pub author: String,
    pub title: String,
    pub summary: String,
    pub blocks: Vec<PageBlock>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct CreateFeedPostRequest {
    pub author_id: String,
    pub author_name: String,
    pub community_id: String,
    pub title: String,
    pub body: String,
    pub hashtags: Vec<String>,
    pub promoted: bool,
}
