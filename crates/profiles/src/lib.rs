use anyhow::Result;
use ui_contracts::DirectoryEntity;
use workspace::WorkspaceRepository;

pub fn list_entities(repository: &WorkspaceRepository, kind: &str) -> Result<Vec<DirectoryEntity>> {
    let snapshot = repository.load_snapshot()?;
    let entities = match kind {
        "employees" => snapshot.people,
        "projects" => snapshot.projects,
        _ => Vec::new(),
    };
    Ok(entities)
}
