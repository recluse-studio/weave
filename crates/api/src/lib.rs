use std::sync::Arc;

use automations::{list_recipes, preview};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use drive_sync::sync_health;
use feed::{append_post, list_posts};
use libraries::list_items;
use lms::list_courses;
use pages::{get_page, list_drafts, list_pages, publish_page, save_draft};
use profiles::list_entities;
use search::search_workspace;
use serde::Deserialize;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use ui_contracts::{CreateFeedPostRequest, PublishPageRequest, SaveDraftRequest};
use workspace::WorkspaceRepository;

#[derive(Clone)]
pub struct AppState {
    repository: Arc<WorkspaceRepository>,
}

impl AppState {
    #[must_use]
    pub fn new(repository: WorkspaceRepository) -> Self {
        Self {
            repository: Arc::new(repository),
        }
    }
}

pub fn app(repository: WorkspaceRepository) -> Router {
    let state = AppState::new(repository);

    Router::new()
        .route("/api/dashboard", get(dashboard))
        .route("/api/workspace", get(workspace))
        .route("/api/pages", get(pages))
        .route("/api/pages/{page_id}", get(page))
        .route(
            "/api/pages/{page_id}/drafts",
            get(page_drafts).post(draft_save),
        )
        .route("/api/pages/{page_id}/publish", post(publish))
        .route("/api/feed", get(feed_index).post(feed_create))
        .route("/api/directories/{kind}", get(directory_index))
        .route("/api/libraries/{kind}", get(library_index))
        .route("/api/courses", get(courses))
        .route("/api/agents", get(agent_index))
        .route("/api/search", get(search))
        .route("/api/automations", get(automation_index))
        .route(
            "/api/automations/{recipe_id}/preview",
            get(automation_preview),
        )
        .route("/api/google/previews", get(google_preview_index))
        .route("/api/health", get(health))
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}

async fn dashboard(State(state): State<AppState>) -> ApiResult<impl IntoResponse> {
    Ok(Json(state.repository.dashboard()?))
}

async fn workspace(State(state): State<AppState>) -> ApiResult<impl IntoResponse> {
    Ok(Json(state.repository.load_snapshot()?.settings))
}

async fn pages(State(state): State<AppState>) -> ApiResult<impl IntoResponse> {
    Ok(Json(list_pages(&state.repository)?))
}

async fn page(
    State(state): State<AppState>,
    Path(page_id): Path<String>,
) -> ApiResult<impl IntoResponse> {
    Ok(Json(get_page(&state.repository, &page_id)?))
}

async fn publish(
    State(state): State<AppState>,
    Path(page_id): Path<String>,
    Json(request): Json<PublishPageRequest>,
) -> ApiResult<impl IntoResponse> {
    Ok(Json(publish_page(&state.repository, &page_id, request)?))
}

async fn page_drafts(
    State(state): State<AppState>,
    Path(page_id): Path<String>,
) -> ApiResult<impl IntoResponse> {
    Ok(Json(list_drafts(&state.repository, &page_id)?))
}

async fn draft_save(
    State(state): State<AppState>,
    Path(page_id): Path<String>,
    Json(request): Json<SaveDraftRequest>,
) -> ApiResult<impl IntoResponse> {
    Ok((
        StatusCode::CREATED,
        Json(save_draft(&state.repository, &page_id, request)?),
    ))
}

async fn feed_index(State(state): State<AppState>) -> ApiResult<impl IntoResponse> {
    Ok(Json(list_posts(&state.repository)?))
}

async fn feed_create(
    State(state): State<AppState>,
    Json(request): Json<CreateFeedPostRequest>,
) -> ApiResult<impl IntoResponse> {
    Ok((
        StatusCode::CREATED,
        Json(append_post(&state.repository, request)?),
    ))
}

async fn directory_index(
    State(state): State<AppState>,
    Path(kind): Path<String>,
) -> ApiResult<impl IntoResponse> {
    Ok(Json(list_entities(&state.repository, &kind)?))
}

async fn library_index(
    State(state): State<AppState>,
    Path(kind): Path<String>,
) -> ApiResult<impl IntoResponse> {
    Ok(Json(list_items(&state.repository, &kind)?))
}

async fn courses(State(state): State<AppState>) -> ApiResult<impl IntoResponse> {
    Ok(Json(list_courses(&state.repository)?))
}

async fn agent_index(State(state): State<AppState>) -> ApiResult<impl IntoResponse> {
    Ok(Json(state.repository.load_snapshot()?.agents))
}

#[derive(Debug, Deserialize)]
struct SearchQuery {
    q: String,
}

async fn search(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> ApiResult<impl IntoResponse> {
    Ok(Json(search_workspace(&state.repository, &query.q)?))
}

async fn automation_index(State(state): State<AppState>) -> ApiResult<impl IntoResponse> {
    Ok(Json(list_recipes(&state.repository)?))
}

async fn automation_preview(
    State(state): State<AppState>,
    Path(recipe_id): Path<String>,
) -> ApiResult<impl IntoResponse> {
    Ok(Json(preview(&state.repository, &recipe_id)?))
}

async fn google_preview_index(State(state): State<AppState>) -> ApiResult<impl IntoResponse> {
    Ok(Json(state.repository.load_snapshot()?.google_previews))
}

async fn health(State(state): State<AppState>) -> ApiResult<impl IntoResponse> {
    Ok(Json(sync_health(&state.repository)?))
}

type ApiResult<T> = std::result::Result<T, ApiError>;

pub struct ApiError(anyhow::Error);

impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(error: E) -> Self {
        Self(error.into())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;
    use workspace::find_fixture_root;

    #[tokio::test]
    async fn dashboard_endpoint_responds() {
        let app = app(WorkspaceRepository::new(
            find_fixture_root().expect("fixture root"),
        ));
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/dashboard")
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("response");

        assert_eq!(response.status(), StatusCode::OK);
    }
}
