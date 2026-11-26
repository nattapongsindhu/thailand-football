use askama::Template;

use crate::data::{AppData, SearchEntry};
use crate::model::{Competition, Match, NewsArticle, Season, Source, Standing};

#[derive(Debug, Clone)]
pub struct NavView {
    pub sections: Vec<NavSectionView>,
}

#[derive(Debug, Clone)]
pub struct NavSectionView {
    pub title: String,
    pub subtitle: String,
    pub is_bare: bool,
    pub items: Vec<NavItemView>,
}

#[derive(Debug, Clone)]
pub struct NavItemView {
    pub label: String,
    pub subtitle: String,
    pub href: String,
    pub active: bool,
    pub open: bool,
    pub is_group: bool,
    pub children: Vec<NavItemView>,
}

#[derive(Debug, Clone)]
pub struct CompetitionHeaderView {
    pub id: String,
    pub name: String,
    pub short_name: String,
    pub section: String,
    pub category: String,
    pub kind: String,
    pub age_label: String,
    pub group_label: String,
    pub description: String,
    pub official_name: String,
    pub aliases: String,
    pub is_cup: bool,
    pub is_youth: bool,
    pub verified_label: String,
    pub verified_class: String,
}

#[derive(Debug, Clone)]
pub struct SeasonOptionView {
    pub id: String,
    pub label: String,
    pub selected: bool,
}

#[derive(Debug, Clone)]
pub struct TabView {
    pub id: String,
    pub label: String,
    pub href: String,
    pub active: bool,
}

#[derive(Debug, Clone, Default)]
pub struct GroupLinkView {
    pub id: String,
    pub name: String,
    pub href: String,
    pub active: bool,
}

#[derive(Debug, Clone, Default)]
pub struct StandingRowView {
    pub team_id: String,
    pub rank: i32,
    pub team_name: String,
    pub played: i32,
    pub won: i32,
    pub drawn: i32,
    pub lost: i32,
    pub goals_for: i32,
    pub goals_against: i32,
    pub goal_difference: String,
    pub points: i32,
    pub form: Vec<String>,
    pub zone: String,
    pub highlight: bool,
}

#[derive(Debug, Clone, Default)]
pub struct StandingsView {
    pub visible: bool,
    pub group_name: String,
    pub show_group_nav: bool,
    pub groups: Vec<GroupLinkView>,
    pub rows: Vec<StandingRowView>,
    pub validation_status: String,
    pub validation_messages: Vec<String>,
    pub empty_title: String,
    pub empty_message: String,
}

#[derive(Debug, Clone, Default)]
pub struct MatchRowView {
    pub home: String,
    pub away: String,
    pub score: String,
    pub status: String,
    pub note: String,
}

#[derive(Debug, Clone, Default)]
pub struct MatchGroupView {
    pub label: String,
    pub matches: Vec<MatchRowView>,
}

#[derive(Debug, Clone, Default)]
pub struct MatchesView {
    pub visible: bool,
    pub title: String,
    pub subtitle: String,
    pub is_fixture: bool,
    pub groups: Vec<MatchGroupView>,
    pub unresolved_count: usize,
    pub empty_title: String,
    pub empty_message: String,
}

#[derive(Debug, Clone, Default)]
pub struct CupTieView {
    pub home: String,
    pub away: String,
    pub score: String,
    pub status: String,
    pub note: String,
    pub penalties: String,
    pub winner: String,
}

#[derive(Debug, Clone, Default)]
pub struct CupRoundView {
    pub name: String,
    pub date_label: String,
    pub clubs: String,
    pub status: String,
    pub note: String,
    pub ties: Vec<CupTieView>,
    pub has_ties: bool,
    pub tie_count: usize,
}

#[derive(Debug, Clone, Default)]
pub struct BracketTieView {
    pub id: String,
    pub stage: String,
    pub date_label: String,
    pub home: String,
    pub away: String,
    pub home_score: String,
    pub away_score: String,
    pub score_unavailable: bool,
    pub status: String,
    pub penalties: String,
    pub winner: String,
    pub winner_name: String,
    pub note: String,
    pub source_url: String,
    pub source_title: String,
    pub home_advanced: bool,
    pub away_advanced: bool,
    pub awarded: bool,
    pub extra_time: bool,
    pub feeds: String,
}

#[derive(Debug, Clone, Default)]
pub struct BracketRoundView {
    pub name: String,
    pub date_label: String,
    pub clubs: String,
    pub note: String,
    pub ties: Vec<BracketTieView>,
}

#[derive(Debug, Clone, Default)]
pub struct PastEditionView {
    pub year: String,
    pub home: String,
    pub away: String,
    pub score: String,
    pub note: String,
}

#[derive(Debug, Clone, Default)]
pub struct CupView {
    pub visible: bool,
    pub name: String,
    pub sponsor: String,
    pub teams_count: String,
    pub prize: String,
    pub reward: String,
    pub rounds: Vec<CupRoundView>,
    pub early_rounds: Vec<CupRoundView>,
    pub has_early_rounds: bool,
    pub bracket_rounds: Vec<BracketRoundView>,
    pub has_bracket: bool,
    pub bracket_source_url: String,
    pub bracket_source_title: String,
    pub past_editions: Vec<PastEditionView>,
    pub has_rounds: bool,
    pub has_past_editions: bool,
    pub warning: String,
    pub empty_title: String,
    pub empty_message: String,
}

#[derive(Debug, Clone, Default)]
pub struct StatRowView {
    pub rank: i32,
    pub team_id: String,
    pub team_name: String,
    pub value: String,
    pub bar_pct: i32,
}

#[derive(Debug, Clone, Default)]
pub struct StatPanelView {
    pub title: String,
    pub rows: Vec<StatRowView>,
}

#[derive(Debug, Clone, Default)]
pub struct StatsView {
    pub visible: bool,
    pub panels: Vec<StatPanelView>,
    pub warning: String,
    pub empty_title: String,
    pub empty_message: String,
}

#[derive(Debug, Clone, Default)]
pub struct ScorerRowView {
    pub rank: i32,
    pub player: String,
    pub team: String,
    pub team_id: String,
    pub nationality: String,
    pub goals: i32,
    pub appearances: i32,
    pub assists: i32,
}

#[derive(Debug, Clone, Default)]
pub struct ScorersView {
    pub visible: bool,
    pub rows: Vec<ScorerRowView>,
    pub unresolved_count: usize,
    pub empty_title: String,
    pub empty_message: String,
}

#[derive(Debug, Clone, Default)]
pub struct NewsCardView {
    pub title: String,
    pub summary: String,
    pub published_label: String,
    pub source_name: String,
    pub tag: String,
    pub emoji: String,
}

#[derive(Debug, Clone, Default)]
pub struct NewsView {
    pub visible: bool,
    pub articles: Vec<NewsCardView>,
    pub empty_title: String,
    pub empty_message: String,
}

#[derive(Debug, Clone, Default)]
pub struct TeamRowView {
    pub team_id: String,
    pub display_name: String,
    pub thai_name: String,
    pub organization_type: String,
    pub province: String,
    pub status: String,
    pub status_label: String,
    pub status_class: String,
    pub source_title: String,
    pub source_url: String,
    pub source_publisher: String,
    pub source_date: String,
    pub last_seen_at: String,
    pub notes: String,
    pub highlight: bool,
}

#[derive(Debug, Clone, Default)]
pub struct TeamsView {
    pub visible: bool,
    pub count: usize,
    pub verified_count: usize,
    pub unverified_count: usize,
    pub updated_at: String,
    pub age_label: String,
    pub type_label: String,
    pub group_name: String,
    pub rows: Vec<TeamRowView>,
    pub empty_title: String,
    pub empty_message: String,
    pub roster_status: String,
    pub roster_badge: String,
    pub roster_badge_class: String,
    pub roster_meta_line: String,
    pub displayed_season: String,
    pub source_season: String,
    pub roster_source_url: String,
    pub roster_source_checked_at: String,
}

#[derive(Debug, Clone, Default)]
pub struct SourceView {
    pub title: String,
    pub url: String,
    pub retrieved_at: String,
    pub kind: String,
    pub note: String,
}

#[derive(Debug, Clone, Template)]
#[template(path = "page.html")]
pub struct PageView {
    pub page_title: String,
    pub nav: NavView,
    pub competition: CompetitionHeaderView,
    pub season_options: Vec<SeasonOptionView>,
    pub season_label: String,
    pub season_id: String,
    pub tabs: Vec<TabView>,
    pub active_tab: String,
    pub group_query: String,
    pub mode_label: String,
    pub is_preview: bool,
    pub search_query: String,
    pub standings: StandingsView,
    pub teams: TeamsView,
    pub matches: MatchesView,
    pub cup_view: CupView,
    pub stats: StatsView,
    pub scorers: ScorersView,
    pub news_view: NewsView,
    pub sources: Vec<SourceView>,
    pub show_sources: bool,
    pub data_warnings: Vec<String>,
    pub captured_at: String,
    pub season_verified: bool,
}

#[derive(Debug, Clone, Default)]
pub struct SearchResultView {
    pub team_id: String,
    pub team_name: String,
    pub competition_name: String,
    pub group_name: String,
    pub season_label: String,
    pub href: String,
}

#[derive(Debug, Clone, Template)]
#[template(path = "search.html")]
pub struct SearchPageView {
    pub page_title: String,
    pub nav: NavView,
    pub mode_label: String,
    pub is_preview: bool,
    pub search_query: String,
    pub has_query: bool,
    pub query_too_short: bool,
    pub has_results: bool,
    pub results: Vec<SearchResultView>,
}

#[derive(Debug, Clone, Template)]
#[template(path = "partials/search_results.html")]
pub struct SearchResultsView {
    pub query: String,
    pub query_too_short: bool,
    pub has_results: bool,
    pub results: Vec<SearchResultView>,
}

fn href_competition(id: &str) -> String {
    format!("/competitions/{id}")
}

fn href_competition_season(id: &str, season_id: &str) -> String {
    format!("/competitions/{id}?season={season_id}")
}

fn href_season(comp: &Competition, season: &Season, tab: &str, group: Option<&str>) -> String {
    let mut href = format!("/competitions/{}/{tab}?season={}", comp.id, season.id);
    if let Some(group) = group
        && group != "overall"
    {
        href.push_str("&group=");
        href.push_str(group);
    }
    href
}

pub fn build_nav(data: &AppData, active_id: Option<&str>, active_season: Option<&str>) -> NavView {
    let mut sections = Vec::new();
    for section in &data.seed.nav {
        let mut items = Vec::new();
        for item in &section.items {
            if item.kind == "group" {
                let mut children = Vec::new();
                let mut open = false;
                for child in &item.children {
                    if let Some(id) = &child.id
                        && let Some(comp) = data.competition(id)
                    {
                        let is_season_item = child.kind == "season" && child.season.is_some();
                        let active = if is_season_item {
                            active_id == Some(comp.id.as_str())
                                && active_season == child.season.as_deref()
                        } else {
                            active_id == Some(comp.id.as_str())
                        };
                        open |= active;
                        let label = child
                            .label
                            .clone()
                            .unwrap_or_else(|| comp.short_name.clone());
                        let href = if is_season_item {
                            href_competition_season(
                                &comp.id,
                                child.season.as_deref().unwrap_or_default(),
                            )
                        } else {
                            href_competition(&comp.id)
                        };
                        children.push(NavItemView {
                            label,
                            subtitle: child.subtitle.clone().unwrap_or_default(),
                            href,
                            active,
                            open: false,
                            is_group: false,
                            children: Vec::new(),
                        });
                    }
                }
                items.push(NavItemView {
                    label: item
                        .label
                        .clone()
                        .unwrap_or_else(|| "Competition group".to_string()),
                    subtitle: item.subtitle.clone().unwrap_or_default(),
                    href: String::new(),
                    active: open,
                    open,
                    is_group: true,
                    children,
                });
            } else if let Some(id) = &item.id
                && let Some(comp) = data.competition(id)
            {
                items.push(NavItemView {
                    label: comp.name.clone(),
                    subtitle: item.subtitle.clone().unwrap_or_default(),
                    href: href_competition(&comp.id),
                    active: active_id == Some(comp.id.as_str()),
                    open: false,
                    is_group: false,
                    children: Vec::new(),
                });
            }
        }
        sections.push(NavSectionView {
            title: section.title.clone(),
            subtitle: section.subtitle.clone().unwrap_or_default(),
            is_bare: section.title.is_empty(),
            items,
        });
    }
    NavView { sections }
}

fn competition_header(comp: &Competition, season: &Season) -> CompetitionHeaderView {
    let has_records = !season.groups.is_empty()
        || !season.matches.is_empty()
        || season.cup.is_some()
        || !season.scorers.is_empty()
        || !season.news_ids.is_empty();
    CompetitionHeaderView {
        id: comp.id.clone(),
        name: comp.name.clone(),
        short_name: comp.short_name.clone(),
        section: comp.section.clone(),
        category: comp.category.clone(),
        kind: comp.kind.clone(),
        age_label: season
            .age_group
            .clone()
            .or_else(|| comp.age_group.clone())
            .unwrap_or_default(),
        group_label: comp
            .group_label
            .clone()
            .filter(|group| group != &comp.section)
            .unwrap_or_default(),
        description: comp.description.clone(),
        official_name: comp.official_name.clone(),
        aliases: comp.aliases.join(" · "),
        is_cup: comp.kind == "cup",
        is_youth: comp.category == "Youth",
        verified_label: if season.verified {
            "Verified".to_string()
        } else if comp.category == "Youth" && !has_records {
            "No records yet".to_string()
        } else {
            "Imported snapshot".to_string()
        },
        verified_class: if season.verified {
            "badge-verified".to_string()
        } else {
            "badge-unverified".to_string()
        },
    }
}

fn season_options(comp: &Competition, active: &Season) -> Vec<SeasonOptionView> {
    comp.seasons
        .iter()
        .map(|s| SeasonOptionView {
            id: s.id.clone(),
            label: s.label.clone(),
            selected: s.id == active.id,
        })
        .collect()
}

fn tabs(comp: &Competition, season: &Season, active: &str, group: Option<&str>) -> Vec<TabView> {
    let definitions: &[(&str, &str)] = if comp.kind == "cup" {
        &[("teams", "Teams"), ("rounds", "Rounds"), ("news", "News")]
    } else {
        &[
            ("teams", "Teams"),
            ("standings", "Standings"),
            ("results", "Results"),
            ("fixtures", "Fixtures"),
            ("stats", "Team stats"),
            ("scorers", "Top scorers"),
            ("news", "News"),
        ]
    };
    definitions
        .iter()
        .map(|(id, label)| TabView {
            id: (*id).to_string(),
            label: (*label).to_string(),
            href: href_season(comp, season, id, group),
            active: *id == active,
        })
        .collect()
}

fn format_gd(value: i32) -> String {
    if value > 0 {
        format!("+{value}")
    } else {
        value.to_string()
    }
}

fn standings_view(
    data: &AppData,
    comp: &Competition,
    season: &Season,
    group_id: Option<&str>,
    highlight: Option<&str>,
) -> StandingsView {
    let Some(group) = data.group(season, group_id) else {
        return StandingsView {
            empty_title: "No standings yet".to_string(),
            empty_message: if comp.category == "Youth" {
                "No verified records yet.".to_string()
            } else {
                "No standings records were imported for this competition.".to_string()
            },
            ..Default::default()
        };
    };
    let rows = group
        .standings
        .iter()
        .map(|row| standing_row(row, highlight))
        .collect::<Vec<_>>();
    let groups = season
        .groups
        .iter()
        .map(|g| GroupLinkView {
            id: g.id.clone(),
            name: g.name.clone(),
            href: href_season(comp, season, "standings", Some(&g.id)),
            active: g.id == group.id,
        })
        .collect();
    StandingsView {
        visible: !rows.is_empty(),
        group_name: group.name.clone(),
        show_group_nav: season.groups.len() > 1,
        groups,
        rows,
        validation_status: group.validation.status.clone(),
        validation_messages: group.validation.messages.clone(),
        empty_title: "No standings yet".to_string(),
        empty_message: if comp.category == "Youth" {
            "No verified records yet.".to_string()
        } else {
            "No standings records were imported for this competition.".to_string()
        },
    }
}

fn standing_row(row: &Standing, highlight: Option<&str>) -> StandingRowView {
    StandingRowView {
        team_id: row.team_id.clone(),
        rank: row.rank,
        team_name: row.team_name.clone(),
        played: row.played,
        won: row.won,
        drawn: row.drawn,
        lost: row.lost,
        goals_for: row.goals_for,
        goals_against: row.goals_against,
        goal_difference: format_gd(row.goal_difference),
        points: row.points,
        form: row.form.clone(),
        zone: row.zone.clone(),
        highlight: highlight == Some(row.team_id.as_str()),
    }
}

fn matches_view(comp: &Competition, season: &Season, kind: &str) -> MatchesView {
    let unresolved_count = season
        .matches
        .iter()
        .filter(|m| m.kind == kind && m.resolution != "resolved")
        .count();
    let mut groups: Vec<MatchGroupView> = Vec::new();
    for m in season
        .matches
        .iter()
        .filter(|m| m.kind == kind && m.resolution == "resolved")
    {
        let label = format!("{} · {}", m.date_label, m.round_label);
        let row = match_row(m, kind == "fixture");
        if let Some(group) = groups.iter_mut().find(|g| g.label == label) {
            group.matches.push(row);
        } else {
            groups.push(MatchGroupView {
                label,
                matches: vec![row],
            });
        }
    }
    let is_fixture = kind == "fixture";
    let empty_message = if comp.category == "Youth" {
        "No verified records yet.".to_string()
    } else if unresolved_count > 0 {
        "Imported records exist, but no record could be linked to this season's teams.".to_string()
    } else {
        "No records were imported for this competition.".to_string()
    };
    MatchesView {
        visible: !groups.is_empty(),
        title: if is_fixture {
            "Fixtures".to_string()
        } else {
            "Results".to_string()
        },
        subtitle: if is_fixture {
            "Upcoming matches in the imported snapshot.".to_string()
        } else {
            "Completed matches in the imported snapshot.".to_string()
        },
        is_fixture,
        groups,
        unresolved_count,
        empty_title: if is_fixture {
            "No fixtures yet".to_string()
        } else {
            "No results yet".to_string()
        },
        empty_message,
    }
}

fn match_row(m: &Match, is_fixture: bool) -> MatchRowView {
    let score = if is_fixture {
        m.status.clone()
    } else {
        match (m.home_goals, m.away_goals) {
            (Some(h), Some(a)) => format!("{h} – {a}"),
            _ => "—".to_string(),
        }
    };
    MatchRowView {
        home: m.home_name.clone(),
        away: m.away_name.clone(),
        score,
        status: m.status.clone(),
        note: m.note.clone(),
    }
}

fn cup_view(comp: &Competition, season: &Season, provenance: Option<(String, String)>) -> CupView {
    let Some(cup) = season.cup.as_ref() else {
        return CupView {
            empty_title: "No cup records yet".to_string(),
            empty_message: if comp.category == "Youth" {
                "No verified records yet.".to_string()
            } else {
                "No cup records were imported for this competition.".to_string()
            },
            ..Default::default()
        };
    };
    let rounds = cup
        .rounds
        .iter()
        .map(|round| {
            let ties = cup
                .ties
                .iter()
                .filter(|tie| tie.round_id.as_deref() == Some(round.id.as_str()))
                .map(|tie| CupTieView {
                    home: tie.home_name.clone(),
                    away: tie.away_name.clone(),
                    score: match (tie.home_goals, tie.away_goals) {
                        (Some(h), Some(a)) => format!("{h} - {a}"),
                        _ if tie.score_status.as_deref() == Some("unavailable") => {
                            "Score unavailable".to_string()
                        }
                        _ => "-".to_string(),
                    },
                    status: tie.status.clone(),
                    note: tie.note.clone(),
                    penalties: match (tie.home_penalties, tie.away_penalties) {
                        (Some(h), Some(a)) => format!("Penalties: {h} - {a}"),
                        _ => String::new(),
                    },
                    winner: match (&tie.winner_name, &tie.winner_basis) {
                        (Some(name), Some(basis)) if !basis.is_empty() => {
                            format!("Winner: {name} ({basis})")
                        }
                        (Some(name), _) => format!("Winner: {name}"),
                        _ => String::new(),
                    },
                })
                .collect::<Vec<_>>();
            CupRoundView {
                name: round.name.clone(),
                date_label: round.date_label.clone(),
                clubs: round
                    .clubs
                    .map(|c| c.to_string())
                    .unwrap_or_else(|| "—".to_string()),
                status: round.status.clone(),
                note: round.note.clone(),
                has_ties: !ties.is_empty(),
                tie_count: ties.len(),
                ties,
            }
        })
        .collect::<Vec<_>>();

    let main_order = ["Quarter-finals", "Semi-finals", "Final"];
    let main_rounds = main_order
        .iter()
        .filter_map(|name| {
            cup.rounds.iter().find(|round| {
                round.name == *name
                    && cup
                        .ties
                        .iter()
                        .any(|tie| tie.round_id.as_deref() == Some(round.id.as_str()))
            })
        })
        .collect::<Vec<_>>();
    let has_bracket = main_rounds.len() >= 2;
    let early_rounds = if has_bracket {
        rounds
            .iter()
            .filter(|round| !main_rounds.iter().any(|main| main.name == round.name))
            .cloned()
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    let mut bracket_rounds = Vec::new();
    if has_bracket {
        for round in &main_rounds {
            let ties = cup
                .ties
                .iter()
                .filter(|tie| tie.round_id.as_deref() == Some(round.id.as_str()))
                .map(|tie| {
                    let score_unavailable = tie.home_goals.is_none() || tie.away_goals.is_none();
                    // A few historical ties store no winner_name. Derive the
                    // advancing side from the recorded score; never from order.
                    let derived_winner = if tie.winner_name.is_none() {
                        match (tie.home_goals, tie.away_goals) {
                            (Some(home), Some(away)) if home > away => Some(tie.home_name.clone()),
                            (Some(home), Some(away)) if away > home => Some(tie.away_name.clone()),
                            _ => match (tie.home_penalties, tie.away_penalties) {
                                (Some(home), Some(away)) if home > away => {
                                    Some(tie.home_name.clone())
                                }
                                (Some(home), Some(away)) if away > home => {
                                    Some(tie.away_name.clone())
                                }
                                _ => None,
                            },
                        }
                    } else {
                        tie.winner_name.clone()
                    };
                    let winner = match (&tie.winner_name, &tie.winner_basis) {
                        (Some(name), Some(basis)) if !basis.is_empty() => {
                            format!("Winner: {name} ({basis})")
                        }
                        (Some(name), _) => format!("Winner: {name}"),
                        (None, _) => {
                            match (
                                &derived_winner,
                                tie.home_penalties.is_some(),
                                tie.extra_time,
                            ) {
                                (Some(name), true, _) => {
                                    format!("Winner: {name} (penalty shootout)")
                                }
                                (Some(name), false, true) => {
                                    format!("Winner: {name} (extra time)")
                                }
                                (Some(name), false, false) => format!("Winner: {name}"),
                                (None, _, _) => String::new(),
                            }
                        }
                    };
                    let (source_url, source_title) = provenance.clone().unwrap_or_default();
                    BracketTieView {
                        id: tie.id.clone(),
                        stage: round.name.clone(),
                        date_label: round.date_label.clone(),
                        home: tie.home_name.clone(),
                        away: tie.away_name.clone(),
                        home_score: tie
                            .home_goals
                            .map(|goals| goals.to_string())
                            .unwrap_or_default(),
                        away_score: tie
                            .away_goals
                            .map(|goals| goals.to_string())
                            .unwrap_or_default(),
                        score_unavailable,
                        status: tie.status.clone(),
                        penalties: match (tie.home_penalties, tie.away_penalties) {
                            (Some(h), Some(a)) => format!("Penalties: {h} - {a}"),
                            _ => String::new(),
                        },
                        winner,
                        winner_name: derived_winner.clone().unwrap_or_default(),
                        note: tie.note.clone(),
                        source_url,
                        source_title,
                        home_advanced: derived_winner.as_deref() == Some(tie.home_name.as_str()),
                        away_advanced: derived_winner.as_deref() == Some(tie.away_name.as_str()),
                        awarded: tie.status.eq_ignore_ascii_case("awarded"),
                        extra_time: tie.extra_time,
                        feeds: String::new(),
                    }
                })
                .collect::<Vec<_>>();
            bracket_rounds.push(BracketRoundView {
                name: round.name.clone(),
                date_label: round.date_label.clone(),
                clubs: round
                    .clubs
                    .map(|clubs| clubs.to_string())
                    .unwrap_or_else(|| "—".to_string()),
                note: round.note.clone(),
                ties,
            });
        }
        // Connections come from the recorded winner, never from card order.
        for index in 0..bracket_rounds.len().saturating_sub(1) {
            let targets = bracket_rounds[index + 1]
                .ties
                .iter()
                .map(|tie| (tie.id.clone(), tie.home.clone(), tie.away.clone()))
                .collect::<Vec<_>>();
            for tie in &mut bracket_rounds[index].ties {
                if tie.winner_name.is_empty() {
                    continue;
                }
                if let Some((id, _, _)) = targets
                    .iter()
                    .find(|(_, home, away)| home == &tie.winner_name || away == &tie.winner_name)
                {
                    tie.feeds = id.clone();
                }
            }
        }
    }

    let past_editions: Vec<PastEditionView> = cup
        .past_editions
        .iter()
        .map(|e| PastEditionView {
            year: e.year.clone(),
            home: e.home_name.clone(),
            away: e.away_name.clone(),
            score: match (e.home_goals, e.away_goals) {
                (Some(h), Some(a)) => format!("{h} – {a}"),
                _ => "—".to_string(),
            },
            note: e.note.clone(),
        })
        .collect();
    let warning = match comp.id.as_str() {
        "thai-fa-cup" => String::new(),
        "thai-league-cup" => String::new(),
        "thai-champion-cup" => {
            "The final participants are TBD in the imported snapshot.".to_string()
        }
        _ => String::new(),
    };
    CupView {
        visible: true,
        name: cup.name.clone(),
        sponsor: cup.sponsor.clone(),
        teams_count: cup
            .teams_count
            .map(|c| c.to_string())
            .unwrap_or_else(|| "—".to_string()),
        prize: cup.prize.clone(),
        reward: cup.reward.clone(),
        has_rounds: !rounds.is_empty(),
        has_past_editions: !past_editions.is_empty(),
        has_early_rounds: !early_rounds.is_empty(),
        early_rounds,
        bracket_rounds,
        has_bracket,
        bracket_source_url: provenance
            .as_ref()
            .map(|(url, _)| url.clone())
            .unwrap_or_default(),
        bracket_source_title: provenance
            .as_ref()
            .map(|(_, title)| title.clone())
            .unwrap_or_default(),
        rounds,
        past_editions,
        warning,
        empty_title: "No cup records yet".to_string(),
        empty_message: "No cup records were imported for this competition.".to_string(),
    }
}

fn stats_view(comp: &Competition, season: &Season, group_id: Option<&str>) -> StatsView {
    let Some(group) = season
        .groups
        .iter()
        .find(|g| Some(g.id.as_str()) == group_id)
        .or_else(|| season.groups.first())
    else {
        return StatsView {
            empty_title: "No team statistics yet".to_string(),
            empty_message: if comp.category == "Youth" {
                "No verified records yet.".to_string()
            } else {
                "Statistics require standings records for this competition.".to_string()
            },
            ..Default::default()
        };
    };
    if group.standings.is_empty() {
        return StatsView {
            empty_title: "No team statistics yet".to_string(),
            empty_message: "Statistics require standings records for this competition.".to_string(),
            ..Default::default()
        };
    }
    let rows = &group.standings;
    let max_gf = rows.iter().map(|r| r.goals_for).max().unwrap_or(1).max(1);
    let max_ga = rows
        .iter()
        .map(|r| r.goals_against)
        .max()
        .unwrap_or(1)
        .max(1);
    let max_gd = rows
        .iter()
        .map(|r| r.goal_difference.abs())
        .max()
        .unwrap_or(1)
        .max(1);
    let max_wr = rows
        .iter()
        .map(|r| {
            if r.played > 0 {
                r.won as f64 / r.played as f64
            } else {
                0.0
            }
        })
        .fold(0.0_f64, f64::max)
        .max(0.01);

    let mut goals = rows.clone();
    goals.sort_by_key(|a| std::cmp::Reverse(a.goals_for));
    let mut defence = rows.clone();
    defence.sort_by_key(|a| a.goals_against);
    let mut gd = rows.clone();
    gd.sort_by_key(|a| std::cmp::Reverse(a.goal_difference));
    let mut win_rate = rows.clone();
    win_rate.sort_by(|a, b| {
        let ar = if a.played > 0 {
            a.won as f64 / a.played as f64
        } else {
            0.0
        };
        let br = if b.played > 0 {
            b.won as f64 / b.played as f64
        } else {
            0.0
        };
        br.partial_cmp(&ar).unwrap_or(std::cmp::Ordering::Equal)
    });

    let panels = vec![
        stat_panel(
            "Most goals scored",
            &goals,
            |r| r.goals_for.to_string(),
            |r| (r.goals_for as f64 / max_gf as f64 * 100.0).round() as i32,
        ),
        stat_panel(
            "Best defence",
            &defence,
            |r| r.goals_against.to_string(),
            |r| {
                ((1.0 - (r.goals_against as f64 / max_ga as f64)) * 100.0)
                    .max(4.0)
                    .round() as i32
            },
        ),
        stat_panel(
            "Goal difference",
            &gd,
            |r| format_gd(r.goal_difference),
            |r| (r.goal_difference.abs() as f64 / max_gd as f64 * 100.0).round() as i32,
        ),
        stat_panel(
            "Win rate",
            &win_rate,
            |r| {
                format!(
                    "{}%",
                    if r.played > 0 {
                        (r.won as f64 / r.played as f64 * 100.0).round() as i32
                    } else {
                        0
                    }
                )
            },
            |r| {
                let wr = if r.played > 0 {
                    r.won as f64 / r.played as f64
                } else {
                    0.0
                };
                (wr / max_wr * 100.0).round() as i32
            },
        ),
    ];

    StatsView {
        visible: true,
        panels,
        warning: if group.validation.status == "failed" {
            "Standings for this group failed arithmetic validation; statistics are derived from the imported rows and remain unverified.".to_string()
        } else {
            String::new()
        },
        empty_title: "No team statistics yet".to_string(),
        empty_message: "Statistics require standings records for this competition.".to_string(),
    }
}

fn stat_panel<F, B>(title: &str, rows: &[Standing], value: F, bar: B) -> StatPanelView
where
    F: Fn(&Standing) -> String,
    B: Fn(&Standing) -> i32,
{
    StatPanelView {
        title: title.to_string(),
        rows: rows
            .iter()
            .take(8)
            .enumerate()
            .map(|(index, row)| StatRowView {
                rank: index as i32 + 1,
                team_id: row.team_id.clone(),
                team_name: row.team_name.clone(),
                value: value(row),
                bar_pct: bar(row).clamp(0, 100),
            })
            .collect(),
    }
}

fn scorers_view(
    comp: &Competition,
    season: &Season,
    team_names: &[(String, String)],
) -> ScorersView {
    let unresolved_count = season
        .scorers
        .iter()
        .filter(|s| s.resolution != "resolved")
        .count();
    let rows = season
        .scorers
        .iter()
        .filter(|s| s.resolution == "resolved")
        .map(|s| {
            let team_id = s.team_id.clone().unwrap_or_default();
            let team = team_names
                .iter()
                .find(|(id, _)| id == &team_id)
                .map(|(_, name)| name.clone())
                .unwrap_or_else(|| s.team_name.clone());
            ScorerRowView {
                rank: s.rank,
                player: s.player_name.clone(),
                team,
                team_id,
                nationality: s.nationality.clone(),
                goals: s.goals,
                appearances: s.appearances,
                assists: s.assists,
            }
        })
        .collect::<Vec<_>>();
    ScorersView {
        visible: !rows.is_empty(),
        rows,
        unresolved_count,
        empty_title: "No top scorers yet".to_string(),
        empty_message: if comp.category == "Youth" {
            "No verified records yet.".to_string()
        } else if unresolved_count > 0 {
            "Imported scorer records exist, but none can be linked to this season's teams."
                .to_string()
        } else {
            "No scorer records were imported for this competition.".to_string()
        },
    }
}

fn news_view(data: &AppData, season: &Season) -> NewsView {
    let articles = data
        .news_for(season)
        .into_iter()
        .map(news_card)
        .collect::<Vec<_>>();
    NewsView {
        visible: !articles.is_empty(),
        articles,
        empty_title: "No news yet".to_string(),
        empty_message: "No news articles are linked to this competition in the imported snapshot."
            .to_string(),
    }
}

fn news_card(article: &NewsArticle) -> NewsCardView {
    NewsCardView {
        title: article.title.clone(),
        summary: article.summary.clone(),
        published_label: article.published_label.clone(),
        source_name: article.source_name.clone(),
        tag: article.tag.clone(),
        emoji: article.emoji.clone(),
    }
}

fn source_view(source: &Source) -> SourceView {
    SourceView {
        title: source.title.clone(),
        url: source.url.clone().unwrap_or_default(),
        retrieved_at: source.retrieved_at.clone(),
        kind: source.kind.clone(),
        note: source.note.clone(),
    }
}

fn team_name_pairs(season: &Season) -> Vec<(String, String)> {
    season
        .groups
        .iter()
        .flat_map(|g| {
            g.standings
                .iter()
                .map(|r| (r.team_id.clone(), r.team_name.clone()))
        })
        .collect()
}

fn team_status_label(status: &str) -> (&'static str, &'static str) {
    match status {
        "verified_official" => ("Verified official", "badge-verified"),
        "verified_secondary" => ("Verified secondary", "badge-verified"),
        "discovered_unverified" => ("Unverified candidate", "badge-unverified"),
        "conflicting" => ("Conflicting", "badge-warn"),
        "rejected" => ("Rejected", "badge-warn"),
        _ => ("Unknown", "badge-neutral"),
    }
}

fn teams_view(
    data: &AppData,
    comp: &Competition,
    season: &Season,
    group_id: Option<&str>,
    highlight: Option<&str>,
) -> TeamsView {
    let rows = data.teams_for(&comp.id, &season.id, group_id);
    let verified_count = rows
        .iter()
        .filter(|(_, participation)| participation.verification_status.starts_with("verified"))
        .count();
    let unverified_count = rows.len().saturating_sub(verified_count);
    let updated_at = if data.team_registry.generated_at.is_empty() {
        season.captured_at.clone()
    } else {
        data.team_registry.generated_at.clone()
    };
    let updated_at = updated_at.chars().take(10).collect::<String>();
    let age_label = season
        .age_group
        .clone()
        .or_else(|| comp.age_group.clone())
        .unwrap_or_else(|| "Open age".to_string());
    let group_name = group_id
        .and_then(|wanted| season.groups.iter().find(|group| group.id == wanted))
        .map(|group| group.name.clone())
        .unwrap_or_else(|| "Team list".to_string());
    let roster = data.roster_for(&comp.id, &season.id);
    let roster_status = roster
        .map(|roster| roster.roster_status.clone())
        .unwrap_or_default();
    let displayed_season = roster
        .map(|roster| roster.displayed_season.clone())
        .unwrap_or_default();
    let source_season = roster
        .map(|roster| roster.source_season.clone())
        .unwrap_or_default();
    let roster_source_url = roster
        .and_then(|roster| roster.source_url.clone())
        .unwrap_or_default();
    let roster_source_checked_at = roster
        .map(|roster| roster.source_checked_at.clone())
        .unwrap_or_default();
    let (roster_badge, roster_badge_class) = match roster_status.as_str() {
        "current" => (
            format!("Current season roster: {displayed_season}"),
            "badge-verified",
        ),
        "latest_verified" => (
            format!("Latest verified roster: {displayed_season}"),
            "badge-warn",
        ),
        "partial" => (format!("Partial list: {displayed_season}"), "badge-warn"),
        "unavailable" => ("Roster unavailable".to_string(), "badge-neutral"),
        _ => (String::new(), "badge-neutral"),
    };
    let roster_meta_line = if displayed_season.is_empty() {
        String::new()
    } else if roster_source_checked_at.is_empty() {
        format!("Displayed season {displayed_season} · source season {source_season}")
    } else {
        format!(
            "Displayed season {displayed_season} · source season {source_season} · checked {roster_source_checked_at}"
        )
    };
    let empty_message = match comp.id.as_str() {
        "thai-youth-league-u18" | "thai-youth-league-u16" | "thai-youth-league-u14" => {
            format!(
                "No verified 2026 team list is published for {} {}. The FA Thailand Youth League was not listed in the 2025/26 or 2026/27 official competition calendar, and no teams are fabricated.",
                comp.name, age_label
            )
        }
        "thai-school-league-general" => {
            "The General category is an open-entry knockout (413 teams entered in 2025/2568). No complete 2026 public entry list was located, and a partial list is not presented as the full field. No teams are fabricated."
                .to_string()
        }
        "thai-college-cup-u19" => {
            "No complete public team list for the 2026 national vocational (U19) football competition was located; public reporting names individual colleges only. No teams are fabricated."
                .to_string()
        }
        "thai-school-league-type-b" if age_label == "U14" => {
            "No 2026 Type B U14 public team list was located. The latest published final was Ratchawinit Bang Kaeo vs Mathayom Taksin Rayong (2025/2568). No teams are fabricated."
                .to_string()
        }
        _ if comp.category == "Youth" => {
            "No verified team list has been discovered for this competition and age group. No teams are fabricated."
                .to_string()
        }
        _ => {
            "No team records have been discovered or imported for this competition. No teams are fabricated."
                .to_string()
        }
    };
    let empty_message = if roster_status == "unavailable" {
        format!("{empty_message} No reliable previous-season roster is available.")
    } else {
        empty_message
    };
    TeamsView {
        visible: !rows.is_empty(),
        count: rows.len(),
        verified_count,
        unverified_count,
        updated_at,
        age_label: age_label.clone(),
        type_label: comp.kind.clone(),
        group_name,
        rows: rows
            .into_iter()
            .map(|(team, participation)| {
                let (status_label, status_class) =
                    team_status_label(&participation.verification_status);
                TeamRowView {
                    team_id: team.team_id.clone(),
                    display_name: team.display_name.clone(),
                    thai_name: team
                        .thai_name
                        .clone()
                        .filter(|name| !name.trim().is_empty())
                        .unwrap_or_else(|| "—".to_string()),
                    organization_type: team.organization_type.clone(),
                    province: team
                        .province
                        .clone()
                        .filter(|province| !province.trim().is_empty())
                        .unwrap_or_else(|| "—".to_string()),
                    status: participation.verification_status.clone(),
                    status_label: status_label.to_string(),
                    status_class: status_class.to_string(),
                    source_title: participation
                        .source_title
                        .clone()
                        .unwrap_or_else(|| "No source recorded".to_string()),
                    source_url: participation.source_url.clone().unwrap_or_default(),
                    source_publisher: participation.source_publisher.clone().unwrap_or_default(),
                    source_date: participation.source_date.clone().unwrap_or_default(),
                    last_seen_at: participation
                        .last_seen_at
                        .chars()
                        .take(10)
                        .collect::<String>(),
                    notes: participation.notes.clone(),
                    highlight: highlight == Some(team.team_id.as_str()),
                }
            })
            .collect(),
        empty_title: "No verified team list yet".to_string(),
        empty_message,
        roster_status,
        roster_badge,
        roster_badge_class: roster_badge_class.to_string(),
        roster_meta_line,
        displayed_season,
        source_season,
        roster_source_url,
        roster_source_checked_at,
    }
}

pub fn build_page(
    data: &AppData,
    competition_id: &str,
    season_id: Option<&str>,
    requested_tab: Option<&str>,
    group_id: Option<&str>,
    highlight: Option<&str>,
    query: &str,
) -> Option<PageView> {
    let comp = data.competition(competition_id)?;
    if !comp.active {
        return None;
    }
    let season = data.season(comp, season_id)?;
    // Open on the tab that has data. School and youth pages often have no
    // standings/cup rounds, but they do have team-list participations.
    let has_standings = season
        .groups
        .iter()
        .any(|group| !group.standings.is_empty());
    let has_cup_content = season
        .cup
        .as_ref()
        .is_some_and(|cup| !cup.rounds.is_empty() || !cup.ties.is_empty());
    let default_tab = if comp.kind == "cup" {
        if has_cup_content { "rounds" } else { "teams" }
    } else if has_standings {
        "standings"
    } else {
        "teams"
    };
    let allowed = if comp.kind == "cup" {
        ["teams", "rounds", "news"].as_slice()
    } else {
        [
            "teams",
            "standings",
            "results",
            "fixtures",
            "stats",
            "scorers",
            "news",
        ]
        .as_slice()
    };
    let active_tab = requested_tab
        .filter(|tab| allowed.contains(tab))
        .unwrap_or(default_tab);
    let group = data.group(season, group_id);
    let group_key = group.map(|g| g.id.as_str());
    let group_query = group_key
        .filter(|id| *id != "overall")
        .unwrap_or("")
        .to_string();
    let header = competition_header(comp, season);
    let season_opts = season_options(comp, season);
    let tab_views = tabs(comp, season, active_tab, group_key);
    let sources = data
        .sources_for(comp, season)
        .into_iter()
        .map(source_view)
        .collect::<Vec<_>>();

    let mut data_warnings = Vec::new();
    let team_rows = data.teams_for(&comp.id, &season.id, group_key);
    let has_records = !season.groups.is_empty()
        || !season.matches.is_empty()
        || season.cup.is_some()
        || !season.scorers.is_empty()
        || !season.news_ids.is_empty()
        || !team_rows.is_empty();
    let teams = if active_tab == "teams" {
        teams_view(data, comp, season, group_key, highlight)
    } else {
        TeamsView::default()
    };
    if active_tab == "teams" {
        if teams.unverified_count > 0 {
            data_warnings.push(format!(
                "{} team list entr{} marked unverified; resolve them before treating the list as official.",
                teams.unverified_count,
                if teams.unverified_count == 1 { "y is" } else { "ies are" }
            ));
        } else if teams.verified_count > 0 {
            data_warnings.push(
                "Team list entries are verified against secondary sources; official confirmation remains preferred."
                    .to_string(),
            );
        }
        if let Some(roster) = data.roster_for(&comp.id, &season.id) {
            match roster.roster_status.as_str() {
                "latest_verified" => data_warnings.push(format!(
                    "Latest verified roster: {}. The current-season team list has not been published or verified. The latest verified roster is shown for reference.",
                    roster.displayed_season
                )),
                "partial" => data_warnings.push(format!(
                    "Partial list: {}. The full current-season team list has not been published or verified.",
                    roster.displayed_season
                )),
                "unavailable" => data_warnings.push(
                    "Roster unavailable: no reliable current-season or previous-season team list is available."
                        .to_string(),
                ),
                _ => {}
            }
        }
    } else if comp.category == "Youth" && !has_records {
        data_warnings.push("Owner-defined youth category. No verified records have been imported; no fixtures, results, or teams are fabricated.".to_string());
    } else if !season.verified {
        data_warnings.push("Imported snapshot. Values are preserved from the owner-supplied reference application and are not independently verified.".to_string());
    }

    let standings = if active_tab == "standings" {
        standings_view(data, comp, season, group_key, highlight)
    } else {
        StandingsView::default()
    };
    if active_tab == "standings" && standings.validation_status == "failed" {
        data_warnings.push(format!(
            "Arithmetic validation failed for {}: {}",
            standings.group_name,
            standings.validation_messages.join(" ")
        ));
    }

    let matches = if active_tab == "results" {
        matches_view(comp, season, "result")
    } else if active_tab == "fixtures" {
        matches_view(comp, season, "fixture")
    } else {
        MatchesView::default()
    };
    if matches.unresolved_count > 0 && (active_tab == "results" || active_tab == "fixtures") {
        data_warnings.push(format!(
            "{} imported {} record(s) could not be linked to this season's teams and are not shown in the table.",
            matches.unresolved_count,
            if active_tab == "fixtures" { "fixture" } else { "result" }
        ));
    }

    let cup_view = if active_tab == "rounds" {
        let provenance = data
            .team_registry
            .participations
            .iter()
            .filter(|participation| {
                participation.competition_id == comp.id && participation.season_id == season.id
            })
            .find_map(|participation| {
                let url = participation
                    .source_url
                    .as_deref()
                    .filter(|url| !url.is_empty())?;
                Some((
                    url.to_string(),
                    participation.source_title.clone().unwrap_or_default(),
                ))
            })
            .or_else(|| {
                sources
                    .iter()
                    .find(|source| !source.url.is_empty())
                    .map(|source| (source.url.clone(), source.title.clone()))
            });
        cup_view(comp, season, provenance)
    } else {
        CupView::default()
    };
    if active_tab == "rounds" && !cup_view.warning.is_empty() {
        data_warnings.push(cup_view.warning.clone());
    }

    let stats = if active_tab == "stats" {
        stats_view(comp, season, group_key)
    } else {
        StatsView::default()
    };
    if active_tab == "stats" && !stats.warning.is_empty() {
        data_warnings.push(stats.warning.clone());
    }

    let scorers = if active_tab == "scorers" {
        scorers_view(comp, season, &team_name_pairs(season))
    } else {
        ScorersView::default()
    };
    if active_tab == "scorers" && scorers.unresolved_count > 0 {
        data_warnings.push(format!(
            "{} imported scorer record(s) could not be linked to this season's teams and are not shown.",
            scorers.unresolved_count
        ));
    }

    let news_view = if active_tab == "news" {
        news_view(data, season)
    } else {
        NewsView::default()
    };

    Some(PageView {
        page_title: format!("{} · Thailand Football", comp.name),
        nav: build_nav(data, Some(comp.id.as_str()), Some(season.id.as_str())),
        competition: header,
        season_options: season_opts,
        season_label: season.label.clone(),
        season_id: season.id.clone(),
        tabs: tab_views,
        active_tab: active_tab.to_string(),
        group_query,
        mode_label: if data.seed.meta.mode == "local-preview" {
            "Local preview".to_string()
        } else {
            "PostgreSQL".to_string()
        },
        is_preview: data.seed.meta.mode == "local-preview",
        search_query: query.to_string(),
        standings,
        teams,
        matches,
        cup_view,
        stats,
        scorers,
        news_view,
        sources,
        show_sources: !data.sources_for(comp, season).is_empty(),
        data_warnings,
        captured_at: season.captured_at.clone(),
        season_verified: season.verified,
    })
}

fn search_href(entry: &SearchEntry) -> String {
    let mut href = format!(
        "/competitions/{}/standings?season={}&group={}&highlight={}",
        entry.competition_id, entry.season_id, entry.group_id, entry.team_id
    );
    if entry.group_id == "overall" {
        href = format!(
            "/competitions/{}/standings?season={}&highlight={}",
            entry.competition_id, entry.season_id, entry.team_id
        );
    }
    href
}

pub fn search_results(data: &AppData, query: &str) -> Vec<SearchResultView> {
    data.search(query)
        .into_iter()
        .map(|entry| {
            let season_label = data
                .roster_for(&entry.competition_id, &entry.season_id)
                .map(|roster| roster.displayed_season.clone())
                .unwrap_or_else(|| entry.season_label.clone());
            SearchResultView {
                team_id: entry.team_id.clone(),
                team_name: entry.team_name.clone(),
                competition_name: entry.competition_name.clone(),
                group_name: entry.group_name.clone(),
                season_label,
                href: search_href(entry),
            }
        })
        .collect()
}

pub fn build_search_page(data: &AppData, query: &str) -> SearchPageView {
    let trimmed = query.trim();
    let results = search_results(data, query);
    SearchPageView {
        page_title: "Search · Thailand Football".to_string(),
        nav: build_nav(data, None, None),
        mode_label: if data.seed.meta.mode == "local-preview" {
            "Local preview".to_string()
        } else {
            "PostgreSQL".to_string()
        },
        is_preview: data.seed.meta.mode == "local-preview",
        search_query: query.to_string(),
        has_query: !trimmed.is_empty(),
        query_too_short: trimmed.len() < 2,
        has_results: !results.is_empty(),
        results,
    }
}

pub fn search_results_view(data: &AppData, query: &str) -> SearchResultsView {
    let trimmed = query.trim();
    let results = search_results(data, query);
    SearchResultsView {
        query: query.to_string(),
        query_too_short: trimmed.len() < 2,
        has_results: !results.is_empty(),
        results,
    }
}

pub fn default_competition(data: &AppData) -> Option<&Competition> {
    data.active_competitions().min_by_key(|c| c.sort_order)
}
