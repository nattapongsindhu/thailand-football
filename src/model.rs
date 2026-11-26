use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct Seed {
    pub meta: Meta,
    pub sources: Vec<Source>,
    pub nav: Vec<NavSection>,
    pub competitions: Vec<Competition>,
    pub news: Vec<NewsArticle>,
    #[serde(default)]
    pub legacy: Vec<Competition>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    pub app: String,
    pub mode: String,
    pub generated_at: String,
    pub legacy_source: String,
    pub legacy_sha256: String,
    #[serde(default)]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Source {
    pub id: String,
    pub title: String,
    pub url: Option<String>,
    pub retrieved_at: String,
    pub kind: String,
    pub note: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NavSection {
    pub title: String,
    #[serde(default)]
    pub subtitle: Option<String>,
    pub items: Vec<NavItem>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NavItem {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: Option<String>,
    pub label: Option<String>,
    #[serde(default)]
    pub subtitle: Option<String>,
    #[serde(default)]
    pub season: Option<String>,
    #[serde(default)]
    pub children: Vec<NavItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Competition {
    pub id: String,
    pub name: String,
    pub short_name: String,
    pub section: String,
    pub category: String,
    pub kind: String,
    pub age_group: Option<String>,
    pub sort_order: i32,
    pub active: bool,
    pub official_name: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    pub group_label: Option<String>,
    pub description: String,
    #[serde(default)]
    pub source_ids: Vec<String>,
    pub seasons: Vec<Season>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Season {
    pub id: String,
    pub label: String,
    pub is_current: bool,
    pub verified: bool,
    pub captured_at: String,
    #[serde(default)]
    pub age_group: Option<String>,
    #[serde(default)]
    pub source_ids: Vec<String>,
    #[serde(default)]
    pub groups: Vec<Group>,
    #[serde(default)]
    pub matches: Vec<Match>,
    pub cup: Option<Cup>,
    #[serde(default)]
    pub scorers: Vec<Scorer>,
    #[serde(default)]
    pub news_ids: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Group {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub standings: Vec<Standing>,
    pub validation: Validation,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Standing {
    pub team_id: String,
    pub team_name: String,
    pub rank: i32,
    pub played: i32,
    pub won: i32,
    pub drawn: i32,
    pub lost: i32,
    pub goals_for: i32,
    pub goals_against: i32,
    pub goal_difference: i32,
    pub points: i32,
    #[serde(default)]
    pub form: Vec<String>,
    pub zone: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Validation {
    pub status: String,
    #[serde(default)]
    pub messages: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Match {
    pub id: String,
    pub kind: String,
    pub round_label: String,
    pub date_label: String,
    pub home_team_id: Option<String>,
    pub away_team_id: Option<String>,
    pub home_name: String,
    pub away_name: String,
    pub home_goals: Option<i32>,
    pub away_goals: Option<i32>,
    pub status: String,
    pub note: String,
    pub resolution: String,
    #[serde(default)]
    pub issues: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Cup {
    pub name: String,
    pub sponsor: String,
    pub teams_count: Option<i32>,
    pub prize: String,
    pub reward: String,
    #[serde(default)]
    pub rounds: Vec<CupRound>,
    #[serde(default)]
    pub ties: Vec<CupTie>,
    #[serde(default)]
    pub past_editions: Vec<PastEdition>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CupRound {
    pub id: String,
    pub name: String,
    pub date_label: String,
    pub clubs: Option<i32>,
    pub status: String,
    pub note: String,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CupTie {
    pub id: String,
    pub round_id: Option<String>,
    pub stage: String,
    pub home_name: String,
    pub away_name: String,
    pub home_goals: Option<i32>,
    pub away_goals: Option<i32>,
    pub status: String,
    pub note: String,
    pub sort_order: i32,
    #[serde(default)]
    pub score_status: Option<String>,
    #[serde(default)]
    pub home_penalties: Option<i32>,
    #[serde(default)]
    pub away_penalties: Option<i32>,
    #[serde(default)]
    pub winner_name: Option<String>,
    #[serde(default)]
    pub winner_basis: Option<String>,
    #[serde(default)]
    pub extra_time: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PastEdition {
    pub id: String,
    pub year: String,
    pub home_name: String,
    pub away_name: String,
    pub home_goals: Option<i32>,
    pub away_goals: Option<i32>,
    pub note: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Scorer {
    pub id: String,
    pub rank: i32,
    pub player_name: String,
    pub team_id: Option<String>,
    pub team_name: String,
    pub nationality: String,
    pub goals: i32,
    pub appearances: i32,
    pub assists: i32,
    pub resolution: String,
    #[serde(default)]
    pub issues: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewsArticle {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub published_label: String,
    pub source_name: String,
    pub tag: String,
    pub emoji: String,
    #[serde(default)]
    pub competition_ids: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TeamRegistry {
    pub generated_at: String,
    #[serde(default)]
    pub source_registry_version: u32,
    #[serde(default)]
    pub teams: Vec<Team>,
    #[serde(default)]
    pub participations: Vec<TeamParticipation>,
    #[serde(default)]
    pub rosters: Vec<RosterMeta>,
}

impl Default for TeamRegistry {
    fn default() -> Self {
        Self {
            generated_at: String::new(),
            source_registry_version: 1,
            teams: Vec::new(),
            participations: Vec::new(),
            rosters: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Team {
    pub team_id: String,
    pub display_name: String,
    #[serde(default)]
    pub thai_name: Option<String>,
    #[serde(default)]
    pub english_name: Option<String>,
    #[serde(default)]
    pub aliases: Vec<String>,
    pub organization_type: String,
    #[serde(default)]
    pub province: Option<String>,
    pub verification_status: String,
    pub first_seen_at: String,
    pub last_seen_at: String,
    #[serde(default)]
    pub notes: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TeamParticipation {
    pub participation_id: String,
    pub team_id: String,
    pub competition_id: String,
    pub season_id: String,
    #[serde(default)]
    pub age_group: Option<String>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub division_type: Option<String>,
    #[serde(default)]
    pub source_url: Option<String>,
    #[serde(default)]
    pub source_title: Option<String>,
    #[serde(default)]
    pub source_publisher: Option<String>,
    #[serde(default)]
    pub source_date: Option<String>,
    pub verification_status: String,
    pub first_seen_at: String,
    pub last_seen_at: String,
    #[serde(default)]
    pub notes: String,
    #[serde(default)]
    pub displayed_season: Option<String>,
    #[serde(default)]
    pub source_season: Option<String>,
    #[serde(default)]
    pub roster_status: Option<String>,
    #[serde(default)]
    pub source_checked_at: Option<String>,
}

/// Roster-level fallback metadata for a competition season. Participation
/// season IDs stay unchanged; this record explains which season the visible
/// team list actually came from.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RosterMeta {
    pub competition_id: String,
    pub season_id: String,
    #[serde(default)]
    pub age_group: Option<String>,
    pub displayed_season: String,
    pub source_season: String,
    pub roster_status: String,
    #[serde(default)]
    pub source_url: Option<String>,
    pub source_checked_at: String,
    #[serde(default)]
    pub note: String,
    #[serde(default)]
    pub team_count: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DiscoveryNews {
    pub news_id: String,
    pub title: String,
    pub source_name: String,
    #[serde(default)]
    pub source_url: Option<String>,
    pub published_at: String,
    #[serde(default)]
    pub competition_id: Option<String>,
    #[serde(default)]
    pub season_id: Option<String>,
    #[serde(default)]
    pub age_group: Option<String>,
    #[serde(default)]
    pub extracted_team_ids: Vec<String>,
    pub verification_status: String,
    pub summary: String,
    pub discovered_at: String,
}
