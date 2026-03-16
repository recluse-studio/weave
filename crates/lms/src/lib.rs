use anyhow::Result;
use ui_contracts::CourseRecord;
use workspace::WorkspaceRepository;

pub fn list_courses(repository: &WorkspaceRepository) -> Result<Vec<CourseRecord>> {
    Ok(repository.load_snapshot()?.courses)
}
