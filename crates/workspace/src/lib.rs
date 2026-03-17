use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, anyhow};
use chrono::Utc;
use gws_adapter::preview_workspace_actions;
use serde::de::DeserializeOwned;
use ui_contracts::{
    AgentRecord, AutomationRecipe, CommunityRecord, CourseRecord, DashboardSnapshot,
    DirectoryEntity, FeedPost, LibraryItem, PageDraft, PageMeta, PageRecord, PageRevision,
    SyncHealth, ThemeDefinition, WorkspaceSettings, WorkspaceSnapshot, WorkspaceSummary,
};
use walkdir::WalkDir;

#[derive(Clone, Debug)]
pub struct WorkspaceRepository {
    root: PathBuf,
}

impl WorkspaceRepository {
    #[must_use]
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn load_snapshot(&self) -> Result<WorkspaceSnapshot> {
        let settings: WorkspaceSettings = self.read_json("workspace/settings.json")?;
        let theme: ThemeDefinition = self.read_json("themes/theme.json")?;
        let communities = self.read_json_dir::<CommunityRecord>("communities")?;
        let pages = self.load_pages()?;
        let people = self.read_json_dir::<DirectoryEntity>("directories/employees/entities")?;
        let projects = self.read_json_dir::<DirectoryEntity>("directories/projects/entities")?;
        let documents = self.read_json_dir::<LibraryItem>("libraries/documents/items")?;
        let videos = self.read_json_dir::<LibraryItem>("libraries/videos/items")?;
        let courses = self.load_courses()?;
        let feed = self.load_feed()?;
        let agents = self.load_agents()?;
        let automations = self.load_automations()?;
        let sync_health = SyncHealth {
            workspace_root: self.root.display().to_string(),
            drive_mode: settings.storage_mode.clone(),
            ownership_mode: settings.ownership_mode.clone(),
            last_local_scan: Utc::now(),
            last_replay: Utc::now(),
            unresolved_conflicts: 0,
            stale_cache_count: 0,
        };

        let mut snapshot = WorkspaceSnapshot {
            settings,
            theme,
            communities,
            pages,
            people,
            projects,
            documents,
            videos,
            courses,
            feed,
            agents,
            automations,
            google_previews: Vec::new(),
            sync_health,
        };
        snapshot.google_previews = preview_workspace_actions(&snapshot);

        Ok(snapshot)
    }

    pub fn dashboard(&self) -> Result<DashboardSnapshot> {
        let snapshot = self.load_snapshot()?;

        Ok(DashboardSnapshot {
            workspace: WorkspaceSummary {
                id: snapshot.settings.id.clone(),
                name: snapshot.settings.name.clone(),
                tagline: snapshot.settings.tagline.clone(),
                pages: snapshot.pages.len(),
                posts: snapshot.feed.len(),
                documents: snapshot.documents.len(),
                videos: snapshot.videos.len(),
                courses: snapshot.courses.len(),
            },
            communities: snapshot.communities.clone(),
            featured_pages: snapshot
                .pages
                .iter()
                .filter(|page| page.meta.featured)
                .map(|page| page.meta.clone())
                .collect(),
            promoted_posts: snapshot
                .feed
                .iter()
                .filter(|post| post.promoted)
                .cloned()
                .collect(),
            featured_people: snapshot
                .people
                .iter()
                .filter(|person| person.featured)
                .cloned()
                .collect(),
            featured_projects: snapshot
                .projects
                .iter()
                .filter(|project| project.featured)
                .cloned()
                .collect(),
            featured_documents: snapshot.documents.iter().take(4).cloned().collect(),
            featured_videos: snapshot.videos.iter().take(4).cloned().collect(),
            featured_courses: snapshot
                .courses
                .iter()
                .filter(|course| course.featured)
                .cloned()
                .collect(),
            agents: snapshot.agents.clone(),
            automations: snapshot.automations.clone(),
            google_previews: snapshot.google_previews.clone(),
            sync_health: snapshot.sync_health,
        })
    }

    fn load_pages(&self) -> Result<Vec<PageRecord>> {
        let pages_dir = self.root.join("pages");
        if !pages_dir.exists() {
            return Ok(Vec::new());
        }

        let mut pages = fs::read_dir(&pages_dir)?
            .filter_map(std::result::Result::ok)
            .filter(|entry| entry.path().is_dir())
            .map(|entry| self.load_page(&entry.path()))
            .collect::<Result<Vec<_>>>()?;

        pages.sort_by(|left, right| right.meta.updated_at.cmp(&left.meta.updated_at));
        Ok(pages)
    }

    fn load_page(&self, page_dir: &Path) -> Result<PageRecord> {
        let meta: PageMeta = read_json_file(&page_dir.join("meta.json"))?;
        let published_ref = fs::read_to_string(page_dir.join("published.ref"))
            .with_context(|| format!("missing published ref for {}", meta.id))?;
        let revision_path = page_dir.join("revisions").join(published_ref.trim());
        let published_revision: PageRevision = read_json_file(&revision_path)?;
        let revisions =
            self.read_json_dir_from_path::<PageRevision>(&page_dir.join("revisions"))?;
        let mut drafts = self.read_json_dir_from_path::<PageDraft>(&page_dir.join("drafts"))?;
        drafts.sort_by(|left, right| right.updated_at.cmp(&left.updated_at));

        Ok(PageRecord {
            meta,
            published_revision,
            revisions,
            drafts,
        })
    }

    fn load_feed(&self) -> Result<Vec<FeedPost>> {
        let feed_root = self.root.join("feed/segments");
        if !feed_root.exists() {
            return Ok(Vec::new());
        }

        let mut posts: Vec<FeedPost> = Vec::new();
        for entry in WalkDir::new(feed_root)
            .into_iter()
            .filter_map(std::result::Result::ok)
            .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "jsonl"))
        {
            let contents = fs::read_to_string(entry.path())?;
            for line in contents.lines().filter(|line| !line.trim().is_empty()) {
                posts.push(serde_json::from_str(line).with_context(|| {
                    format!("failed parsing feed line in {}", entry.path().display())
                })?);
            }
        }

        posts.sort_by(|left, right| right.published_at.cmp(&left.published_at));
        Ok(posts)
    }

    fn load_courses(&self) -> Result<Vec<CourseRecord>> {
        let course_root = self.root.join("courses");
        if !course_root.exists() {
            return Ok(Vec::new());
        }

        let mut courses = fs::read_dir(course_root)?
            .filter_map(std::result::Result::ok)
            .map(|entry| entry.path())
            .filter(|path| {
                path.is_file()
                    && path
                        .file_name()
                        .and_then(|name| name.to_str())
                        .is_some_and(|name| name.starts_with("course-") && name.ends_with(".json"))
            })
            .map(|path| read_json_file::<CourseRecord>(&path))
            .collect::<Result<Vec<_>>>()?;

        courses.sort_by(|left, right| left.title.cmp(&right.title));
        Ok(courses)
    }

    fn load_automations(&self) -> Result<Vec<AutomationRecipe>> {
        let recipe_root = self.root.join("automations/recipes");
        if !recipe_root.exists() {
            return Ok(Vec::new());
        }

        let mut recipes = fs::read_dir(recipe_root)?
            .filter_map(std::result::Result::ok)
            .map(|entry| entry.path())
            .filter(|path| path.extension().is_some_and(|ext| ext == "yaml"))
            .map(|path| {
                let contents = fs::read_to_string(&path)?;
                serde_yaml::from_str::<AutomationRecipe>(&contents)
                    .with_context(|| format!("failed parsing {}", path.display()))
            })
            .collect::<Result<Vec<_>>>()?;

        recipes.sort_by(|left, right| left.name.cmp(&right.name));
        Ok(recipes)
    }

    fn load_agents(&self) -> Result<Vec<AgentRecord>> {
        let agent_root = self.root.join("agents");
        if !agent_root.exists() {
            return Ok(Vec::new());
        }

        let mut agents = fs::read_dir(agent_root)?
            .filter_map(std::result::Result::ok)
            .map(|entry| entry.path())
            .filter(|path| path.is_dir())
            .map(|path| read_json_file::<AgentRecord>(&path.join("agent.json")))
            .collect::<Result<Vec<_>>>()?;

        agents.sort_by(|left, right| left.name.cmp(&right.name));
        Ok(agents)
    }

    fn read_json<T>(&self, relative_path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        read_json_file(&self.root.join(relative_path))
    }

    fn read_json_dir<T>(&self, relative_dir: &str) -> Result<Vec<T>>
    where
        T: DeserializeOwned,
    {
        self.read_json_dir_from_path(&self.root.join(relative_dir))
    }

    fn read_json_dir_from_path<T>(&self, dir: &Path) -> Result<Vec<T>>
    where
        T: DeserializeOwned,
    {
        if !dir.exists() {
            return Ok(Vec::new());
        }

        let records = fs::read_dir(&dir)?
            .filter_map(std::result::Result::ok)
            .map(|entry| entry.path())
            .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
            .map(|path| read_json_file(&path))
            .collect::<Result<Vec<_>>>()?;
        Ok(records)
    }
}

fn read_json_file<T>(path: &Path) -> Result<T>
where
    T: DeserializeOwned,
{
    let contents =
        fs::read_to_string(path).with_context(|| format!("failed reading {}", path.display()))?;
    serde_json::from_str(&contents).with_context(|| format!("failed parsing {}", path.display()))
}

pub fn find_fixture_root() -> Result<PathBuf> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = manifest_dir
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| anyhow!("unable to derive repository root"))?
        .join("fixtures/demo-workspace/WEAVE");
    Ok(root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_fixture_workspace() {
        let repository = WorkspaceRepository::new(find_fixture_root().expect("fixture root"));
        let snapshot = repository.load_snapshot().expect("workspace snapshot");

        assert_eq!(snapshot.settings.name, "WEAVE Studio");
        assert!(snapshot.pages.len() >= 3);
        assert!(snapshot.feed.len() >= 3);
        assert!(snapshot.pages.iter().any(|page| !page.drafts.is_empty()));
        assert!(!snapshot.agents.is_empty());
        assert!(!snapshot.google_previews.is_empty());
        assert!(
            snapshot
                .documents
                .iter()
                .any(|item| item.kind == "document")
        );
        assert!(snapshot.videos.iter().any(|item| item.kind == "video"));
    }
}
