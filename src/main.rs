use std::sync::Arc;
use std::time::Instant;

use anyhow::{Context, Result};
use axum::http::{HeaderName, HeaderValue, header};
use thailand_football::data::AppData;
use thailand_football::db;
use thailand_football::discovery::load_json;
use thailand_football::model::TeamRegistry;
use thailand_football::routes::{AppState, router};
use tokio::net::TcpListener;
use tower_http::compression::CompressionLayer;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "thailand_football=info,tower_http=info".into()),
        )
        .init();

    let data = if let Ok(database_url) = std::env::var("DATABASE_URL") {
        if database_url.trim().is_empty() {
            load_preview().await?
        } else {
            tracing::info!("DATABASE_URL found; loading PostgreSQL mode");
            let database_data = db::load_from_database(&database_url)
                .await
                .context("failed to load PostgreSQL mode")?;
            let team_registry = load_team_registry()?;
            AppData::from_seed_with_registry(database_data.seed, team_registry)
        }
    } else {
        load_preview().await?
    };

    let data_mode = data.seed.meta.mode.clone();
    let competitions = data.active_competitions().count();
    let news = data.seed.news.len();
    let state = AppState {
        data: Arc::new(data),
        started_at: Instant::now(),
    };

    let app = router(state)
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(SetResponseHeaderLayer::overriding(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::REFERRER_POLICY,
            HeaderValue::from_static("strict-origin-when-cross-origin"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("permissions-policy"),
            HeaderValue::from_static("camera=(), microphone=(), geolocation=()"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("content-security-policy"),
            HeaderValue::from_static(
                "default-src 'self'; img-src 'self' data:; style-src 'self' 'unsafe-inline'; script-src 'self' 'unsafe-inline'; connect-src 'self'; frame-ancestors 'none'; base-uri 'self'; form-action 'self'",
            ),
        ));

    let requested_port = match std::env::var("PORT") {
        Ok(value) if !value.trim().is_empty() => Some(
            value
                .trim()
                .parse::<u16>()
                .context("PORT must be a valid TCP port")?,
        ),
        _ => None,
    };
    let listener = if let Some(port) = requested_port {
        let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        TcpListener::bind((host.as_str(), port))
            .await
            .with_context(|| format!("cannot bind {host}:{port}"))?
    } else {
        let mut bound = None;
        for port in 8080..8090 {
            match TcpListener::bind(("127.0.0.1", port)).await {
                Ok(listener) => {
                    bound = Some(listener);
                    break;
                }
                Err(error) => {
                    tracing::warn!(port, %error, "local port unavailable; trying next port");
                }
            }
        }
        let Some(listener) = bound else {
            anyhow::bail!("no free local port in range 8080-8089");
        };
        listener
    };
    let bound_address = listener
        .local_addr()
        .context("cannot read bound listener address")?;

    tracing::info!(
        url = %format!("http://{bound_address}"),
        mode = %data_mode,
        competitions,
        news,
        "Thailand Football server started"
    );

    axum::serve(listener, app).await?;
    Ok(())
}

async fn load_preview() -> Result<AppData> {
    let path = std::env::var("TF_SEED_PATH").unwrap_or_else(|_| "data/seed.json".to_string());
    tracing::info!(%path, "loading local preview seed");
    let data = AppData::from_path(std::path::Path::new(&path))?;
    tracing::info!(
        competitions = data.active_competitions().count(),
        teams = data.search_index.len(),
        "local preview data loaded"
    );
    Ok(data)
}

fn load_team_registry() -> Result<TeamRegistry> {
    let path =
        std::env::var("TF_TEAM_REGISTRY").unwrap_or_else(|_| "data/team_registry.json".to_string());
    let path = std::path::Path::new(&path);
    if path.exists() {
        load_json(path)
    } else {
        Ok(TeamRegistry::default())
    }
}
