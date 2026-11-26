#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::Path;

use thailand_football::data::AppData;
use thailand_football::view::{build_nav, build_page, build_search_page};

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
    assert!(cup.cup_view.warning.is_empty());
    assert!(build_page(&data, "legacy-thai-league-4", None, None, None, None, "").is_none());
}

#[test]
fn fa_cup_and_league_cup_are_complete() {
    let data = data();

    let fa_cup = data
        .seed
        .competitions
        .iter()
        .find(|competition| competition.id == "thai-fa-cup")
        .and_then(|competition| competition.seasons[0].cup.as_ref())
        .expect("FA Cup data");
    assert_eq!(fa_cup.teams_count, Some(95));
    assert_eq!(fa_cup.rounds.len(), 7);
    assert_eq!(fa_cup.ties.len(), 94);
    assert_eq!(fa_cup.ties.iter().filter(|tie| tie.extra_time).count(), 10);
    assert_eq!(
        fa_cup
            .ties
            .iter()
            .filter(|tie| tie.home_penalties.is_some())
            .count(),
        14
    );
    assert!(fa_cup.ties.iter().all(|tie| tie.winner_name.is_some()));

    let fa_page = build_page(&data, "thai-fa-cup", None, Some("rounds"), None, None, "")
        .expect("FA Cup rounds");
    assert!(fa_page.cup_view.warning.is_empty());
    assert_eq!(fa_page.cup_view.teams_count, "95");
    assert_eq!(fa_page.cup_view.rounds.len(), 7);
    assert!(
        fa_page.cup_view.rounds.iter().all(|round| round.has_ties),
        "every FA Cup round must have tie records"
    );
    let fa_ties: usize = fa_page
        .cup_view
        .rounds
        .iter()
        .map(|round| round.ties.len())
        .sum();
    assert_eq!(fa_ties, 94);

    let fa_final_round = fa_page
        .cup_view
        .rounds
        .iter()
        .find(|round| round.name == "Final")
        .expect("FA Cup final round");
    assert_eq!(fa_final_round.date_label, "31 May 2026");
    assert!(fa_final_round.note.contains("Champion: Buriram United"));
    assert!(fa_final_round.note.contains("Runner-up: PT Prachuap"));
    let fa_final = &fa_final_round.ties[0];
    assert_eq!(fa_final.home, "Buriram United");
    assert_eq!(fa_final.away, "PT Prachuap");
    assert_eq!(fa_final.score, "1 - 0");
    assert_eq!(fa_final.status, "FT (AET)");
    assert!(fa_final.winner.contains("Buriram United (final)"));

    let fa_semi = fa_page
        .cup_view
        .rounds
        .iter()
        .find(|round| round.name == "Semi-finals")
        .expect("FA Cup semi-finals");
    assert_eq!(fa_semi.date_label, "17 May 2026");
    assert_eq!(fa_semi.ties.len(), 2);

    let qualification = fa_page
        .cup_view
        .rounds
        .first()
        .expect("FA Cup qualification round");
    assert_eq!(qualification.ties.len(), 31);
    let shootout = qualification
        .ties
        .iter()
        .find(|tie| tie.home == "Prime Bangkok")
        .expect("Prime Bangkok shootout");
    assert_eq!(shootout.score, "1 - 1");
    assert_eq!(shootout.penalties, "Penalties: 5 - 3");
    assert!(shootout.winner.contains("Prime Bangkok (penalty shootout)"));

    let awarded = fa_cup
        .ties
        .iter()
        .find(|tie| tie.status == "Awarded")
        .expect("awarded FA Cup tie");
    assert_eq!(awarded.home_name, "Port FC");
    assert_eq!(awarded.away_name, "Bangkok United");
    assert_eq!((awarded.home_goals, awarded.away_goals), (Some(0), Some(3)));
    assert_eq!(awarded.winner_basis.as_deref(), Some("awarded"));

    let league_cup = data
        .seed
        .competitions
        .iter()
        .find(|competition| competition.id == "thai-league-cup")
        .and_then(|competition| competition.seasons[0].cup.as_ref())
        .expect("League Cup data");
    assert_eq!(league_cup.teams_count, Some(80));
    assert_eq!(league_cup.rounds.len(), 8);
    assert_eq!(league_cup.ties.len(), 79);
    assert_eq!(
        league_cup.ties.iter().filter(|tie| tie.extra_time).count(),
        19
    );
    assert_eq!(
        league_cup
            .ties
            .iter()
            .filter(|tie| tie.home_penalties.is_some())
            .count(),
        11
    );

    let league_page = build_page(
        &data,
        "thai-league-cup",
        None,
        Some("rounds"),
        None,
        None,
        "",
    )
    .expect("League Cup rounds");
    assert!(league_page.cup_view.warning.is_empty());
    assert_eq!(league_page.cup_view.teams_count, "80");
    assert_eq!(league_page.cup_view.rounds.len(), 8);
    assert!(
        league_page
            .cup_view
            .rounds
            .iter()
            .all(|round| round.has_ties),
        "every League Cup round must have tie records"
    );
    let league_ties: usize = league_page
        .cup_view
        .rounds
        .iter()
        .map(|round| round.ties.len())
        .sum();
    assert_eq!(league_ties, 79);

    let playoff = league_page
        .cup_view
        .rounds
        .iter()
        .find(|round| round.name == "Qualification Play-off")
        .expect("League Cup play-off");
    assert_eq!(playoff.date_label, "26 Nov – 17 Dec 2025");

    let league_final_round = league_page
        .cup_view
        .rounds
        .iter()
        .find(|round| round.name == "Final")
        .expect("League Cup final round");
    assert_eq!(league_final_round.date_label, "24 May 2026");
    assert!(
        league_final_round
            .note
            .contains("Champion: Port FC · Runner-up: BG Pathum United"),
        "{}",
        league_final_round.note
    );
    let league_final = &league_final_round.ties[0];
    assert_eq!(league_final.home, "BG Pathum United");
    assert_eq!(league_final.away, "Port FC");
    assert_eq!(league_final.score, "0 - 1");
    assert!(league_final.winner.contains("Port FC (final)"));

    let league_semi = league_page
        .cup_view
        .rounds
        .iter()
        .find(|round| round.name == "Semi-finals")
        .expect("League Cup semi-finals");
    assert_eq!(league_semi.date_label, "8 Apr 2026");
    assert!(
        league_semi
            .ties
            .iter()
            .any(|tie| tie.home == "Chonburi" && tie.score == "3 - 4" && tie.away == "Port FC")
    );
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
fn youth_league_parents_own_three_age_groups() {
    let data = data();
    let parents = [
        (
            "thai-youth-league-u18",
            "Thai Youth League 1",
            "ฟุตบอลเยาวชน สมาคมฯ ลีก 1",
        ),
        (
            "thai-youth-league-u16",
            "Thai Youth League 2",
            "ฟุตบอลเยาวชน สมาคมฯ ลีก 2",
        ),
        (
            "thai-youth-league-u14",
            "Thai Youth League 3",
            "ฟุตบอลเยาวชน สมาคมฯ ลีก 3",
        ),
    ];

    for (id, name, thai) in parents {
        let competition = data.competition(id).expect("youth league parent");
        assert_eq!(competition.name, name);
        assert_eq!(competition.age_group, None);
        assert!(
            competition.aliases.iter().any(|alias| alias == thai),
            "missing Thai label for {id}"
        );
        assert_eq!(competition.seasons.len(), 3);
        assert_eq!(
            competition
                .seasons
                .iter()
                .filter(|season| season.is_current)
                .count(),
            1
        );

        for (season, age) in competition.seasons.iter().zip(["U18", "U16", "U14"]) {
            assert_eq!(season.age_group.as_deref(), Some(age));
            assert_eq!(season.id, format!("2026-{}", age.to_lowercase()));

            let teams = build_page(&data, id, Some(&season.id), Some("teams"), None, None, "")
                .expect("youth teams page");
            assert!(!teams.teams.visible);
            assert_eq!(teams.teams.empty_title, "No verified team list yet");
            assert_eq!(teams.teams.roster_status, "unavailable");
            assert_eq!(teams.teams.roster_badge, "Roster unavailable");
            assert!(
                teams.teams.empty_message.contains(name) && teams.teams.empty_message.contains(age),
                "empty message should name the parent and age: {}",
                teams.teams.empty_message
            );

            let standings = build_page(
                &data,
                id,
                Some(&season.id),
                Some("standings"),
                None,
                None,
                "",
            )
            .expect("youth standings page");
            assert!(!standings.standings.visible);
            assert_eq!(standings.standings.empty_title, "No standings yet");
            assert_eq!(
                standings.standings.empty_message,
                "No verified records yet."
            );

            for (tab, title) in [
                ("results", "No results yet"),
                ("fixtures", "No fixtures yet"),
            ] {
                let page = build_page(&data, id, Some(&season.id), Some(tab), None, None, "")
                    .expect("youth matches page");
                assert!(!page.matches.visible);
                assert_eq!(page.matches.empty_title, title);
                assert_eq!(page.matches.empty_message, "No verified records yet.");
            }

            let scorers = build_page(&data, id, Some(&season.id), Some("scorers"), None, None, "")
                .expect("youth scorers page");
            assert!(!scorers.scorers.visible);
            assert_eq!(scorers.scorers.empty_message, "No verified records yet.");
        }

        let default_page =
            build_page(&data, id, None, Some("teams"), None, None, "").expect("default page");
        assert_eq!(default_page.season_id, "2026-u18");
    }

    let nav = build_nav(&data, None, None);
    let section = nav
        .sections
        .iter()
        .find(|section| section.title == "Youth League (FAT & SAT)")
        .expect("youth nav section");
    let groups = section
        .items
        .iter()
        .filter(|item| item.is_group)
        .collect::<Vec<_>>();
    assert_eq!(groups.len(), 3, "exactly three expandable parents");
    for (group, (id, name, thai)) in groups.iter().zip(parents) {
        assert_eq!(group.label, name);
        assert_eq!(group.subtitle, format!("({thai})"));
        let labels = group
            .children
            .iter()
            .map(|child| child.label.as_str())
            .collect::<Vec<_>>();
        assert_eq!(labels, ["U18", "U16", "U14"]);
        assert_eq!(
            group
                .children
                .iter()
                .map(|child| child.href.clone())
                .collect::<Vec<_>>(),
            ["2026-u18", "2026-u16", "2026-u14"]
                .map(|season| format!("/competitions/{id}?season={season}"))
        );
    }

    for (id, expected_name, expected_age) in [
        ("thai-youth-fa-cup-u17", "Thai Youth FA Cup (U17)", "U17"),
        (
            "thai-youth-league-cup-u18",
            "Thai Youth League Cup (U18)",
            "U18",
        ),
        (
            "thai-youth-champion-cup-u18",
            "Thai Youth Champion Cup (U18)",
            "U18",
        ),
    ] {
        let competition = data.competition(id).expect("youth cup");
        assert_eq!(competition.name, expected_name);
        assert_eq!(competition.age_group.as_deref(), Some(expected_age));
        assert_eq!(competition.seasons.len(), 1);
        assert_eq!(competition.seasons[0].id, "2026");
    }
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
fn college_cup_bracket_is_corrected() {
    let data = data();

    let teams = build_page(
        &data,
        "thai-college-cup-u19",
        None,
        Some("teams"),
        None,
        None,
        "",
    )
    .expect("college cup teams");
    assert_eq!(teams.teams.count, 8);
    assert_eq!(teams.teams.roster_status, "partial");
    assert!(teams.teams.roster_badge.contains("Partial list"));
    assert!(
        teams
            .teams
            .rows
            .iter()
            .any(|row| row.display_name.contains("Thawee SC"))
    );
    assert!(
        !teams
            .teams
            .rows
            .iter()
            .any(|row| row.display_name.to_lowercase().contains("yasothon"))
    );

    let rounds = build_page(
        &data,
        "thai-college-cup-u19",
        None,
        Some("rounds"),
        None,
        None,
        "",
    )
    .expect("college cup rounds");
    assert!(rounds.cup_view.visible);
    assert_eq!(rounds.cup_view.rounds.len(), 3);

    let quarter = rounds
        .cup_view
        .rounds
        .iter()
        .find(|round| round.name == "Quarter-finals")
        .expect("quarter-final round");
    assert_eq!(quarter.ties.len(), 4);
    assert!(
        quarter
            .ties
            .iter()
            .any(|tie| tie.score == "6 - 1" && tie.home.contains("Rajdamnern"))
    );
    assert!(
        quarter
            .ties
            .iter()
            .any(|tie| tie.score == "0 - 7" && tie.away.contains("Thawee SC"))
    );
    let chonburi_qf = quarter
        .ties
        .iter()
        .find(|tie| tie.home.contains("Chonburi"))
        .expect("Chonburi quarter-final");
    assert_eq!(chonburi_qf.score, "Score unavailable");
    assert!(chonburi_qf.winner.contains("Chonburi"));
    assert!(chonburi_qf.winner.contains("inferred from next round"));
    let lampang_qf = quarter
        .ties
        .iter()
        .find(|tie| tie.home.contains("Lampang"))
        .expect("Lampang quarter-final");
    assert_eq!(lampang_qf.score, "Score unavailable");
    assert!(lampang_qf.winner.contains("Lampang"));
    assert!(
        quarter
            .ties
            .iter()
            .all(|tie| !tie.away.contains("Yasothon"))
    );

    let semi = rounds
        .cup_view
        .rounds
        .iter()
        .find(|round| round.name == "Semi-finals")
        .expect("semi-final round");
    assert_eq!(semi.ties.len(), 2);
    let penalty = semi
        .ties
        .iter()
        .find(|tie| !tie.penalties.is_empty())
        .expect("penalty semi-final");
    assert_eq!(penalty.score, "1 - 1");
    assert_eq!(penalty.penalties, "Penalties: 4 - 5");
    assert!(penalty.winner.contains("Chonburi"));

    let final_round = rounds
        .cup_view
        .rounds
        .iter()
        .find(|round| round.name == "Final")
        .expect("final round");
    assert_eq!(final_round.ties.len(), 1);
    assert_eq!(final_round.ties[0].score, "0 - 2");
    assert!(
        final_round
            .ties
            .iter()
            .all(|tie| !tie.away.contains("Yasothon"))
    );
    assert!(final_round.note.contains("Champion: Thawee SC"));
    assert!(final_round.note.contains("alternate final claim"));
    assert!(
        final_round
            .ties
            .iter()
            .all(|tie| !tie.home.contains("Yasothon") && !tie.away.contains("Yasothon"))
    );
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
