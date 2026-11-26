use std::sync::Arc;
use std::time::Instant;

use askama::Template;
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;
use tower_http::services::ServeDir;

use crate::data::AppData;
use crate::view::{
    SearchPageView, SearchResultsView, build_page, build_search_page, default_competition,
    search_results_view,
};

#[derive(Clone)]
pub struct AppState {
    pub data: Arc<AppData>,
    pub started_at: Instant,
}

#[derive(Debug, Default, Deserialize)]
pub struct PageQuery {
    pub season: Option<String>,
    pub group: Option<String>,
    pub highlight: Option<String>,
    pub q: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
}

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorPageView {
    page_title: String,
    status: u16,
    heading: String,
    message: String,
}

fn error_response(status: StatusCode, heading: &str, message: &str) -> Response {
    let view = ErrorPageView {
        page_title: format!("{heading} · Thailand Football"),
        status: status.as_u16(),
        heading: heading.to_string(),
        message: message.to_string(),
    };
    match view.render() {
        Ok(body) => (status, Html(body)).into_response(),
        Err(error) => {
            tracing::error!(%error, "failed to render error page");
            (status, "Something went wrong.").into_response()
        }
    }
}

async fn root(State(state): State<AppState>) -> Response {
    match default_competition(&state.data) {
        Some(comp) => Redirect::to(&format!("/competitions/{}", comp.id)).into_response(),
        None => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "No competitions",
            "No active competitions are available in the local snapshot.",
        ),
    }
}

async fn competition_default(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(query): Query<PageQuery>,
) -> Response {
    render_competition(&state, &id, None, &query)
}

async fn competition_tab(
    State(state): State<AppState>,
    Path((id, tab)): Path<(String, String)>,
    Query(query): Query<PageQuery>,
) -> Response {
    render_competition(&state, &id, Some(&tab), &query)
}

fn render_competition(
    state: &AppState,
    id: &str,
    tab: Option<&str>,
    query: &PageQuery,
) -> Response {
    let Some(page) = build_page(
        &state.data,
        id,
        query.season.as_deref(),
        tab,
        query.group.as_deref(),
        query.highlight.as_deref(),
        query.q.as_deref().unwrap_or(""),
    ) else {
        return error_response(
            StatusCode::NOT_FOUND,
            "Competition not found",
            "This competition does not exist or is not an active sidebar competition.",
        );
    };
    match page.render() {
        Ok(body) => Html(body).into_response(),
        Err(error) => {
            tracing::error!(%error, competition = id, "failed to render competition page");
            error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Render error",
                "The competition page could not be rendered.",
            )
        }
    }
}

async fn search(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<SearchQuery>,
) -> Response {
    let q = query.q.unwrap_or_default();
    let is_htmx = headers
        .get("HX-Request")
        .and_then(|value| value.to_str().ok())
        == Some("true");

    if is_htmx {
        let view: SearchResultsView = search_results_view(&state.data, &q);
        match view.render() {
            Ok(body) => Html(body).into_response(),
            Err(error) => {
                tracing::error!(%error, "failed to render search partial");
                (StatusCode::INTERNAL_SERVER_ERROR, "Search failed.").into_response()
            }
        }
    } else {
        let view: SearchPageView = build_search_page(&state.data, &q);
        match view.render() {
            Ok(body) => Html(body).into_response(),
            Err(error) => {
                tracing::error!(%error, "failed to render search page");
                error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Search error",
                    "The search page could not be rendered.",
                )
            }
        }
    }
}

async fn health(State(state): State<AppState>) -> Json<serde_json::Value> {
    let competitions = state.data.active_competitions().count();
    let data_mode = state.data.seed.meta.mode.clone();
    let source = state.data.seed.meta.legacy_source.clone();
    let team_rows: usize = state
        .data
        .active_competitions()
        .flat_map(|c| c.seasons.iter())
        .flat_map(|s| s.groups.iter())
        .map(|g| g.standings.len())
        .sum();
    Json(json!({
        "status": "ok",
        "mode": data_mode,
        "competitions": competitions,
        "team_rows": team_rows,
        "search_entries": state.data.search_index.len(),
        "news_articles": state.data.seed.news.len(),
        "uptime_seconds": state.started_at.elapsed().as_secs(),
        "snapshot_source": source,
    }))
}

async fn fallback() -> Response {
    error_response(
        StatusCode::NOT_FOUND,
        "Page not found",
        "The requested page does not exist.",
    )
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/competitions/{id}", get(competition_default))
        .route("/competitions/{id}/{tab}", get(competition_tab))
        .route("/search", get(search))
        .route("/healthz", get(health))
        .nest_service("/static", ServeDir::new("static"))
        .fallback(fallback)
        .with_state(state)
}
