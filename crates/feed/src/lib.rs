use std::fs::{self, OpenOptions};
use std::io::Write;

use anyhow::Result;
use chrono::{Datelike, Utc};
use ui_contracts::{CreateFeedPostRequest, FeedPost};
use uuid::Uuid;
use workspace::WorkspaceRepository;

pub fn list_posts(repository: &WorkspaceRepository) -> Result<Vec<FeedPost>> {
    Ok(repository.load_snapshot()?.feed)
}

pub fn append_post(
    repository: &WorkspaceRepository,
    request: CreateFeedPostRequest,
) -> Result<FeedPost> {
    let now = Utc::now();
    let year = now.year();
    let month = now.month();
    let day = now.day();
    let segment_dir = repository
        .root()
        .join(format!("feed/segments/{year:04}/{month:02}/{day:02}"));
    fs::create_dir_all(&segment_dir)?;
    let segment_path = segment_dir.join("feed-0001.jsonl");

    let post = FeedPost {
        schema_version: 1,
        object_type: "feed_post".to_string(),
        id: format!("post-{}", Uuid::new_v4()),
        community_id: request.community_id,
        author_id: request.author_id,
        author_name: request.author_name,
        title: request.title,
        body: request.body,
        hashtags: request.hashtags,
        likes: 0,
        comments: 0,
        promoted: request.promoted,
        published_at: now,
    };

    let serialized = serde_json::to_string(&post)?;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(segment_path)?;
    writeln!(file, "{serialized}")?;

    Ok(post)
}
