use anyhow::Result;
use ui_contracts::LibraryItem;
use workspace::WorkspaceRepository;

pub fn list_items(repository: &WorkspaceRepository, kind: &str) -> Result<Vec<LibraryItem>> {
    let snapshot = repository.load_snapshot()?;
    let items = match kind {
        "documents" => snapshot.documents,
        "videos" => snapshot.videos,
        _ => Vec::new(),
    };
    Ok(items)
}
