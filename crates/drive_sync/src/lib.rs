use std::{
    collections::{HashMap, HashSet},
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rusqlite::{Connection, OptionalExtension, params};
use ui_contracts::{
    CacheObjectCount, CacheRebuildReport, SyncHealth, SyncIssue, WorkspaceAuditReport,
    WorkspaceAuditSummary, WorkspaceSettings,
};
use walkdir::WalkDir;
use workspace::WorkspaceRepository;

pub fn sync_health(repository: &WorkspaceRepository) -> Result<SyncHealth> {
    let snapshot = repository.load_snapshot()?;
    let audit = audit_workspace(repository)?;
    let expected_counts = expected_object_counts(&snapshot);
    let last_cache_rebuild = read_last_cache_rebuild(&snapshot.settings.id)?;
    let cached_counts = load_cached_counts(&snapshot.settings.id)?;
    let stale_cache_count = if cached_counts.is_empty() {
        expected_counts.len()
    } else {
        expected_counts
            .iter()
            .filter(|count| cached_counts.get(&count.kind) != Some(&count.count))
            .count()
    };
    let now = Utc::now();

    Ok(SyncHealth {
        workspace_root: repository.root().display().to_string(),
        drive_mode: snapshot.settings.storage_mode,
        ownership_mode: snapshot.settings.ownership_mode,
        last_local_scan: now,
        last_replay: last_cache_rebuild.unwrap_or(now),
        unresolved_conflicts: audit.summary.conflict_copies,
        stale_cache_count,
        lost_and_found_items: audit.summary.lost_and_found_items,
        workspace_audit_issue_count: issue_count(&audit),
        export_queue_backlog: snapshot
            .exports
            .iter()
            .filter(|record| record.status != "ready")
            .count(),
        decryption_state: "not_enabled".to_string(),
        relay_connectivity: "desktop_only".to_string(),
        last_cache_rebuild,
    })
}

pub fn audit_workspace(repository: &WorkspaceRepository) -> Result<WorkspaceAuditReport> {
    let scanned_at = Utc::now();
    let fallback_settings = read_workspace_settings(repository.root());
    let workspace_id = fallback_settings
        .as_ref()
        .map(|settings| settings.id.clone())
        .unwrap_or_else(|| {
            repository.root().file_name().map_or_else(
                || "workspace".to_string(),
                |name| name.to_string_lossy().into(),
            )
        });
    let workspace_root = repository.root().display().to_string();
    let mut invalid_paths = required_path_issues(repository.root());
    let mut missing_references = Vec::new();
    let mut orphaned_drafts = Vec::new();
    let mut conflict_copies = Vec::new();
    let mut lost_and_found_items = Vec::new();

    match repository.load_snapshot() {
        Ok(snapshot) => {
            let community_ids = snapshot
                .communities
                .iter()
                .map(|community| community.id.as_str())
                .collect::<HashSet<_>>();
            let page_ids = snapshot
                .pages
                .iter()
                .map(|page| page.meta.id.as_str())
                .collect::<HashSet<_>>();
            let course_ids = snapshot
                .courses
                .iter()
                .map(|course| course.id.as_str())
                .collect::<HashSet<_>>();
            let known_object_ids = snapshot
                .pages
                .iter()
                .map(|page| page.meta.id.clone())
                .chain(snapshot.people.iter().map(|entity| entity.id.clone()))
                .chain(snapshot.projects.iter().map(|entity| entity.id.clone()))
                .chain(
                    ["directories-employees", "directories-projects"]
                        .into_iter()
                        .map(str::to_string),
                )
                .chain(snapshot.documents.iter().map(|item| item.id.clone()))
                .chain(snapshot.videos.iter().map(|item| item.id.clone()))
                .chain(
                    ["libraries-documents", "libraries-videos"]
                        .into_iter()
                        .map(str::to_string),
                )
                .chain(snapshot.courses.iter().map(|course| course.id.clone()))
                .chain(snapshot.agents.iter().map(|agent| agent.id.clone()))
                .chain(snapshot.automations.iter().map(|recipe| recipe.id.clone()))
                .collect::<HashSet<_>>();

            if !community_ids.contains(snapshot.settings.default_community_id.as_str()) {
                missing_references.push(issue(
                    "missing_default_community",
                    "error",
                    format!(
                        "Workspace default community '{}' is not defined.",
                        snapshot.settings.default_community_id
                    ),
                    repository.root().join("workspace/settings.json"),
                    Some(snapshot.settings.default_community_id.clone()),
                ));
            }

            for community in &snapshot.communities {
                if !page_ids.contains(community.home_page_id.as_str()) {
                    missing_references.push(issue(
                        "missing_home_page",
                        "error",
                        format!(
                            "Community '{}' points to missing home page '{}'.",
                            community.id, community.home_page_id
                        ),
                        repository
                            .root()
                            .join(format!("communities/{}.json", community.slug)),
                        Some(community.home_page_id.clone()),
                    ));
                }
            }

            for page in &snapshot.pages {
                if !community_ids.contains(page.meta.community_id.as_str()) {
                    missing_references.push(issue(
                        "missing_page_community",
                        "error",
                        format!(
                            "Page '{}' references missing community '{}'.",
                            page.meta.id, page.meta.community_id
                        ),
                        repository
                            .root()
                            .join(format!("pages/{}/meta.json", page.meta.id)),
                        Some(page.meta.community_id.clone()),
                    ));
                }

                for draft in &page.drafts {
                    if draft.page_id != page.meta.id {
                        orphaned_drafts.push(issue(
                            "orphaned_draft_page_id",
                            "error",
                            format!(
                                "Draft '{}' is stored under page '{}' but declares '{}'.",
                                draft.author, page.meta.id, draft.page_id
                            ),
                            repository.root().join(format!(
                                "pages/{}/drafts/{}.json",
                                page.meta.id, draft.author
                            )),
                            Some(draft.page_id.clone()),
                        ));
                    }
                }
            }

            for item in snapshot.documents.iter().chain(&snapshot.videos) {
                if !community_ids.contains(item.community_id.as_str()) {
                    missing_references.push(issue(
                        "missing_library_community",
                        "error",
                        format!(
                            "Library item '{}' references missing community '{}'.",
                            item.id, item.community_id
                        ),
                        find_item_path(repository.root(), item.kind.as_str(), item.id.as_str()),
                        Some(item.community_id.clone()),
                    ));
                }
            }

            for entity in snapshot.people.iter().chain(&snapshot.projects) {
                for community_id in &entity.communities {
                    if !community_ids.contains(community_id.as_str()) {
                        missing_references.push(issue(
                            "missing_directory_community",
                            "warning",
                            format!(
                                "Directory entity '{}' references missing community '{}'.",
                                entity.id, community_id
                            ),
                            find_directory_entity_path(
                                repository.root(),
                                entity.kind.as_str(),
                                entity.id.as_str(),
                            ),
                            Some(community_id.clone()),
                        ));
                    }
                }
            }

            for course in &snapshot.courses {
                if !community_ids.contains(course.community_id.as_str()) {
                    missing_references.push(issue(
                        "missing_course_community",
                        "error",
                        format!(
                            "Course '{}' references missing community '{}'.",
                            course.id, course.community_id
                        ),
                        repository
                            .root()
                            .join(format!("courses/{}.json", course.id)),
                        Some(course.community_id.clone()),
                    ));
                }
            }

            for session in &snapshot.live_sessions {
                if !course_ids.contains(session.course_id.as_str()) {
                    missing_references.push(issue(
                        "missing_live_session_course",
                        "error",
                        format!(
                            "Live session '{}' references missing course '{}'.",
                            session.id, session.course_id
                        ),
                        repository
                            .root()
                            .join(format!("courses/live-sessions/{}.json", session.id)),
                        Some(session.course_id.clone()),
                    ));
                }

                if !community_ids.contains(session.community_id.as_str()) {
                    missing_references.push(issue(
                        "missing_live_session_community",
                        "error",
                        format!(
                            "Live session '{}' references missing community '{}'.",
                            session.id, session.community_id
                        ),
                        repository
                            .root()
                            .join(format!("courses/live-sessions/{}.json", session.id)),
                        Some(session.community_id.clone()),
                    ));
                }
            }

            for agent in &snapshot.agents {
                for community_id in &agent.communities {
                    if !community_ids.contains(community_id.as_str()) {
                        missing_references.push(issue(
                            "missing_agent_community",
                            "error",
                            format!(
                                "Agent '{}' references missing community '{}'.",
                                agent.id, community_id
                            ),
                            repository.root().join(format!(
                                "agents/{}/agent.json",
                                agent.id.trim_start_matches("agent-")
                            )),
                            Some(community_id.clone()),
                        ));
                    }
                }
            }

            for export in &snapshot.exports {
                if !known_object_ids.contains(export.source_object_id.as_str()) {
                    missing_references.push(issue(
                        "missing_export_source",
                        "warning",
                        format!(
                            "Export '{}' points to missing source object '{}'.",
                            export.id, export.source_object_id
                        ),
                        repository
                            .root()
                            .join(format!("exports/jobs/{}.json", export.id)),
                        Some(export.source_object_id.clone()),
                    ));
                }
            }
        }
        Err(error) => {
            invalid_paths.push(issue(
                "workspace_snapshot_failed",
                "error",
                format!("Workspace snapshot failed: {error}"),
                repository.root(),
                None,
            ));
        }
    }

    for entry in WalkDir::new(repository.root())
        .into_iter()
        .filter_map(std::result::Result::ok)
    {
        let path = entry.path();
        let display_path = path.strip_prefix(repository.root()).unwrap_or(path);
        let path_string = display_path.to_string_lossy().to_lowercase();
        let file_name = entry.file_name().to_string_lossy().to_lowercase();

        if file_name.contains("conflict") || file_name.contains("conflicted copy") {
            conflict_copies.push(issue(
                "conflict_copy_detected",
                "warning",
                "Conflict copy found in the canonical workspace.".to_string(),
                path,
                None,
            ));
        }

        if path_string.contains("lost-and-found")
            || path_string.contains("lost_and_found")
            || path_string.contains("lost+found")
        {
            lost_and_found_items.push(issue(
                "lost_and_found_item",
                "warning",
                "Lost and Found item needs review or repair.".to_string(),
                path,
                None,
            ));
        }
    }

    let summary = WorkspaceAuditSummary {
        missing_references: missing_references.len(),
        orphaned_drafts: orphaned_drafts.len(),
        conflict_copies: conflict_copies.len(),
        lost_and_found_items: lost_and_found_items.len(),
        invalid_paths: invalid_paths.len(),
    };

    Ok(WorkspaceAuditReport {
        workspace_id,
        workspace_root,
        scanned_at,
        summary,
        missing_references,
        orphaned_drafts,
        conflict_copies,
        lost_and_found_items,
        invalid_paths,
    })
}

pub fn rebuild_cache(repository: &WorkspaceRepository) -> Result<CacheRebuildReport> {
    let snapshot = repository.load_snapshot()?;
    let audit = audit_workspace(repository)?;
    let workspace_id = snapshot.settings.id.clone();
    let cache_root = cache_root(&workspace_id);
    fs::create_dir_all(&cache_root)
        .with_context(|| format!("failed creating {}", cache_root.display()))?;
    let sqlite_path = cache_root.join("cache.sqlite");
    let connection = Connection::open(&sqlite_path)
        .with_context(|| format!("failed opening {}", sqlite_path.display()))?;
    let object_counts = expected_object_counts(&snapshot);
    let rebuilt_at = Utc::now();

    initialize_cache_schema(&connection)?;
    connection.execute_batch(
        "DELETE FROM metadata;
         DELETE FROM object_counts;
         DELETE FROM issues;",
    )?;

    connection.execute(
        "INSERT INTO metadata (key, value) VALUES (?1, ?2)",
        params!["workspace_root", repository.root().display().to_string()],
    )?;
    connection.execute(
        "INSERT INTO metadata (key, value) VALUES (?1, ?2)",
        params!["workspace_id", workspace_id.clone()],
    )?;
    connection.execute(
        "INSERT INTO metadata (key, value) VALUES (?1, ?2)",
        params!["rebuilt_at", rebuilt_at.to_rfc3339()],
    )?;

    for count in &object_counts {
        connection.execute(
            "INSERT INTO object_counts (kind, count) VALUES (?1, ?2)",
            params![count.kind, count.count as i64],
        )?;
    }

    for entry in all_issues(&audit) {
        connection.execute(
            "INSERT INTO issues (code, severity, path, message, object_id) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                entry.code,
                entry.severity,
                entry.path,
                entry.message,
                entry.object_id,
            ],
        )?;
    }

    Ok(CacheRebuildReport {
        workspace_id,
        workspace_root: repository.root().display().to_string(),
        cache_root: cache_root.display().to_string(),
        sqlite_path: sqlite_path.display().to_string(),
        rebuilt_at,
        object_counts,
        issue_count: issue_count(&audit),
    })
}

fn read_workspace_settings(root: &Path) -> Option<WorkspaceSettings> {
    let path = root.join("workspace/settings.json");
    fs::read_to_string(path)
        .ok()
        .and_then(|contents| serde_json::from_str(&contents).ok())
}

fn required_path_issues(root: &Path) -> Vec<SyncIssue> {
    let required_paths = [
        "workspace/settings.json",
        "themes/theme.json",
        "communities",
        "pages",
        "directories/employees/entities",
        "directories/projects/entities",
        "libraries/documents/items",
        "libraries/videos/items",
        "courses",
        "agents",
        "automations/recipes",
        "exports/jobs",
        "notifications/items",
        "feed/segments",
    ];

    required_paths
        .iter()
        .filter_map(|relative_path| {
            let path = root.join(relative_path);
            (!path.exists()).then(|| {
                issue(
                    "required_path_missing",
                    "error",
                    format!("Required workspace path '{}' is missing.", relative_path),
                    path,
                    None,
                )
            })
        })
        .collect()
}

fn issue(
    code: &str,
    severity: &str,
    message: String,
    path: impl AsRef<Path>,
    object_id: Option<String>,
) -> SyncIssue {
    SyncIssue {
        code: code.to_string(),
        severity: severity.to_string(),
        message,
        path: path.as_ref().display().to_string(),
        object_id,
    }
}

fn find_item_path(root: &Path, kind: &str, item_id: &str) -> PathBuf {
    let library_kind = match kind {
        "video" => "videos",
        "image" => "images",
        _ => "documents",
    };
    root.join(format!("libraries/{library_kind}/items/{item_id}.json"))
}

fn find_directory_entity_path(root: &Path, kind: &str, entity_id: &str) -> PathBuf {
    let directory_kind = match kind {
        "project" => "projects",
        _ => "employees",
    };
    root.join(format!(
        "directories/{directory_kind}/entities/{entity_id}.json"
    ))
}

fn expected_object_counts(snapshot: &ui_contracts::WorkspaceSnapshot) -> Vec<CacheObjectCount> {
    vec![
        CacheObjectCount {
            kind: "pages".to_string(),
            count: snapshot.pages.len(),
        },
        CacheObjectCount {
            kind: "page_drafts".to_string(),
            count: snapshot.pages.iter().map(|page| page.drafts.len()).sum(),
        },
        CacheObjectCount {
            kind: "feed_posts".to_string(),
            count: snapshot.feed.len(),
        },
        CacheObjectCount {
            kind: "documents".to_string(),
            count: snapshot.documents.len(),
        },
        CacheObjectCount {
            kind: "videos".to_string(),
            count: snapshot.videos.len(),
        },
        CacheObjectCount {
            kind: "people".to_string(),
            count: snapshot.people.len(),
        },
        CacheObjectCount {
            kind: "projects".to_string(),
            count: snapshot.projects.len(),
        },
        CacheObjectCount {
            kind: "courses".to_string(),
            count: snapshot.courses.len(),
        },
        CacheObjectCount {
            kind: "agents".to_string(),
            count: snapshot.agents.len(),
        },
        CacheObjectCount {
            kind: "automations".to_string(),
            count: snapshot.automations.len(),
        },
        CacheObjectCount {
            kind: "exports".to_string(),
            count: snapshot.exports.len(),
        },
        CacheObjectCount {
            kind: "notifications".to_string(),
            count: snapshot.notifications.len(),
        },
        CacheObjectCount {
            kind: "live_sessions".to_string(),
            count: snapshot.live_sessions.len(),
        },
    ]
}

fn cache_root(workspace_id: &str) -> PathBuf {
    let base_root = env::var_os("WEAVE_APPDATA_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| env::temp_dir().join("weave-appdata"));
    base_root.join("workspaces").join(workspace_id)
}

fn initialize_cache_schema(connection: &Connection) -> Result<()> {
    connection.execute_batch(
        "CREATE TABLE IF NOT EXISTS metadata (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS object_counts (
            kind TEXT PRIMARY KEY,
            count INTEGER NOT NULL
        );
        CREATE TABLE IF NOT EXISTS issues (
            id INTEGER PRIMARY KEY,
            code TEXT NOT NULL,
            severity TEXT NOT NULL,
            path TEXT NOT NULL,
            message TEXT NOT NULL,
            object_id TEXT
        );",
    )?;

    Ok(())
}

fn read_last_cache_rebuild(workspace_id: &str) -> Result<Option<DateTime<Utc>>> {
    let sqlite_path = cache_root(workspace_id).join("cache.sqlite");
    if !sqlite_path.exists() {
        return Ok(None);
    }

    let connection = Connection::open(sqlite_path)?;
    let rebuilt_at = connection
        .query_row(
            "SELECT value FROM metadata WHERE key = 'rebuilt_at'",
            [],
            |row| row.get::<_, String>(0),
        )
        .optional()?;

    rebuilt_at
        .map(|value| {
            DateTime::parse_from_rfc3339(&value)
                .map(|timestamp| timestamp.with_timezone(&Utc))
                .with_context(|| format!("invalid rebuilt_at timestamp '{value}'"))
        })
        .transpose()
}

fn load_cached_counts(workspace_id: &str) -> Result<HashMap<String, usize>> {
    let sqlite_path = cache_root(workspace_id).join("cache.sqlite");
    if !sqlite_path.exists() {
        return Ok(HashMap::new());
    }

    let connection = Connection::open(sqlite_path)?;
    let mut statement = connection.prepare("SELECT kind, count FROM object_counts")?;
    let rows = statement.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)? as usize))
    })?;

    let mut counts = HashMap::new();
    for row in rows {
        let (kind, count) = row?;
        counts.insert(kind, count);
    }

    Ok(counts)
}

fn all_issues(report: &WorkspaceAuditReport) -> Vec<SyncIssue> {
    report
        .missing_references
        .iter()
        .chain(&report.orphaned_drafts)
        .chain(&report.conflict_copies)
        .chain(&report.lost_and_found_items)
        .chain(&report.invalid_paths)
        .cloned()
        .collect()
}

fn issue_count(report: &WorkspaceAuditReport) -> usize {
    report.summary.missing_references
        + report.summary.orphaned_drafts
        + report.summary.conflict_copies
        + report.summary.lost_and_found_items
        + report.summary.invalid_paths
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use workspace::find_fixture_root;

    #[test]
    fn audit_detects_missing_references_and_conflicts() {
        let temp = fixture_copy();
        let root = temp.path().join("WEAVE");

        fs::write(
            root.join("communities/home.json"),
            r##"{
  "schema_version": 1,
  "object_type": "community",
  "id": "home",
  "slug": "home",
  "title": "Home",
  "description": "Home base",
  "accent": "#cb5a2d",
  "home_page_id": "page-missing"
}"##,
        )
        .expect("community override");
        fs::write(root.join("feed/conflict-copy.json"), "{}").expect("conflict copy");
        fs::create_dir_all(root.join("lost-and-found")).expect("lost and found dir");
        fs::write(root.join("lost-and-found/orphan.json"), "{}").expect("lost and found item");

        let report = audit_workspace(&WorkspaceRepository::new(root)).expect("audit report");
        assert!(report.summary.missing_references >= 1);
        assert_eq!(report.summary.conflict_copies, 1);
        assert!(report.summary.lost_and_found_items >= 1);
    }

    #[test]
    fn rebuild_cache_creates_sqlite_report() {
        let temp = TempDir::new().expect("temp dir");
        let root = find_fixture_root().expect("fixture root");
        let workspace_id = "weave-studio";
        let appdata_root = temp.path().join("appdata");
        unsafe {
            env::set_var("WEAVE_APPDATA_ROOT", &appdata_root);
        }

        let report = rebuild_cache(&WorkspaceRepository::new(root)).expect("cache rebuild");
        assert!(Path::new(&report.sqlite_path).exists());
        assert_eq!(report.workspace_id, workspace_id);

        let health = sync_health(&WorkspaceRepository::new(
            find_fixture_root().expect("fixture root"),
        ))
        .expect("sync health");
        assert!(health.last_cache_rebuild.is_some());

        unsafe {
            env::remove_var("WEAVE_APPDATA_ROOT");
        }
    }

    fn fixture_copy() -> TempDir {
        let temp = TempDir::new().expect("temp dir");
        let source = find_fixture_root().expect("fixture root");
        let target = temp.path().join("WEAVE");
        copy_dir_all(&source, &target).expect("copied fixture");
        temp
    }

    fn copy_dir_all(source: &Path, target: &Path) -> Result<()> {
        fs::create_dir_all(target)?;
        for entry in WalkDir::new(source)
            .into_iter()
            .filter_map(std::result::Result::ok)
        {
            let relative = entry.path().strip_prefix(source)?;
            let destination = target.join(relative);

            if entry.file_type().is_dir() {
                fs::create_dir_all(&destination)?;
            } else {
                fs::copy(entry.path(), &destination)?;
            }
        }

        Ok(())
    }
}
