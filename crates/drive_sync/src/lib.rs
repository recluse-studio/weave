use anyhow::Result;
use ui_contracts::SyncHealth;
use workspace::WorkspaceRepository;

pub fn sync_health(repository: &WorkspaceRepository) -> Result<SyncHealth> {
    Ok(repository.load_snapshot()?.sync_health)
}
