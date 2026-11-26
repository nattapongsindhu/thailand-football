use std::collections::HashMap;
use std::env;
use std::path::Path;

use anyhow::{Context, Result};
use sqlx::postgres::PgPoolOptions;
use thailand_football::discovery::load_json;
use thailand_football::model::{DiscoveryNews, TeamRegistry};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().with_env_filter("info").init();
    let database_url = env::var("DATABASE_URL").context("DATABASE_URL is required")?;
    let registry: TeamRegistry =
        load_json(Path::new("data/team_registry.json")).context("team registry")?;
    let news: Vec<DiscoveryNews> =
        load_json(Path::new("data/discovery/news_metadata.json")).context("news metadata")?;
    let team_names: HashMap<&str, &str> = registry
        .teams
        .iter()
        .map(|team| (team.team_id.as_str(), team.display_name.as_str()))
        .collect();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .context("cannot connect to PostgreSQL")?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    for team in &registry.teams {
        sqlx::query(
            r#"INSERT INTO teams
               (id, canonical_name, identity_status, notes, thai_name, english_name,
                organization_type, province, first_seen_at, last_seen_at)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)
               ON CONFLICT (id) DO UPDATE SET canonical_name=EXCLUDED.canonical_name,
                 identity_status=EXCLUDED.identity_status, notes=EXCLUDED.notes,
                 thai_name=EXCLUDED.thai_name, english_name=EXCLUDED.english_name,
                 organization_type=EXCLUDED.organization_type, province=EXCLUDED.province,
                 last_seen_at=EXCLUDED.last_seen_at"#,
        )
        .bind(&team.team_id)
        .bind(&team.display_name)
        .bind(&team.verification_status)
        .bind(&team.notes)
        .bind(team.thai_name.as_deref())
        .bind(team.english_name.as_deref())
        .bind(&team.organization_type)
        .bind(team.province.as_deref())
        .bind(&team.first_seen_at)
        .bind(&team.last_seen_at)
        .execute(&pool)
        .await?;
        for alias in &team.aliases {
            sqlx::query(
                "INSERT INTO team_aliases (team_id, alias, verified) VALUES ($1,$2,$3) ON CONFLICT DO NOTHING",
            )
            .bind(&team.team_id)
            .bind(alias)
            .bind(team.verification_status.starts_with("verified"))
            .execute(&pool)
            .await?;
        }
    }

    for participation in &registry.participations {
        let season_key = format!(
            "{}:{}",
            participation.competition_id, participation.season_id
        );
        let group_id = participation.group_id.as_deref().unwrap_or("overall");
        let group_key = format!("{season_key}:{group_id}");
        let group_name = if group_id == "overall" {
            "Overall".to_string()
        } else {
            group_id.to_string()
        };
        sqlx::query(
            r#"INSERT INTO season_groups
               (group_key, season_key, group_id, name, sort_order, validation_status, validation_messages)
               VALUES ($1,$2,$3,$4,0,'unknown','{}')
               ON CONFLICT DO NOTHING"#,
        )
        .bind(&group_key)
        .bind(&season_key)
        .bind(group_id)
        .bind(&group_name)
        .execute(&pool)
        .await?;
        let display_name = team_names
            .get(participation.team_id.as_str())
            .copied()
            .unwrap_or(participation.team_id.as_str());
        sqlx::query(
            r#"INSERT INTO participations
               (season_key, group_key, team_id, display_name, sort_order, age_group,
                division_type, source_url, source_title, source_publisher, source_date,
                verification_status, first_seen_at, last_seen_at, notes)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15)
               ON CONFLICT (season_key, group_key, team_id) DO UPDATE SET
                 display_name=EXCLUDED.display_name, age_group=EXCLUDED.age_group,
                 division_type=EXCLUDED.division_type, source_url=EXCLUDED.source_url,
                 source_title=EXCLUDED.source_title, source_publisher=EXCLUDED.source_publisher,
                 source_date=EXCLUDED.source_date, verification_status=EXCLUDED.verification_status,
                 last_seen_at=EXCLUDED.last_seen_at, notes=EXCLUDED.notes"#,
        )
        .bind(&season_key)
        .bind(&group_key)
        .bind(&participation.team_id)
        .bind(display_name)
        .bind(0_i32)
        .bind(participation.age_group.as_deref())
        .bind(participation.division_type.as_deref())
        .bind(participation.source_url.as_deref())
        .bind(participation.source_title.as_deref())
        .bind(participation.source_publisher.as_deref())
        .bind(participation.source_date.as_deref())
        .bind(&participation.verification_status)
        .bind(&participation.first_seen_at)
        .bind(&participation.last_seen_at)
        .bind(&participation.notes)
        .execute(&pool)
        .await?;
    }

    for item in &news {
        let season_key = item
            .competition_id
            .as_ref()
            .zip(item.season_id.as_ref())
            .map(|(competition, season)| format!("{competition}:{season}"));
        sqlx::query(
            r#"INSERT INTO discovery_news
               (news_id,title,source_name,source_url,published_at,competition_id,season_key,
                age_group,extracted_team_ids,verification_status,summary,discovered_at)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)
               ON CONFLICT (news_id) DO UPDATE SET title=EXCLUDED.title,
                 source_name=EXCLUDED.source_name, source_url=EXCLUDED.source_url,
                 published_at=EXCLUDED.published_at, competition_id=EXCLUDED.competition_id,
                 season_key=EXCLUDED.season_key, age_group=EXCLUDED.age_group,
                 extracted_team_ids=EXCLUDED.extracted_team_ids,
                 verification_status=EXCLUDED.verification_status, summary=EXCLUDED.summary"#,
        )
        .bind(&item.news_id)
        .bind(&item.title)
        .bind(&item.source_name)
        .bind(item.source_url.as_deref())
        .bind(&item.published_at)
        .bind(item.competition_id.as_deref())
        .bind(season_key.as_deref())
        .bind(item.age_group.as_deref())
        .bind(&item.extracted_team_ids)
        .bind(&item.verification_status)
        .bind(&item.summary)
        .bind(&item.discovered_at)
        .execute(&pool)
        .await?;
    }

    tracing::info!(
        teams = registry.teams.len(),
        participations = registry.participations.len(),
        news = news.len(),
        "team registry import complete"
    );
    Ok(())
}
