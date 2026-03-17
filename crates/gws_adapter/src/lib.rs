use ui_contracts::{AutomationRecipe, GoogleActionPreview, RecipePreview, WorkspaceSnapshot};

#[must_use]
pub fn preview_recipe(recipe: &AutomationRecipe) -> RecipePreview {
    let mut scopes = Vec::new();
    let mut commands = Vec::new();

    for step in &recipe.steps {
        match step.step_type.as_str() {
            "gws.chat.spaces.messages.create" => {
                scopes.push("chat.messages.create".to_string());
                commands.push(format!(
                    "gws chat spaces messages create --parent {} --dry-run",
                    step.parent.as_deref().unwrap_or("spaces/UNKNOWN")
                ));
            }
            "gws.gmail.users.messages.send" => {
                scopes.push("gmail.send".to_string());
                commands.push("gws gmail users messages send --dry-run".to_string());
            }
            "gemini.summarize" => {
                scopes.push("gemini.generate".to_string());
                commands.push("weave gemini summarize --dry-run".to_string());
            }
            _ => commands.push(format!("weave action {} --dry-run", step.step_type)),
        }
    }

    RecipePreview {
        id: recipe.id.clone(),
        name: recipe.name.clone(),
        command_preview: commands.join(" && "),
        payload_preview: format!(
            "{} step(s) will run against {}",
            recipe.steps.len(),
            recipe.trigger
        ),
        required_scopes: scopes,
    }
}

#[must_use]
pub fn preview_workspace_actions(snapshot: &WorkspaceSnapshot) -> Vec<GoogleActionPreview> {
    let mut previews = Vec::new();

    for page in snapshot
        .pages
        .iter()
        .filter(|page| page.meta.featured)
        .take(2)
    {
        previews.push(GoogleActionPreview {
            id: format!("docs-export-{}", page.meta.id),
            title: format!("Export {} to Google Docs", page.meta.title),
            surface: "docs".to_string(),
            summary: format!(
                "Create a collaborative Google Doc satellite for the canonical page {}.",
                page.meta.title
            ),
            object_id: page.meta.id.clone(),
            command_preview: format!(
                "gws docs documents create --title '{}' --dry-run",
                page.meta.title
            ),
            required_scopes: vec![
                "docs.documents.create".to_string(),
                "drive.file".to_string(),
            ],
        });
    }

    for post in snapshot.feed.iter().filter(|post| post.promoted).take(1) {
        previews.push(GoogleActionPreview {
            id: format!("chat-mirror-{}", post.id),
            title: format!("Mirror promoted post '{}' to Google Chat", post.title),
            surface: "chat".to_string(),
            summary: "Post the promoted feed update into the announcements space.".to_string(),
            object_id: post.id.clone(),
            command_preview:
                "gws chat spaces messages create --parent spaces/STUDIO_ANNOUNCEMENTS --dry-run"
                    .to_string(),
            required_scopes: vec!["chat.messages.create".to_string()],
        });
    }

    for course in snapshot
        .courses
        .iter()
        .filter(|course| course.featured)
        .take(1)
    {
        previews.push(GoogleActionPreview {
            id: format!("calendar-session-{}", course.id),
            title: format!("Create live session for {}", course.title),
            surface: "calendar".to_string(),
            summary: "Create a Calendar event with Meet attached for the live learning session."
                .to_string(),
            object_id: course.id.clone(),
            command_preview:
                "gws calendar events insert --calendar primary --conference meet --dry-run"
                    .to_string(),
            required_scopes: vec!["calendar.events".to_string()],
        });
    }

    if let Some(recipe) = snapshot
        .automations
        .iter()
        .find(|recipe| recipe.id == "weekly-digest")
    {
        previews.push(GoogleActionPreview {
            id: format!("gmail-digest-{}", recipe.id),
            title: "Send the weekly Gmail digest".to_string(),
            surface: "gmail".to_string(),
            summary: "Render the promoted digest payload and preview the outbound Gmail send."
                .to_string(),
            object_id: recipe.id.clone(),
            command_preview: "gws gmail users messages send --dry-run".to_string(),
            required_scopes: vec!["gmail.send".to_string()],
        });
    }

    previews
}

#[cfg(test)]
mod tests {
    use super::*;
    use ui_contracts::{
        AgentRecord, AutomationRecipe, AutomationStep, CommunityRecord, CourseRecord, FeedPost,
        LessonRecord, LibraryItem, PageMeta, PageRecord, PageRevision, SyncHealth, ThemeDefinition,
        WorkspaceSettings, WorkspaceSnapshot,
    };

    #[test]
    fn builds_google_action_previews() {
        let snapshot = WorkspaceSnapshot {
            settings: WorkspaceSettings {
                schema_version: 1,
                object_type: "workspace_settings".to_string(),
                id: "weave".to_string(),
                name: "WEAVE".to_string(),
                tagline: "quiet".to_string(),
                storage_mode: "mirrored".to_string(),
                ownership_mode: "my_drive".to_string(),
                default_community_id: "home".to_string(),
            },
            theme: ThemeDefinition {
                schema_version: 1,
                object_type: "theme".to_string(),
                id: "theme".to_string(),
                name: "theme".to_string(),
                accent: "#000".to_string(),
                accent_soft: "#111".to_string(),
                canvas: "#222".to_string(),
                surface: "#333".to_string(),
                heading_font: "Fraunces".to_string(),
                body_font: "IBM Plex Sans".to_string(),
            },
            communities: vec![CommunityRecord {
                schema_version: 1,
                object_type: "community".to_string(),
                id: "home".to_string(),
                slug: "home".to_string(),
                title: "Home".to_string(),
                description: "desc".to_string(),
                accent: "#000".to_string(),
                home_page_id: "page-home".to_string(),
            }],
            pages: vec![PageRecord {
                meta: PageMeta {
                    schema_version: 1,
                    object_type: "page_meta".to_string(),
                    id: "page-home".to_string(),
                    slug: "home".to_string(),
                    title: "Home".to_string(),
                    excerpt: "excerpt".to_string(),
                    community_id: "home".to_string(),
                    template: "Home".to_string(),
                    updated_at: chrono::Utc::now(),
                    featured: true,
                },
                published_revision: PageRevision {
                    schema_version: 1,
                    object_type: "page_revision".to_string(),
                    id: "rev-1".to_string(),
                    page_id: "page-home".to_string(),
                    title: "Home".to_string(),
                    summary: "summary".to_string(),
                    author: "agent".to_string(),
                    updated_at: chrono::Utc::now(),
                    blocks: Vec::new(),
                },
                revisions: Vec::new(),
                drafts: Vec::new(),
            }],
            people: Vec::new(),
            projects: Vec::new(),
            documents: vec![LibraryItem {
                schema_version: 1,
                object_type: "library_item".to_string(),
                id: "doc".to_string(),
                slug: "doc".to_string(),
                kind: "document".to_string(),
                title: "Doc".to_string(),
                description: "desc".to_string(),
                community_id: "home".to_string(),
                updated_at: chrono::Utc::now(),
                tags: Vec::new(),
                citation_hint: None,
            }],
            videos: Vec::new(),
            courses: vec![CourseRecord {
                schema_version: 1,
                object_type: "course".to_string(),
                id: "course-1".to_string(),
                slug: "course".to_string(),
                title: "Course".to_string(),
                summary: "summary".to_string(),
                community_id: "home".to_string(),
                duration_minutes: 30,
                assignment_due: None,
                status: "active".to_string(),
                lessons: vec![LessonRecord {
                    id: "lesson-1".to_string(),
                    title: "Lesson".to_string(),
                    minutes: 10,
                }],
                featured: true,
            }],
            feed: vec![FeedPost {
                schema_version: 1,
                object_type: "feed_post".to_string(),
                id: "post-1".to_string(),
                community_id: "home".to_string(),
                author_id: "agent".to_string(),
                author_name: "Herald".to_string(),
                title: "Promoted".to_string(),
                body: "body".to_string(),
                hashtags: Vec::new(),
                likes: 0,
                comments: 0,
                promoted: true,
                published_at: chrono::Utc::now(),
            }],
            agents: vec![AgentRecord {
                schema_version: 1,
                object_type: "agent".to_string(),
                id: "agent".to_string(),
                name: "Agent".to_string(),
                bio: "bio".to_string(),
                communities: vec!["home".to_string()],
                schedule: "daily".to_string(),
                preferred_model: "interactive_model".to_string(),
                allowed_tools: Vec::new(),
                posting_rules: Vec::new(),
            }],
            automations: vec![AutomationRecipe {
                schema_version: 1,
                id: "weekly-digest".to_string(),
                name: "Weekly Digest".to_string(),
                description: "desc".to_string(),
                trigger: "schedule.weekly".to_string(),
                steps: vec![AutomationStep {
                    step_type: "gws.gmail.users.messages.send".to_string(),
                    channel: Some("gmail".to_string()),
                    body_template: Some("weekly-digest".to_string()),
                    parent: None,
                }],
            }],
            google_previews: Vec::new(),
            sync_health: SyncHealth {
                workspace_root: "/tmp".to_string(),
                drive_mode: "mirrored".to_string(),
                ownership_mode: "my_drive".to_string(),
                last_local_scan: chrono::Utc::now(),
                last_replay: chrono::Utc::now(),
                unresolved_conflicts: 0,
                stale_cache_count: 0,
            },
        };

        let previews = preview_workspace_actions(&snapshot);
        assert!(previews.iter().any(|preview| preview.surface == "docs"));
        assert!(previews.iter().any(|preview| preview.surface == "chat"));
        assert!(previews.iter().any(|preview| preview.surface == "calendar"));
        assert!(previews.iter().any(|preview| preview.surface == "gmail"));
    }
}
