use std::env;
use std::path::PathBuf;

use anyhow::{Context, Result};
use sqlx::postgres::PgPoolOptions;
use thailand_football::db::{group_key, season_key};
use thailand_football::model::Seed;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().with_env_filter("info").init();

    let database_url = env::var("DATABASE_URL").context("DATABASE_URL is required")?;
    let seed_path = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("data/seed.json"));
    let raw = std::fs::read_to_string(&seed_path)
        .with_context(|| format!("cannot read {}", seed_path.display()))?;
    let seed: Seed = serde_json::from_str(&raw).context("cannot parse seed JSON")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .context("cannot connect to PostgreSQL")?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    for source in &seed.sources {
        sqlx::query(
            r#"INSERT INTO sources (id, title, url, retrieved_at, kind, note)
               VALUES ($1,$2,$3,$4,$5,$6)
               ON CONFLICT (id) DO UPDATE SET title=EXCLUDED.title, url=EXCLUDED.url,
                 retrieved_at=EXCLUDED.retrieved_at, kind=EXCLUDED.kind, note=EXCLUDED.note"#,
        )
        .bind(&source.id)
        .bind(&source.title)
        .bind(source.url.as_deref())
        .bind(&source.retrieved_at)
        .bind(&source.kind)
        .bind(&source.note)
        .execute(&pool)
        .await?;
    }

    let all_competitions = seed
        .competitions
        .iter()
        .chain(seed.legacy.iter())
        .collect::<Vec<_>>();

    for comp in &all_competitions {
        sqlx::query(
            r#"INSERT INTO competitions
               (id,name,short_name,section,category,kind,age_group,sort_order,active,official_name,group_label,description)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)
               ON CONFLICT (id) DO UPDATE SET name=EXCLUDED.name, short_name=EXCLUDED.short_name,
                 section=EXCLUDED.section, category=EXCLUDED.category, kind=EXCLUDED.kind,
                 age_group=EXCLUDED.age_group, sort_order=EXCLUDED.sort_order, active=EXCLUDED.active,
                 official_name=EXCLUDED.official_name, group_label=EXCLUDED.group_label,
                 description=EXCLUDED.description"#,
        )
        .bind(&comp.id)
        .bind(&comp.name)
        .bind(&comp.short_name)
        .bind(&comp.section)
        .bind(&comp.category)
        .bind(&comp.kind)
        .bind(comp.age_group.as_deref())
        .bind(comp.sort_order)
        .bind(comp.active)
        .bind(&comp.official_name)
        .bind(comp.group_label.as_deref())
        .bind(&comp.description)
        .execute(&pool)
        .await?;

        for alias in &comp.aliases {
            sqlx::query(
                "INSERT INTO competition_aliases (competition_id, alias) VALUES ($1,$2) ON CONFLICT DO NOTHING",
            )
            .bind(&comp.id)
            .bind(alias)
            .execute(&pool)
            .await?;
        }
        for source_id in &comp.source_ids {
            sqlx::query(
                "INSERT INTO competition_sources (competition_id, source_id) VALUES ($1,$2) ON CONFLICT DO NOTHING",
            )
            .bind(&comp.id)
            .bind(source_id)
            .execute(&pool)
            .await?;
        }

        for season in &comp.seasons {
            let skey = season_key(&comp.id, &season.id);
            sqlx::query(
                r#"INSERT INTO seasons
                   (season_key,competition_id,season_id,label,is_current,verified,captured_at,age_group)
                   VALUES ($1,$2,$3,$4,$5,$6,$7,$8)
                   ON CONFLICT (season_key) DO UPDATE SET label=EXCLUDED.label,
                     is_current=EXCLUDED.is_current, verified=EXCLUDED.verified,
                     captured_at=EXCLUDED.captured_at, age_group=EXCLUDED.age_group"#,
            )
            .bind(&skey)
            .bind(&comp.id)
            .bind(&season.id)
            .bind(&season.label)
            .bind(season.is_current)
            .bind(season.verified)
            .bind(&season.captured_at)
            .bind(season.age_group.as_deref())
            .execute(&pool)
            .await?;

            for source_id in &season.source_ids {
                sqlx::query(
                    "INSERT INTO season_sources (season_key, source_id) VALUES ($1,$2) ON CONFLICT DO NOTHING",
                )
                .bind(&skey)
                .bind(source_id)
                .execute(&pool)
                .await?;
            }

            for (group_index, group) in season.groups.iter().enumerate() {
                let gkey = group_key(&skey, &group.id);
                sqlx::query(
                    r#"INSERT INTO season_groups
                       (group_key,season_key,group_id,name,sort_order,validation_status,validation_messages)
                       VALUES ($1,$2,$3,$4,$5,$6,$7)
                       ON CONFLICT (group_key) DO UPDATE SET name=EXCLUDED.name,
                         sort_order=EXCLUDED.sort_order, validation_status=EXCLUDED.validation_status,
                         validation_messages=EXCLUDED.validation_messages"#,
                )
                .bind(&gkey)
                .bind(&skey)
                .bind(&group.id)
                .bind(&group.name)
                .bind(group_index as i32)
                .bind(&group.validation.status)
                .bind(&group.validation.messages)
                .execute(&pool)
                .await?;

                for (row_index, row) in group.standings.iter().enumerate() {
                    sqlx::query(
                        r#"INSERT INTO teams (id, canonical_name, identity_status, notes)
                           VALUES ($1,$2,'imported','Imported from owner-supplied snapshot')
                           ON CONFLICT (id) DO UPDATE SET canonical_name=EXCLUDED.canonical_name"#,
                    )
                    .bind(&row.team_id)
                    .bind(&row.team_name)
                    .execute(&pool)
                    .await?;

                    sqlx::query(
                        r#"INSERT INTO participations
                           (season_key,group_key,team_id,display_name,sort_order)
                           VALUES ($1,$2,$3,$4,$5)
                           ON CONFLICT (season_key,group_key,team_id) DO UPDATE SET
                             display_name=EXCLUDED.display_name, sort_order=EXCLUDED.sort_order"#,
                    )
                    .bind(&skey)
                    .bind(&gkey)
                    .bind(&row.team_id)
                    .bind(&row.team_name)
                    .bind(row_index as i32)
                    .execute(&pool)
                    .await?;

                    sqlx::query(
                        r#"INSERT INTO standings
                           (season_key,group_key,team_id,captured_at,rank,played,won,drawn,lost,
                            goals_for,goals_against,goal_difference,points,form,zone)
                           VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15)
                           ON CONFLICT (season_key,group_key,team_id,captured_at) DO UPDATE SET
                             rank=EXCLUDED.rank, played=EXCLUDED.played, won=EXCLUDED.won,
                             drawn=EXCLUDED.drawn, lost=EXCLUDED.lost, goals_for=EXCLUDED.goals_for,
                             goals_against=EXCLUDED.goals_against, goal_difference=EXCLUDED.goal_difference,
                             points=EXCLUDED.points, form=EXCLUDED.form, zone=EXCLUDED.zone"#,
                    )
                    .bind(&skey)
                    .bind(&gkey)
                    .bind(&row.team_id)
                    .bind(&season.captured_at)
                    .bind(row.rank)
                    .bind(row.played)
                    .bind(row.won)
                    .bind(row.drawn)
                    .bind(row.lost)
                    .bind(row.goals_for)
                    .bind(row.goals_against)
                    .bind(row.goal_difference)
                    .bind(row.points)
                    .bind(&row.form)
                    .bind(&row.zone)
                    .execute(&pool)
                    .await?;
                }
            }

            let first_group = season.groups.first().map(|g| group_key(&skey, &g.id));
            for m in &season.matches {
                sqlx::query(
                    r#"INSERT INTO matches
                       (id,season_key,group_key,kind,round_label,date_label,home_team_id,away_team_id,
                        home_name,away_name,home_goals,away_goals,status,note,resolution,issues)
                       VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16)
                       ON CONFLICT (id) DO UPDATE SET group_key=EXCLUDED.group_key, kind=EXCLUDED.kind,
                         round_label=EXCLUDED.round_label, date_label=EXCLUDED.date_label,
                         home_team_id=EXCLUDED.home_team_id, away_team_id=EXCLUDED.away_team_id,
                         home_name=EXCLUDED.home_name, away_name=EXCLUDED.away_name,
                         home_goals=EXCLUDED.home_goals, away_goals=EXCLUDED.away_goals,
                         status=EXCLUDED.status, note=EXCLUDED.note, resolution=EXCLUDED.resolution,
                         issues=EXCLUDED.issues"#,
                )
                .bind(&m.id)
                .bind(&skey)
                .bind(first_group.as_deref())
                .bind(&m.kind)
                .bind(&m.round_label)
                .bind(&m.date_label)
                .bind(m.home_team_id.as_deref())
                .bind(m.away_team_id.as_deref())
                .bind(&m.home_name)
                .bind(&m.away_name)
                .bind(m.home_goals)
                .bind(m.away_goals)
                .bind(&m.status)
                .bind(&m.note)
                .bind(&m.resolution)
                .bind(&m.issues)
                .execute(&pool)
                .await?;
            }

            if let Some(cup) = &season.cup {
                sqlx::query(
                    r#"INSERT INTO cup_editions
                       (season_key,name,sponsor,teams_count,prize,reward)
                       VALUES ($1,$2,$3,$4,$5,$6)
                       ON CONFLICT (season_key) DO UPDATE SET name=EXCLUDED.name,
                         sponsor=EXCLUDED.sponsor, teams_count=EXCLUDED.teams_count,
                         prize=EXCLUDED.prize, reward=EXCLUDED.reward"#,
                )
                .bind(&skey)
                .bind(&cup.name)
                .bind(&cup.sponsor)
                .bind(cup.teams_count)
                .bind(&cup.prize)
                .bind(&cup.reward)
                .execute(&pool)
                .await?;

                for round in &cup.rounds {
                    sqlx::query(
                        r#"INSERT INTO cup_rounds
                           (id,season_key,name,date_label,clubs,status,note,sort_order)
                           VALUES ($1,$2,$3,$4,$5,$6,$7,$8)
                           ON CONFLICT (id) DO UPDATE SET name=EXCLUDED.name,
                             date_label=EXCLUDED.date_label, clubs=EXCLUDED.clubs,
                             status=EXCLUDED.status, note=EXCLUDED.note, sort_order=EXCLUDED.sort_order"#,
                    )
                    .bind(&round.id)
                    .bind(&skey)
                    .bind(&round.name)
                    .bind(&round.date_label)
                    .bind(round.clubs)
                    .bind(&round.status)
                    .bind(&round.note)
                    .bind(round.sort_order)
                    .execute(&pool)
                    .await?;
                }
                for tie in &cup.ties {
                    sqlx::query(
                        r#"INSERT INTO cup_ties
                           (id,season_key,round_id,stage,home_name,away_name,home_goals,away_goals,status,note,sort_order,
                            score_status,home_penalties,away_penalties,winner_name,winner_basis,extra_time)
                           VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17)
                           ON CONFLICT (id) DO UPDATE SET round_id=EXCLUDED.round_id,
                             stage=EXCLUDED.stage, home_name=EXCLUDED.home_name, away_name=EXCLUDED.away_name,
                             home_goals=EXCLUDED.home_goals, away_goals=EXCLUDED.away_goals,
                             status=EXCLUDED.status, note=EXCLUDED.note, sort_order=EXCLUDED.sort_order,
                             score_status=EXCLUDED.score_status, home_penalties=EXCLUDED.home_penalties,
                             away_penalties=EXCLUDED.away_penalties, winner_name=EXCLUDED.winner_name,
                             winner_basis=EXCLUDED.winner_basis, extra_time=EXCLUDED.extra_time"#,
                    )
                    .bind(&tie.id)
                    .bind(&skey)
                    .bind(tie.round_id.as_deref())
                    .bind(&tie.stage)
                    .bind(&tie.home_name)
                    .bind(&tie.away_name)
                    .bind(tie.home_goals)
                    .bind(tie.away_goals)
                    .bind(&tie.status)
                    .bind(&tie.note)
                    .bind(tie.sort_order)
                    .bind(tie.score_status.as_deref())
                    .bind(tie.home_penalties)
                    .bind(tie.away_penalties)
                    .bind(tie.winner_name.as_deref())
                    .bind(tie.winner_basis.as_deref())
                    .bind(tie.extra_time)
                    .execute(&pool)
                    .await?;
                }
                for past in &cup.past_editions {
                    sqlx::query(
                        r#"INSERT INTO cup_past_editions
                           (id,season_key,year,home_name,away_name,home_goals,away_goals,note)
                           VALUES ($1,$2,$3,$4,$5,$6,$7,$8)
                           ON CONFLICT (id) DO UPDATE SET year=EXCLUDED.year,
                             home_name=EXCLUDED.home_name, away_name=EXCLUDED.away_name,
                             home_goals=EXCLUDED.home_goals, away_goals=EXCLUDED.away_goals,
                             note=EXCLUDED.note"#,
                    )
                    .bind(&past.id)
                    .bind(&skey)
                    .bind(&past.year)
                    .bind(&past.home_name)
                    .bind(&past.away_name)
                    .bind(past.home_goals)
                    .bind(past.away_goals)
                    .bind(&past.note)
                    .execute(&pool)
                    .await?;
                }
            }

            for scorer in &season.scorers {
                sqlx::query(
                    r#"INSERT INTO scorers
                       (id,season_key,team_id,rank,player_name,team_name,nationality,goals,appearances,assists,resolution,issues)
                       VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)
                       ON CONFLICT (id) DO UPDATE SET team_id=EXCLUDED.team_id, rank=EXCLUDED.rank,
                         player_name=EXCLUDED.player_name, team_name=EXCLUDED.team_name,
                         nationality=EXCLUDED.nationality, goals=EXCLUDED.goals,
                         appearances=EXCLUDED.appearances, assists=EXCLUDED.assists,
                         resolution=EXCLUDED.resolution, issues=EXCLUDED.issues"#,
                )
                .bind(&scorer.id)
                .bind(&skey)
                .bind(scorer.team_id.as_deref())
                .bind(scorer.rank)
                .bind(&scorer.player_name)
                .bind(&scorer.team_name)
                .bind(&scorer.nationality)
                .bind(scorer.goals)
                .bind(scorer.appearances)
                .bind(scorer.assists)
                .bind(&scorer.resolution)
                .bind(&scorer.issues)
                .execute(&pool)
                .await?;
            }
        }
    }

    for article in &seed.news {
        sqlx::query(
            r#"INSERT INTO news (id,title,summary,published_label,source_name,tag,emoji)
               VALUES ($1,$2,$3,$4,$5,$6,$7)
               ON CONFLICT (id) DO UPDATE SET title=EXCLUDED.title, summary=EXCLUDED.summary,
                 published_label=EXCLUDED.published_label, source_name=EXCLUDED.source_name,
                 tag=EXCLUDED.tag, emoji=EXCLUDED.emoji"#,
        )
        .bind(&article.id)
        .bind(&article.title)
        .bind(&article.summary)
        .bind(&article.published_label)
        .bind(&article.source_name)
        .bind(&article.tag)
        .bind(&article.emoji)
        .execute(&pool)
        .await?;
        for competition_id in &article.competition_ids {
            sqlx::query(
                "INSERT INTO news_competitions (news_id, competition_id) VALUES ($1,$2) ON CONFLICT DO NOTHING",
            )
            .bind(&article.id)
            .bind(competition_id)
            .execute(&pool)
            .await?;
        }
    }

    let meta_pairs = [
        ("app", seed.meta.app.clone()),
        ("mode", "postgresql".to_string()),
        ("generated_at", seed.meta.generated_at.clone()),
        ("legacy_source", seed.meta.legacy_source.clone()),
        ("legacy_sha256", seed.meta.legacy_sha256.clone()),
        ("notes_json", serde_json::to_string(&seed.meta.notes)?),
        ("nav_json", serde_json::to_string(&seed.nav)?),
    ];
    for (key, value) in meta_pairs {
        sqlx::query(
            "INSERT INTO app_meta (key, value) VALUES ($1,$2) ON CONFLICT (key) DO UPDATE SET value=EXCLUDED.value",
        )
        .bind(key)
        .bind(value)
        .execute(&pool)
        .await?;
    }

    tracing::info!(
        competitions = all_competitions.len(),
        "seed import complete"
    );
    Ok(())
}
