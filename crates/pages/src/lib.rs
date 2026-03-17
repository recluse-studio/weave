use std::{fs, path::Path};

use anyhow::{Context, Result, anyhow};
use chrono::Utc;
use ui_contracts::{
    PageDraft, PageMeta, PageRecord, PageRevision, PublishPageRequest, SaveDraftRequest,
};
use workspace::WorkspaceRepository;

pub fn list_pages(repository: &WorkspaceRepository) -> Result<Vec<PageMeta>> {
    Ok(repository
        .load_snapshot()?
        .pages
        .into_iter()
        .map(|page| page.meta)
        .collect())
}

pub fn get_page(repository: &WorkspaceRepository, page_id: &str) -> Result<PageRecord> {
    repository
        .load_snapshot()?
        .pages
        .into_iter()
        .find(|page| page.meta.id == page_id)
        .ok_or_else(|| anyhow!("page not found: {page_id}"))
}

pub fn publish_page(
    repository: &WorkspaceRepository,
    page_id: &str,
    request: PublishPageRequest,
) -> Result<PageRecord> {
    let page_root = repository.root().join("pages").join(page_id);
    let meta_path = page_root.join("meta.json");
    let revision_dir = page_root.join("revisions");
    fs::create_dir_all(&revision_dir)?;

    let mut record = get_page(repository, page_id)?;
    let now = Utc::now();
    let revision_file = format!(
        "{}_{}.json",
        now.format("%Y-%m-%dT%H-%M-%SZ"),
        request.author
    );

    record.meta.title = request.title.clone();
    record.meta.excerpt = request.summary.clone();
    record.meta.updated_at = now;
    record.published_revision = PageRevision {
        schema_version: 1,
        object_type: "page_revision".to_string(),
        id: format!("{page_id}-{}", now.timestamp()),
        page_id: page_id.to_string(),
        title: request.title,
        summary: request.summary,
        author: request.author.clone(),
        updated_at: now,
        blocks: request.blocks,
    };

    write_pretty_json(&meta_path, &record.meta)?;
    write_pretty_json(
        &revision_dir.join(&revision_file),
        &record.published_revision,
    )?;
    fs::write(
        page_root.join("published.ref"),
        format!("{revision_file}\n"),
    )
    .with_context(|| format!("failed updating published ref for {page_id}"))?;

    get_page(repository, page_id)
}

pub fn list_drafts(repository: &WorkspaceRepository, page_id: &str) -> Result<Vec<PageDraft>> {
    Ok(get_page(repository, page_id)?.drafts)
}

pub fn save_draft(
    repository: &WorkspaceRepository,
    page_id: &str,
    request: SaveDraftRequest,
) -> Result<PageDraft> {
    let page_root = repository.root().join("pages").join(page_id);
    let draft_dir = page_root.join("drafts");
    fs::create_dir_all(&draft_dir)?;

    let draft = PageDraft {
        schema_version: 1,
        object_type: "page_draft".to_string(),
        page_id: page_id.to_string(),
        author: request.author.clone(),
        title: request.title,
        summary: request.summary,
        updated_at: Utc::now(),
        blocks: request.blocks,
    };

    write_pretty_json(&draft_dir.join(format!("{}.json", request.author)), &draft)?;

    Ok(draft)
}

fn write_pretty_json<T: serde::Serialize>(path: &Path, value: &T) -> Result<()> {
    let contents = serde_json::to_string_pretty(value)?;
    fs::write(path, contents).with_context(|| format!("failed writing {}", path.display()))?;
    Ok(())
}
