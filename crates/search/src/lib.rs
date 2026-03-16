use anyhow::Result;
use gemini_broker::synthesize_answer;
use ui_contracts::{SearchResponse, SearchResult};
use workspace::WorkspaceRepository;

pub fn search_workspace(repository: &WorkspaceRepository, query: &str) -> Result<SearchResponse> {
    let snapshot = repository.load_snapshot()?;
    let needle = query.trim().to_lowercase();
    let mut results = Vec::new();

    for page in snapshot.pages {
        let haystack = format!(
            "{} {} {}",
            page.meta.title, page.meta.excerpt, page.published_revision.summary
        )
        .to_lowercase();
        if let Some(score) = score(&needle, &haystack, 120) {
            results.push(SearchResult {
                id: page.meta.id.clone(),
                kind: "page".to_string(),
                title: page.meta.title.clone(),
                snippet: page.meta.excerpt.clone(),
                path: format!("/pages/{}", page.meta.slug),
                citation: format!("{} > {}", page.meta.title, page.published_revision.summary),
                score,
            });
        }
    }

    for post in snapshot.feed {
        let haystack =
            format!("{} {} {}", post.title, post.body, post.hashtags.join(" ")).to_lowercase();
        if let Some(score) = score(&needle, &haystack, 90) {
            results.push(SearchResult {
                id: post.id.clone(),
                kind: "post".to_string(),
                title: post.title.clone(),
                snippet: post.body.clone(),
                path: format!("/feed/{}", post.id),
                citation: format!("{} by {}", post.title, post.author_name),
                score,
            });
        }
    }

    for item in snapshot.documents.into_iter().chain(snapshot.videos) {
        let haystack = format!(
            "{} {} {}",
            item.title,
            item.description,
            item.tags.join(" ")
        )
        .to_lowercase();
        if let Some(score) = score(&needle, &haystack, 80) {
            results.push(SearchResult {
                id: item.id.clone(),
                kind: item.kind.clone(),
                title: item.title.clone(),
                snippet: item.description.clone(),
                path: format!("/library/{}", item.slug),
                citation: item.citation_hint.unwrap_or_else(|| item.title.clone()),
                score,
            });
        }
    }

    for entity in snapshot.people.into_iter().chain(snapshot.projects) {
        let haystack =
            format!("{} {} {}", entity.name, entity.title, entity.summary).to_lowercase();
        if let Some(score) = score(&needle, &haystack, 100) {
            results.push(SearchResult {
                id: entity.id.clone(),
                kind: entity.kind.clone(),
                title: entity.name.clone(),
                snippet: entity.summary.clone(),
                path: format!("/directory/{}", entity.slug),
                citation: format!("{} > {}", entity.name, entity.title),
                score,
            });
        }
    }

    for course in snapshot.courses {
        let haystack =
            format!("{} {} {}", course.title, course.summary, course.status).to_lowercase();
        if let Some(score) = score(&needle, &haystack, 110) {
            results.push(SearchResult {
                id: course.id.clone(),
                kind: "course".to_string(),
                title: course.title.clone(),
                snippet: course.summary.clone(),
                path: format!("/courses/{}", course.slug),
                citation: format!("{} > {} minutes", course.title, course.duration_minutes),
                score,
            });
        }
    }

    results.sort_by(|left, right| right.score.cmp(&left.score));
    let answer = synthesize_answer(query, &results);

    Ok(SearchResponse {
        query: query.to_string(),
        results,
        answer,
    })
}

fn score(needle: &str, haystack: &str, base: i32) -> Option<i32> {
    if needle.is_empty() {
        return None;
    }

    if haystack.contains(needle) {
        Some(base + needle.len() as i32)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use workspace::{WorkspaceRepository, find_fixture_root};

    #[test]
    fn finds_launch_related_content() {
        let repository = WorkspaceRepository::new(find_fixture_root().expect("fixture root"));
        let response = search_workspace(&repository, "launch").expect("search results");

        assert!(!response.results.is_empty());
        assert!(response.answer.summary.contains("launch"));
    }
}
