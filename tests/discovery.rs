#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::Path;

use thailand_football::discovery::{SourceRegistry, load_json, validate};
use thailand_football::model::{DiscoveryNews, Seed, TeamRegistry};

#[test]
fn discovery_data_is_valid_and_bounded() {
    let seed: Seed = load_json(Path::new("data/seed.json")).expect("seed");
    let registry: TeamRegistry =
        load_json(Path::new("data/team_registry.json")).expect("team registry");
    let sources: SourceRegistry =
        load_json(Path::new("data/source_registry.json")).expect("source registry");
    let news: Vec<DiscoveryNews> =
        load_json(Path::new("data/discovery/news_metadata.json")).expect("news metadata");

    assert!(registry.teams.len() >= 69);
    assert!(registry.participations.len() >= 69);

    let verified_secondary = registry
        .participations
        .iter()
        .filter(|participation| participation.verification_status == "verified_secondary")
        .count();
    let discovered_unverified = registry
        .participations
        .iter()
        .filter(|participation| participation.verification_status == "discovered_unverified")
        .count();
    assert!(verified_secondary >= 34);
    assert!(discovered_unverified >= 35);
    assert_eq!(
        verified_secondary + discovered_unverified,
        registry.participations.len()
    );

    for (competition_id, minimum) in [
        ("thai-league-1", 16),
        ("thai-league-2", 18),
        ("thai-league-3", 35),
        ("thai-fa-cup", 95),
        ("thai-league-cup", 80),
        ("thai-champion-cup", 2),
        ("thai-school-league-type-a", 48),
        ("thai-school-league-type-b", 32),
        ("thai-youth-fa-cup-u17", 12),
        ("thai-youth-league-cup-u18", 8),
        ("thai-youth-champion-cup-u18", 4),
        ("thai-university-cup-u23", 7),
    ] {
        let count = registry
            .participations
            .iter()
            .filter(|participation| participation.competition_id == competition_id)
            .count();
        assert!(
            count >= minimum,
            "{competition_id} has {count} participations, expected at least {minimum}"
        );
    }

    assert_eq!(registry.rosters.len(), 18);
    for roster in &registry.rosters {
        assert!(
            ["current", "latest_verified", "partial", "unavailable"]
                .contains(&roster.roster_status.as_str()),
            "invalid roster status {}",
            roster.roster_status
        );
        let count = registry
            .participations
            .iter()
            .filter(|participation| {
                participation.competition_id == roster.competition_id
                    && participation.season_id == roster.season_id
            })
            .count();
        assert_eq!(roster.team_count, count);
        assert!(!roster.displayed_season.is_empty());
        assert!(!roster.source_season.is_empty());
        assert!(!roster.source_checked_at.is_empty());
    }

    assert_eq!(sources.competitions.len(), 17);
    assert!(news.len() >= 13);

    let report = validate(&seed, &registry, &sources, &news, &[]);
    assert!(report.errors.is_empty(), "{:?}", report.errors);
    assert_eq!(report.counts.verified_official, 0);
    assert_eq!(report.counts.teams, registry.teams.len());
    assert_eq!(report.counts.participations, registry.participations.len());
    assert_eq!(report.counts.verified_secondary, verified_secondary);
    assert_eq!(report.counts.discovered_unverified, discovered_unverified);
}

#[test]
fn legacy_and_super_cup_are_inactive() {
    let registry: TeamRegistry =
        load_json(Path::new("data/team_registry.json")).expect("team registry");
    assert!(
        !registry
            .participations
            .iter()
            .any(|participation| participation.competition_id.starts_with("legacy-"))
    );
    assert!(
        !registry
            .participations
            .iter()
            .any(|participation| participation.competition_id.contains("super-cup"))
    );
}
