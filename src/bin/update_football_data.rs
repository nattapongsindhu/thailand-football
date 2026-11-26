use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Context, Result};
use chrono::Utc;
use thailand_football::discovery::{
    CuratedTeamFile, FetchRecord, SourceRegistry, ValidationReport, apply_curated,
    bootstrap_registry, build_news_metadata, fetch_source, load_json, validate,
};
use thailand_football::model::{Seed, TeamRegistry};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "thailand_football=info".into()),
        )
        .init();

    let root = std::env::current_dir().context("cannot determine current directory")?;
    let seed_path = root.join("data/seed.json");
    let source_path = root.join("data/source_registry.json");
    let curated_path = root.join("data/discovery/curated_verified_teams.json");
    let registry_path = root.join("data/team_registry.json");

    let seed: Seed = load_json(&seed_path)?;
    let mut source_registry: SourceRegistry = load_json(&source_path)?;
    let curated: CuratedTeamFile = load_json(&curated_path)?;

    let mut registry: TeamRegistry = if registry_path.exists() {
        load_json(&registry_path).context("cannot load existing team registry")?
    } else {
        tracing::info!("bootstrapping team registry from standings snapshot");
        bootstrap_registry(&seed)
    };
    apply_curated(&mut registry, &curated);
    registry.generated_at = Utc::now().to_rfc3339();
    registry.source_registry_version = source_registry.version;

    let fetch_enabled = std::env::var("TF_DISCOVERY_NO_FETCH").ok().as_deref() != Some("1");
    let mut fetches: Vec<FetchRecord> = Vec::new();
    let mut rss_news = Vec::new();
    if fetch_enabled {
        let client = reqwest::Client::builder()
            .user_agent("ThailandFootballLocalDiscovery/0.1 (+local preview; public sources only)")
            .timeout(Duration::from_secs(25))
            .redirect(reqwest::redirect::Policy::limited(5))
            .build()
            .context("cannot build HTTP client")?;
        let now = Utc::now().to_rfc3339();

        for competition in &mut source_registry.competitions {
            let mut urls = competition.fetch_urls.clone();
            urls.extend(competition.news_feed_urls.clone());
            urls.sort();
            urls.dedup();
            if urls.is_empty() {
                continue;
            }
            let mut attempted = 0;
            let mut failed = 0;
            for url in urls {
                attempted += 1;
                let result = fetch_source(&client, competition, &url).await;
                if result.record.error.is_some() || result.record.http_status != Some(200) {
                    failed += 1;
                }
                fetches.push(result.record);
                rss_news.extend(result.rss_news);
            }
            competition.last_checked_at = Some(now.clone());
            competition.update_status = if failed == attempted {
                "checked_with_errors".to_string()
            } else {
                "checked".to_string()
            };
        }
    } else {
        tracing::warn!("network fetch disabled by TF_DISCOVERY_NO_FETCH=1");
    }

    let news = build_news_metadata(&seed, &registry, rss_news);
    let report = validate(&seed, &registry, &source_registry, &news, &fetches);

    let timestamp = Utc::now().format("%Y%m%dT%H%M%SZ").to_string();
    let inbox_dir = root.join("data/discovery/inbox").join(&timestamp);
    let reports_dir = root.join("data/discovery/reports");
    std::fs::create_dir_all(&inbox_dir)?;
    std::fs::create_dir_all(&reports_dir)?;

    std::fs::write(
        inbox_dir.join("sources.json"),
        serde_json::to_string_pretty(&fetches)?,
    )?;
    std::fs::write(
        root.join("data/team_registry.json"),
        serde_json::to_string_pretty(&registry)?,
    )?;
    std::fs::write(
        root.join("data/discovery/news_metadata.json"),
        serde_json::to_string_pretty(&news)?,
    )?;
    std::fs::write(
        root.join("data/source_registry.json"),
        serde_json::to_string_pretty(&source_registry)?,
    )?;
    std::fs::write(
        reports_dir.join("latest.json"),
        serde_json::to_string_pretty(&report)?,
    )?;
    std::fs::write(
        reports_dir.join("latest.md"),
        render_markdown(&report, &fetches),
    )?;

    tracing::info!(
        teams = report.counts.teams,
        participations = report.counts.participations,
        verified_secondary = report.counts.verified_secondary,
        discovered = report.counts.discovered_unverified,
        news = report.counts.news_items,
        candidates = report.counts.team_candidates,
        errors = report.errors.len(),
        warnings = report.warnings.len(),
        "team discovery update complete"
    );

    if !report.errors.is_empty() {
        for error in &report.errors {
            tracing::error!(%error, "validation error");
        }
        anyhow::bail!("validation failed with {} error(s)", report.errors.len());
    }

    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "team_registry": registry_path,
            "source_registry": source_path,
            "news_metadata": root.join("data/discovery/news_metadata.json"),
            "inbox": inbox_dir,
            "report": reports_dir.join("latest.json"),
            "counts": report.counts,
            "errors": report.errors.len(),
            "warnings": report.warnings.len(),
        }))?
    );
    Ok(())
}

fn render_markdown(report: &ValidationReport, fetches: &[FetchRecord]) -> String {
    let mut output = String::new();
    output.push_str("# Team Discovery Report\n\n");
    output.push_str(&format!("Generated: {}\n\n", report.generated_at));
    output.push_str("## Counts\n\n");
    output.push_str(&format!("- Teams: {}\n", report.counts.teams));
    output.push_str(&format!(
        "- Participations: {}\n",
        report.counts.participations
    ));
    output.push_str(&format!(
        "- Verified secondary: {}\n",
        report.counts.verified_secondary
    ));
    output.push_str(&format!(
        "- Verified official: {}\n",
        report.counts.verified_official
    ));
    output.push_str(&format!(
        "- Discovered unverified: {}\n",
        report.counts.discovered_unverified
    ));
    output.push_str(&format!(
        "- News metadata items: {}\n",
        report.counts.news_items
    ));
    output.push_str(&format!(
        "- Team candidates: {}\n",
        report.counts.team_candidates
    ));
    output.push_str(&format!(
        "- Sources fetched: {}; failed: {}\n\n",
        report.counts.fetched_sources, report.counts.failed_sources
    ));

    output.push_str("## Competition summary\n\n");
    output.push_str("| Competition | Group | Teams | Verified secondary | Unverified |\n");
    output.push_str("|---|---|---:|---:|---:|\n");
    for row in &report.competition_summary {
        output.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            row.display_name,
            row.organizer_group,
            row.teams,
            row.verified_secondary,
            row.discovered_unverified
        ));
    }

    if !report.warnings.is_empty() {
        output.push_str("\n## Warnings\n\n");
        for warning in &report.warnings {
            output.push_str(&format!("- {warning}\n"));
        }
    }
    if !report.errors.is_empty() {
        output.push_str("\n## Errors\n\n");
        for error in &report.errors {
            output.push_str(&format!("- {error}\n"));
        }
    }

    output.push_str("\n## Fetch results\n\n");
    let mut urls = HashSet::new();
    for fetch in fetches {
        if !urls.insert(fetch.url.clone()) {
            continue;
        }
        output.push_str(&format!(
            "- `{}` — {} — candidates {} — {}\n",
            fetch.url,
            fetch.status,
            fetch.candidate_count,
            fetch.error.clone().unwrap_or_else(|| "ok".to_string())
        ));
    }
    output
}

#[allow(dead_code)]
fn ensure_parent(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(())
}

#[allow(dead_code)]
fn pathbuf(value: &str) -> PathBuf {
    PathBuf::from(value)
}
