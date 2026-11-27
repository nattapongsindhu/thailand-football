use std::collections::HashMap;

use anyhow::{Context, Result};
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};

use crate::data::AppData;
use crate::model::{
    Competition, Cup, CupRound, CupTie, Group, Match, Meta, NavSection, NewsArticle, PastEdition,
    Scorer, Season, Seed, Source, Standing, Validation,
};

pub fn season_key(competition_id: &str, season_id: &str) -> String {
    format!("{competition_id}:{season_id}")
}

pub fn group_key(season_key: &str, group_id: &str) -> String {
    format!("{season_key}:{group_id}")
}

pub async fn load_from_database(database_url: &str) -> Result<AppData> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(20))
        .connect(database_url)
        .await
        .context("cannot connect to PostgreSQL")?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let sources = load_sources(&pool).await?;
    let aliases = load_competition_aliases(&pool).await?;
    let comp_sources = load_competition_sources(&pool).await?;
    let news = load_news(&pool).await?;
    let news_by_competition = load_news_competition_ids(&pool).await?;

    let rows = sqlx::query(
        r#"SELECT id,name,short_name,section,category,kind,age_group,sort_order,active,
                  official_name,group_label,description
           FROM competitions ORDER BY sort_order, id"#,
    )
    .fetch_all(&pool)
    .await?;

    let mut active = Vec::new();
    let mut legacy = Vec::new();
    for row in rows {
        let id: String = row.try_get("id")?;
        let seasons = load_seasons(
            &pool,
            &id,
            news_by_competition.get(&id).cloned().unwrap_or_default(),
        )
        .await?;
        let competition = Competition {
            id: id.clone(),
            name: row.try_get("name")?,
            short_name: row.try_get("short_name")?,
            section: row.try_get("section")?,
            category: row.try_get("category")?,
            kind: row.try_get("kind")?,
            age_group: row.try_get("age_group")?,
            sort_order: row.try_get("sort_order")?,
            active: row.try_get("active")?,
            official_name: row.try_get("official_name")?,
            aliases: aliases.get(&id).cloned().unwrap_or_default(),
            group_label: row.try_get("group_label")?,
            description: row.try_get("description")?,
            source_ids: comp_sources.get(&id).cloned().unwrap_or_default(),
            seasons,
        };
        if competition.active {
            active.push(competition);
        } else {
            legacy.push(competition);
        }
    }

    let meta = load_meta(&pool).await?;
    let nav_json: Option<String> = meta.get("nav_json").cloned();
    let nav: Vec<NavSection> = match nav_json {
        Some(json) => serde_json::from_str(&json).context("cannot parse nav_json from app_meta")?,
        None => Vec::new(),
    };
    let notes: Vec<String> = meta
        .get("notes_json")
        .and_then(|raw| serde_json::from_str(raw).ok())
        .unwrap_or_default();
    let meta_model = Meta {
        app: meta
            .get("app")
            .cloned()
            .unwrap_or_else(|| "Thailand Football".to_string()),
        mode: meta
            .get("mode")
            .cloned()
            .unwrap_or_else(|| "postgresql".to_string()),
        generated_at: meta.get("generated_at").cloned().unwrap_or_default(),
        legacy_source: meta.get("legacy_source").cloned().unwrap_or_default(),
        legacy_sha256: meta.get("legacy_sha256").cloned().unwrap_or_default(),
        notes,
    };

    let seed = Seed {
        meta: meta_model,
        sources,
        nav,
        competitions: active,
        news,
        legacy,
    };
    Ok(AppData::from_seed(seed))
}

async fn load_sources(pool: &PgPool) -> Result<Vec<Source>> {
    let rows = sqlx::query("SELECT id,title,url,retrieved_at,kind,note FROM sources ORDER BY id")
        .fetch_all(pool)
        .await?;
    rows.into_iter()
        .map(|row| {
            Ok(Source {
                id: row.try_get("id")?,
                title: row.try_get("title")?,
                url: row.try_get("url")?,
                retrieved_at: row.try_get("retrieved_at")?,
                kind: row.try_get("kind")?,
                note: row.try_get("note")?,
            })
        })
        .collect()
}

async fn load_competition_aliases(pool: &PgPool) -> Result<HashMap<String, Vec<String>>> {
    let rows = sqlx::query("SELECT competition_id, alias FROM competition_aliases ORDER BY alias")
        .fetch_all(pool)
        .await?;
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for row in rows {
        map.entry(row.try_get("competition_id")?)
            .or_default()
            .push(row.try_get("alias")?);
    }
    Ok(map)
}

async fn load_competition_sources(pool: &PgPool) -> Result<HashMap<String, Vec<String>>> {
    let rows = sqlx::query("SELECT competition_id, source_id FROM competition_sources")
        .fetch_all(pool)
        .await?;
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for row in rows {
        map.entry(row.try_get("competition_id")?)
            .or_default()
            .push(row.try_get("source_id")?);
    }
    Ok(map)
}

async fn load_news(pool: &PgPool) -> Result<Vec<NewsArticle>> {
    let rows = sqlx::query(
        "SELECT id,title,summary,published_label,source_name,tag,emoji FROM news ORDER BY id",
    )
    .fetch_all(pool)
    .await?;
    let mut articles = Vec::new();
    for row in rows {
        let id: String = row.try_get("id")?;
        let competition_ids = sqlx::query(
            "SELECT competition_id FROM news_competitions WHERE news_id=$1 ORDER BY competition_id",
        )
        .bind(&id)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|r| r.try_get("competition_id"))
        .collect::<Result<Vec<String>, _>>()?;
        articles.push(NewsArticle {
            id,
            title: row.try_get("title")?,
            summary: row.try_get("summary")?,
            published_label: row.try_get("published_label")?,
            source_name: row.try_get("source_name")?,
            tag: row.try_get("tag")?,
            emoji: row.try_get("emoji")?,
            competition_ids,
        });
    }
    Ok(articles)
}

async fn load_news_competition_ids(pool: &PgPool) -> Result<HashMap<String, Vec<String>>> {
    let rows = sqlx::query("SELECT competition_id, news_id FROM news_competitions")
        .fetch_all(pool)
        .await?;
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for row in rows {
        map.entry(row.try_get("competition_id")?)
            .or_default()
            .push(row.try_get("news_id")?);
    }
    Ok(map)
}

async fn load_seasons(
    pool: &PgPool,
    competition_id: &str,
    news_ids: Vec<String>,
) -> Result<Vec<Season>> {
    let rows = sqlx::query(
        r#"SELECT season_key,season_id,label,is_current,verified,captured_at,age_group
           FROM seasons WHERE competition_id=$1 ORDER BY is_current DESC, season_id DESC"#,
    )
    .bind(competition_id)
    .fetch_all(pool)
    .await?;

    let mut seasons = Vec::new();
    for row in rows {
        let skey: String = row.try_get("season_key")?;
        let source_ids = sqlx::query("SELECT source_id FROM season_sources WHERE season_key=$1")
            .bind(&skey)
            .fetch_all(pool)
            .await?
            .into_iter()
            .map(|r| r.try_get("source_id"))
            .collect::<Result<Vec<String>, _>>()?;
        seasons.push(Season {
            id: row.try_get("season_id")?,
            label: row.try_get("label")?,
            is_current: row.try_get("is_current")?,
            verified: row.try_get("verified")?,
            captured_at: row.try_get("captured_at")?,
            age_group: row.try_get("age_group")?,
            source_ids,
            groups: load_groups(pool, &skey).await?,
            matches: load_matches(pool, &skey).await?,
            cup: load_cup(pool, &skey).await?,
            scorers: load_scorers(pool, &skey).await?,
            news_ids: news_ids.clone(),
        });
    }
    Ok(seasons)
}

async fn load_groups(pool: &PgPool, season_key: &str) -> Result<Vec<Group>> {
    let rows = sqlx::query(
        r#"SELECT group_key,group_id,name,validation_status,validation_messages
           FROM season_groups WHERE season_key=$1 ORDER BY sort_order, group_id"#,
    )
    .bind(season_key)
    .fetch_all(pool)
    .await?;
    let mut groups = Vec::new();
    for row in rows {
        let gkey: String = row.try_get("group_key")?;
        let standings = sqlx::query(
            r#"SELECT s.team_id,t.canonical_name,s.rank,s.played,s.won,s.drawn,s.lost,
                      s.goals_for,s.goals_against,s.goal_difference,s.points,s.form,s.zone
               FROM standings s JOIN teams t ON t.id=s.team_id
               WHERE s.group_key=$1 ORDER BY s.rank"#,
        )
        .bind(&gkey)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|r| {
            Ok(Standing {
                team_id: r.try_get("team_id")?,
                team_name: r.try_get("canonical_name")?,
                rank: r.try_get("rank")?,
                played: r.try_get("played")?,
                won: r.try_get("won")?,
                drawn: r.try_get("drawn")?,
                lost: r.try_get("lost")?,
                goals_for: r.try_get("goals_for")?,
                goals_against: r.try_get("goals_against")?,
                goal_difference: r.try_get("goal_difference")?,
                points: r.try_get("points")?,
                form: r.try_get("form")?,
                zone: r.try_get("zone")?,
            })
        })
        .collect::<Result<Vec<_>, sqlx::Error>>()?;
        groups.push(Group {
            id: row.try_get("group_id")?,
            name: row.try_get("name")?,
            standings,
            validation: Validation {
                status: row.try_get("validation_status")?,
                messages: row.try_get("validation_messages")?,
            },
        });
    }
    Ok(groups)
}

async fn load_matches(pool: &PgPool, season_key: &str) -> Result<Vec<Match>> {
    let rows = sqlx::query(
        r#"SELECT id,kind,round_label,date_label,home_team_id,away_team_id,home_name,away_name,
                  home_goals,away_goals,status,note,resolution,issues
           FROM matches WHERE season_key=$1 ORDER BY date_label, round_label"#,
    )
    .bind(season_key)
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(|r| {
            Ok(Match {
                id: r.try_get("id")?,
                kind: r.try_get("kind")?,
                round_label: r.try_get("round_label")?,
                date_label: r.try_get("date_label")?,
                home_team_id: r.try_get("home_team_id")?,
                away_team_id: r.try_get("away_team_id")?,
                home_name: r.try_get("home_name")?,
                away_name: r.try_get("away_name")?,
                home_goals: r.try_get("home_goals")?,
                away_goals: r.try_get("away_goals")?,
                status: r.try_get("status")?,
                note: r.try_get("note")?,
                resolution: r.try_get("resolution")?,
                issues: r.try_get("issues")?,
            })
        })
        .collect()
}

async fn load_cup(pool: &PgPool, season_key: &str) -> Result<Option<Cup>> {
    let Some(row) = sqlx::query(
        "SELECT name,sponsor,teams_count,prize,reward FROM cup_editions WHERE season_key=$1",
    )
    .bind(season_key)
    .fetch_optional(pool)
    .await?
    else {
        return Ok(None);
    };
    let rounds = sqlx::query(
        "SELECT id,name,date_label,clubs,status,note,sort_order FROM cup_rounds WHERE season_key=$1 ORDER BY sort_order",
    )
    .bind(season_key)
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|r| {
        Ok(CupRound {
            id: r.try_get("id")?,
            name: r.try_get("name")?,
            date_label: r.try_get("date_label")?,
            clubs: r.try_get("clubs")?,
            status: r.try_get("status")?,
            note: r.try_get("note")?,
            sort_order: r.try_get("sort_order")?,
        })
    })
    .collect::<Result<Vec<_>, sqlx::Error>>()?;
    let ties = sqlx::query(
        "SELECT id,round_id,stage,home_name,away_name,home_goals,away_goals,status,note,sort_order,
                score_status,home_penalties,away_penalties,winner_name,winner_basis,extra_time
         FROM cup_ties WHERE season_key=$1 ORDER BY sort_order",
    )
    .bind(season_key)
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|r| {
        Ok(CupTie {
            id: r.try_get("id")?,
            round_id: r.try_get("round_id")?,
            stage: r.try_get("stage")?,
            home_name: r.try_get("home_name")?,
            away_name: r.try_get("away_name")?,
            home_goals: r.try_get("home_goals")?,
            away_goals: r.try_get("away_goals")?,
            status: r.try_get("status")?,
            note: r.try_get("note")?,
            sort_order: r.try_get("sort_order")?,
            score_status: r.try_get("score_status")?,
            home_penalties: r.try_get("home_penalties")?,
            away_penalties: r.try_get("away_penalties")?,
            winner_name: r.try_get("winner_name")?,
            winner_basis: r.try_get("winner_basis")?,
            extra_time: r.try_get("extra_time")?,
        })
    })
    .collect::<Result<Vec<_>, sqlx::Error>>()?;
    let past_editions = sqlx::query(
        "SELECT id,year,home_name,away_name,home_goals,away_goals,note FROM cup_past_editions WHERE season_key=$1 ORDER BY year DESC",
    )
    .bind(season_key)
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|r| {
        Ok(PastEdition {
            id: r.try_get("id")?,
            year: r.try_get("year")?,
            home_name: r.try_get("home_name")?,
            away_name: r.try_get("away_name")?,
            home_goals: r.try_get("home_goals")?,
            away_goals: r.try_get("away_goals")?,
            note: r.try_get("note")?,
        })
    })
    .collect::<Result<Vec<_>, sqlx::Error>>()?;
    Ok(Some(Cup {
        name: row.try_get("name")?,
        sponsor: row.try_get("sponsor")?,
        teams_count: row.try_get("teams_count")?,
        prize: row.try_get("prize")?,
        reward: row.try_get("reward")?,
        rounds,
        ties,
        past_editions,
    }))
}

async fn load_scorers(pool: &PgPool, season_key: &str) -> Result<Vec<Scorer>> {
    let rows = sqlx::query(
        r#"SELECT id,team_id,rank,player_name,team_name,nationality,goals,appearances,assists,resolution,issues
           FROM scorers WHERE season_key=$1 ORDER BY rank"#,
    )
    .bind(season_key)
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(|r| {
            Ok(Scorer {
                id: r.try_get("id")?,
                rank: r.try_get("rank")?,
                player_name: r.try_get("player_name")?,
                team_id: r.try_get("team_id")?,
                team_name: r.try_get("team_name")?,
                nationality: r.try_get("nationality")?,
                goals: r.try_get("goals")?,
                appearances: r.try_get("appearances")?,
                assists: r.try_get("assists")?,
                resolution: r.try_get("resolution")?,
                issues: r.try_get("issues")?,
            })
        })
        .collect()
}

async fn load_meta(pool: &PgPool) -> Result<HashMap<String, String>> {
    let rows = sqlx::query("SELECT key,value FROM app_meta")
        .fetch_all(pool)
        .await?;
    let mut map = HashMap::new();
    for row in rows {
        map.insert(row.try_get("key")?, row.try_get("value")?);
    }
    Ok(map)
}
