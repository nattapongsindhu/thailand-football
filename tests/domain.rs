#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::Path;

use thailand_football::data::AppData;
use thailand_football::view::{build_page, build_search_page};

fn data() -> AppData {
    AppData::from_path(Path::new("data/seed.json")).expect("seed should load")
}

#[test]
fn seed_contains_expected_active_competitions() {
    let data = data();
    assert_eq!(data.active_competitions().count(), 17);
    assert_eq!(
        data.search_index.len(),
        data.team_registry.participations.len()
    );
    assert!(data.search_index.len() >= 69);
    assert!(data.competition("thai-league-1").is_some());
    assert!(data.competition("thai-school-league-general").is_some());
    assert!(data.competition("thai-university-cup-u23").is_some());
    assert!(
        data.competition("legacy-thai-league-4")
            .is_some_and(|c| !c.active)
    );
    assert!(
        data.competition("legacy-thai-league-5")
            .is_some_and(|c| !c.active)
    );
}

#[test]
fn thai_league_one_page_has_standings_and_no_fallback_news() {
    let data = data();
    let page = build_page(&data, "thai-league-1", None, None, None, None, "").expect("page");
    assert_eq!(page.competition.name, "Thai League 1");
    assert!(page.standings.visible);
    assert_eq!(page.standings.rows.len(), 16);
    assert_eq!(page.standings.validation_status, "ok");

    let news_page =
        build_page(&data, "thai-league-1", None, Some("news"), None, None, "").expect("news");
    assert_eq!(news_page.news_view.articles.len(), 5);

    let empty_news = build_page(
        &data,
        "thai-university-cup-u23",
        None,
        Some("news"),
        None,
        None,
        "",
    )
    .expect("empty news");
    assert!(!empty_news.news_view.visible);
    assert_eq!(empty_news.news_view.articles.len(), 0);
}

#[test]
fn thai_league_three_fails_arithmetic_and_keeps_warning() {
    let data = data();
    let page = build_page(
        &data,
        "thai-league-3",
        None,
        None,
        Some("group-1"),
        None,
        "",
    )
    .expect("page");
    assert_eq!(page.standings.group_name, "Northern Region");
    assert_eq!(page.standings.validation_status, "failed");
    assert!(
        page.data_warnings
            .iter()
            .any(|warning| warning.contains("Arithmetic validation failed"))
    );
}

#[test]
fn school_leagues_are_distinct_and_do_not_fall_back() {
    let data = data();
    let type_a = build_page(
        &data,
        "thai-school-league-type-a",
        Some("2026-u18"),
        Some("teams"),
        None,
        None,
        "",
    )
    .expect("school league type A");
    let general = build_page(
        &data,
        "thai-school-league-general",
        Some("2026-u12"),
        Some("teams"),
        None,
        None,
        "",
    )
    .expect("school league general division");
    assert!(type_a.competition.name.contains("Type A"));
    assert!(general.competition.name.contains("General"));
    assert!(type_a.competition.age_label.contains("U18"));
    assert!(general.competition.age_label.contains("U12"));
    assert!(type_a.competition.aliases.contains("ประเภท ก"));
    assert!(general.competition.aliases.contains("ประเภท ทั่วไป"));
    assert!(!type_a.standings.visible);
    assert!(type_a.competition.is_youth);
    assert_eq!(type_a.teams.count, 16);
    assert!(type_a.teams.visible);
    assert!(!general.teams.visible);
    assert_eq!(general.teams.count, 0);
}

#[test]
fn cups_and_legacy_routes_are_bounded() {
    let data = data();
    let cup = build_page(&data, "thai-fa-cup", None, Some("rounds"), None, None, "").expect("cup");
    assert!(cup.cup_view.visible);
    assert!(cup.cup_view.has_rounds);
    assert!(cup.cup_view.warning.contains("date conflict"));
    assert!(build_page(&data, "legacy-thai-league-4", None, None, None, None, "").is_none());
}

#[test]
fn search_includes_competition_and_group_context() {
    let data = data();
    let page = build_search_page(&data, "buriram");
    assert!(page.has_results);
    let united = page
        .results
        .iter()
        .find(|r| r.team_name == "Buriram United" && r.competition_name == "Thai League 1")
        .expect("Buriram United league result");
    assert_eq!(united.group_name, "Overall");
    let reserve = page
        .results
        .iter()
        .find(|r| r.team_name == "Buriram Res.")
        .expect("Buriram Res. result");
    assert_eq!(reserve.competition_name, "Thai League 3");
    assert_eq!(reserve.group_name, "Northeastern Region");
}

#[test]
fn search_uses_displayed_season_for_fallback_rosters() {
    let data = data();

    let type_a = build_search_page(&data, "Wat Thong Samrit");
    let type_a_result = type_a
        .results
        .iter()
        .find(|result| result.competition_name.contains("Type A"))
        .expect("Type A U14 search result");
    assert_eq!(type_a_result.season_label, "2024");

    let champion = build_search_page(&data, "Bangkok Christian College");
    let champion_result = champion
        .results
        .iter()
        .find(|result| result.competition_name.contains("Youth Champion"))
        .expect("Youth Champion search result");
    assert_eq!(champion_result.season_label, "2025");
}

#[test]
fn empty_youth_competitions_do_not_fall_back_to_senior_data() {
    let data = data();
    for tab in [
        "standings",
        "results",
        "fixtures",
        "stats",
        "scorers",
        "news",
    ] {
        let page = build_page(
            &data,
            "thai-university-cup-u23",
            None,
            Some(tab),
            None,
            None,
            "",
        )
        .expect("youth page");
        assert!(!page.standings.visible, "standings fallback for {tab}");
        assert!(!page.matches.visible, "matches fallback for {tab}");
        assert!(!page.stats.visible, "stats fallback for {tab}");
        assert!(!page.scorers.visible, "scorers fallback for {tab}");
        assert!(!page.news_view.visible, "news fallback for {tab}");
    }
    let cup = build_page(
        &data,
        "thai-university-cup-u23",
        None,
        Some("rounds"),
        None,
        None,
        "",
    )
    .expect("youth cup");
    assert!(!cup.cup_view.visible);
}

#[test]
fn team_registry_populates_team_first_views() {
    let data = data();
    let t1 =
        build_page(&data, "thai-league-1", None, Some("teams"), None, None, "").expect("T1 teams");
    assert_eq!(t1.teams.count, 16);
    assert_eq!(t1.teams.verified_count, 16);
    assert_eq!(t1.teams.unverified_count, 0);

    let t2 =
        build_page(&data, "thai-league-2", None, Some("teams"), None, None, "").expect("T2 teams");
    assert_eq!(t2.teams.count, 18);

    let t3 = build_page(
        &data,
        "thai-league-3",
        None,
        Some("teams"),
        Some("group-2"),
        None,
        "",
    )
    .expect("T3 teams");
    assert_eq!(t3.teams.group_name, "Northeastern Region");
    assert_eq!(t3.teams.count, 5);
    assert_eq!(t3.teams.unverified_count, 5);

    let youth = build_page(
        &data,
        "thai-youth-league-u18",
        None,
        Some("teams"),
        None,
        None,
        "",
    )
    .expect("youth teams");
    assert!(!youth.teams.visible);
    assert_eq!(youth.teams.count, 0);
}

#[test]
fn roster_fallback_metadata_is_exposed_without_standings_leakage() {
    let data = data();

    let type_a_u14 = build_page(
        &data,
        "thai-school-league-type-a",
        Some("2026-u14"),
        Some("teams"),
        None,
        None,
        "",
    )
    .expect("Type A U14 teams");
    assert_eq!(type_a_u14.teams.count, 16);
    assert_eq!(type_a_u14.teams.roster_status, "latest_verified");
    assert_eq!(type_a_u14.teams.displayed_season, "2024");
    assert_eq!(type_a_u14.teams.source_season, "2024");
    assert!(
        type_a_u14
            .teams
            .roster_badge
            .contains("Latest verified roster: 2024")
    );
    assert!(
        type_a_u14
            .teams
            .roster_meta_line
            .contains("Displayed season 2024")
    );
    assert!(
        type_a_u14
            .data_warnings
            .iter()
            .any(|warning| warning.contains("latest verified roster is shown for reference"))
    );

    let type_a_u18 = build_page(
        &data,
        "thai-school-league-type-a",
        Some("2026-u18"),
        Some("teams"),
        None,
        None,
        "",
    )
    .expect("Type A U18 teams");
    assert_eq!(type_a_u18.teams.roster_status, "current");
    assert_eq!(type_a_u18.teams.displayed_season, "2026");

    let youth_champion = build_page(
        &data,
        "thai-youth-champion-cup-u18",
        None,
        Some("teams"),
        None,
        None,
        "",
    )
    .expect("Youth Champion Cup teams");
    assert_eq!(youth_champion.teams.count, 4);
    assert_eq!(youth_champion.teams.roster_status, "latest_verified");
    assert_eq!(youth_champion.teams.displayed_season, "2025");
    assert!(
        youth_champion
            .teams
            .roster_badge
            .contains("Latest verified roster: 2025")
    );

    let youth_fa_cup = build_page(
        &data,
        "thai-youth-fa-cup-u17",
        None,
        Some("teams"),
        None,
        None,
        "",
    )
    .expect("Youth FA Cup teams");
    assert_eq!(youth_fa_cup.teams.roster_status, "partial");
    assert!(youth_fa_cup.teams.roster_badge.contains("Partial list"));

    let general = build_page(
        &data,
        "thai-school-league-general",
        Some("2026-u12"),
        Some("teams"),
        None,
        None,
        "",
    )
    .expect("General teams");
    assert_eq!(general.teams.roster_status, "unavailable");
    assert_eq!(general.teams.count, 0);
    assert!(
        general
            .data_warnings
            .iter()
            .any(|warning| warning.contains("Roster unavailable"))
    );

    // Fallback teams must never be joined into current-season tables.
    for tab in ["standings", "results", "fixtures", "stats", "scorers"] {
        let page = build_page(
            &data,
            "thai-school-league-type-a",
            Some("2026-u14"),
            Some(tab),
            None,
            None,
            "",
        )
        .expect("Type A U14 fallback tab");
        assert!(!page.standings.visible, "standings leaked for {tab}");
        assert!(!page.matches.visible, "matches leaked for {tab}");
        assert!(!page.stats.visible, "stats leaked for {tab}");
        assert!(!page.scorers.visible, "scorers leaked for {tab}");
    }

    let fallback_rounds = build_page(
        &data,
        "thai-youth-champion-cup-u18",
        None,
        Some("rounds"),
        None,
        None,
        "",
    )
    .expect("Youth Champion Cup rounds");
    assert!(!fallback_rounds.cup_view.visible);

    for (competition_id, season_id) in [
        ("thai-school-league-type-b", Some("2026-u16")),
        ("thai-youth-fa-cup-u17", None),
        ("thai-youth-league-cup-u18", None),
        ("thai-youth-champion-cup-u18", None),
        ("thai-university-cup-u23", None),
    ] {
        for tab in ["standings", "results", "fixtures", "stats", "scorers"] {
            let page = build_page(&data, competition_id, season_id, Some(tab), None, None, "")
                .expect("fallback competition tab");
            assert!(
                !page.standings.visible,
                "standings leaked for {competition_id} / {tab}"
            );
            assert!(
                !page.matches.visible,
                "matches leaked for {competition_id} / {tab}"
            );
            assert!(
                !page.stats.visible,
                "stats leaked for {competition_id} / {tab}"
            );
            assert!(
                !page.scorers.visible,
                "scorers leaked for {competition_id} / {tab}"
            );
        }
    }
}

#[test]
fn unresolved_records_are_reported_not_presented() {
    let data = data();
    let page = build_page(
        &data,
        "thai-league-1",
        None,
        Some("results"),
        None,
        None,
        "",
    )
    .expect("page");
    assert!(page.matches.unresolved_count > 0);
    assert!(
        page.data_warnings
            .iter()
            .any(|warning| warning.contains("could not be linked"))
    );
}
