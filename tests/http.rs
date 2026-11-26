#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use thailand_football::data::AppData;
use thailand_football::routes::{AppState, router};
use tower::ServiceExt;

fn app() -> axum::Router {
    let data = AppData::from_path(Path::new("data/seed.json")).expect("seed should load");
    router(AppState {
        data: Arc::new(data),
        started_at: Instant::now(),
    })
}

async fn body_text(response: axum::response::Response) -> String {
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    String::from_utf8_lossy(&bytes).to_string()
}

#[tokio::test]
async fn health_endpoint_reports_preview_mode() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = body_text(response).await;
    assert!(body.contains("\"status\":\"ok\""));
    assert!(body.contains("\"mode\":\"local-preview\""));
}

#[tokio::test]
async fn main_routes_render() {
    for uri in [
        "/competitions/thai-league-1",
        "/competitions/thai-league-1/results",
        "/competitions/thai-league-1/scorers",
        "/competitions/thai-league-1/teams",
        "/competitions/thai-league-3/teams?group=group-2",
        "/competitions/thai-youth-league-u18/teams",
        "/competitions/thai-league-2/standings",
        "/competitions/thai-league-3/standings?group=group-2",
        "/competitions/thai-fa-cup/rounds",
        "/competitions/thai-youth-league-u18/standings",
        "/competitions/thai-youth-league-u18/teams?season=2026-u16",
        "/competitions/thai-youth-league-u16/standings?season=2026-u14",
        "/competitions/thai-youth-league-u16/fixtures?season=2026-u18",
        "/competitions/thai-youth-league-u14/results?season=2026-u16",
        "/competitions/thai-youth-league-u14/teams?season=2026-u14",
        "/competitions/thai-school-league-type-a?season=2026-u18",
        "/competitions/thai-school-league-general?season=2026-u12",
        "/competitions/thai-college-cup-u19/rounds",
        "/competitions/thai-youth-fa-cup-u17/rounds",
        "/competitions/thai-youth-league-cup-u18/rounds",
        "/competitions/thai-youth-champion-cup-u18/rounds",
        "/search?q=buriram",
    ] {
        let response = app()
            .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK, "uri {uri}");
    }
}

#[tokio::test]
async fn inactive_legacy_competition_is_not_routed() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/competitions/legacy-thai-league-4")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn search_partial_is_returned_for_htmx() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/search?q=buriram")
                .header("HX-Request", "true")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = body_text(response).await;
    assert!(body.contains("Buriram United"));
    assert!(body.contains("Thai League 1"));
    assert!(!body.contains("<html"));
}

#[tokio::test]
async fn sidebar_contains_latest_owner_labels() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/competitions/thai-league-1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = body_text(response).await;
    for label in [
        "Thai Champion Cup",
        "Youth League",
        "FAT",
        "SAT",
        "Thai Youth League 1",
        "Thai Youth League 2",
        "Thai Youth League 3",
        "ฟุตบอลเยาวชน สมาคมฯ ลีก 1",
        "ฟุตบอลเยาวชน สมาคมฯ ลีก 2",
        "ฟุตบอลเยาวชน สมาคมฯ ลีก 3",
        "School League",
        "Type A",
        "Type B",
        "General",
        "Thai College Cup (U19)",
        "Thai University Cup (U23)",
        "Thai Youth FA Cup (U17)",
        "Thai Youth League Cup (U18)",
        "Thai Youth Champion Cup (U18)",
        "Coke Cup",
    ] {
        assert!(body.contains(label), "missing sidebar label: {label}");
    }
    assert!(
        !body.contains("Thai Youth League 1 (U18)"),
        "age group must not remain in the parent competition name"
    );
    assert!(
        !body.contains("Private Cups"),
        "sponsor section must be removed"
    );
    assert!(
        !body.contains("Education Cups"),
        "education section must be removed"
    );
    assert!(
        !body.contains("ใช้สำหรับรายการที่จัดโดยสมาคม"),
        "youth section description must be removed"
    );
    assert!(
        !body.contains("ใช้สำหรับฟุตบอลนักเรียน"),
        "school section description must be removed"
    );
    assert!(
        !body.contains("Thai School FA Cup U17"),
        "old cup name must be removed"
    );
    assert!(
        !body.contains("Thai School League Cup U18"),
        "old cup name must be removed"
    );
}

#[tokio::test]
async fn static_assets_are_served() {
    for uri in ["/static/app.css", "/static/app.js", "/static/htmx.min.js"] {
        let response = app()
            .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK, "uri {uri}");
    }
}
