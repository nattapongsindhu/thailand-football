use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result};
use chrono::Utc;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::model::{
    Competition, DiscoveryNews, RosterMeta, Seed, Team, TeamParticipation, TeamRegistry,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SourceRegistry {
    pub version: u32,
    pub generated_at: String,
    #[serde(default)]
    pub competitions: Vec<RegistryCompetition>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegistryCompetition {
    pub competition_id: String,
    pub display_name: String,
    pub organizer_group: String,
    #[serde(default)]
    pub age_group: Option<String>,
    pub source_priority: String,
    #[serde(default)]
    pub official_domains: Vec<String>,
    #[serde(default)]
    pub official_page_urls: Vec<String>,
    #[serde(default)]
    pub secondary_domains: Vec<String>,
    #[serde(default)]
    pub secondary_page_urls: Vec<String>,
    #[serde(default)]
    pub thai_search_terms: Vec<String>,
    #[serde(default)]
    pub english_search_terms: Vec<String>,
    pub update_status: String,
    #[serde(default)]
    pub last_checked_at: Option<String>,
    #[serde(default)]
    pub fetch_urls: Vec<String>,
    #[serde(default)]
    pub news_feed_urls: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CuratedTeamFile {
    #[serde(default)]
    pub teams: Vec<Team>,
    #[serde(default)]
    pub participations: Vec<TeamParticipation>,
    #[serde(default)]
    pub rosters: Vec<RosterMeta>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FetchRecord {
    pub competition_id: String,
    pub url: String,
    pub fetched_at: String,
    pub status: String,
    pub http_status: Option<u16>,
    pub content_type: Option<String>,
    pub title: Option<String>,
    pub bytes: usize,
    pub sha256: Option<String>,
    pub candidate_count: usize,
    pub candidates: Vec<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidationCounts {
    pub teams: usize,
    pub participations: usize,
    pub verified_official: usize,
    pub verified_secondary: usize,
    pub discovered_unverified: usize,
    pub conflicting: usize,
    pub rejected: usize,
    pub news_items: usize,
    pub team_candidates: usize,
    pub fetched_sources: usize,
    pub failed_sources: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidationReport {
    pub generated_at: String,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub counts: ValidationCounts,
    pub competition_summary: Vec<CompetitionSummary>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CompetitionSummary {
    pub competition_id: String,
    pub display_name: String,
    pub organizer_group: String,
    pub teams: usize,
    pub verified_official: usize,
    pub verified_secondary: usize,
    pub discovered_unverified: usize,
}

pub fn bootstrap_registry(seed: &Seed) -> TeamRegistry {
    let mut teams: HashMap<String, Team> = HashMap::new();
    let mut participations = Vec::new();
    let mut seen_participations = HashSet::new();

    for competition in seed
        .competitions
        .iter()
        .filter(|competition| competition.active)
    {
        for season in &competition.seasons {
            for group in &season.groups {
                for row in &group.standings {
                    let captured = if season.captured_at.is_empty() {
                        "2026-03-22".to_string()
                    } else {
                        season.captured_at.clone()
                    };
                    teams.entry(row.team_id.clone()).or_insert_with(|| Team {
                        team_id: row.team_id.clone(),
                        display_name: row.team_name.clone(),
                        thai_name: None,
                        english_name: None,
                        aliases: Vec::new(),
                        organization_type: if competition.id.starts_with("thai-league-")
                            && competition.id != "thai-league-3"
                        {
                            "club".to_string()
                        } else {
                            "unknown".to_string()
                        },
                        province: None,
                        verification_status: "discovered_unverified".to_string(),
                        first_seen_at: captured.clone(),
                        last_seen_at: captured.clone(),
                        notes: "Bootstrapped from the owner-supplied standings snapshot."
                            .to_string(),
                    });
                    let key = format!(
                        "{}|{}|{}|{}",
                        row.team_id, competition.id, season.id, group.id
                    );
                    if !seen_participations.insert(key) {
                        continue;
                    }
                    participations.push(TeamParticipation {
                        participation_id: format!(
                            "part-{}-{}-{}-{}",
                            competition.id, season.id, group.id, row.team_id
                        ),
                        team_id: row.team_id.clone(),
                        competition_id: competition.id.clone(),
                        season_id: season.id.clone(),
                        age_group: season
                            .age_group
                            .clone()
                            .or_else(|| competition.age_group.clone()),
                        group_id: Some(group.id.clone()),
                        division_type: Some(competition.kind.clone()),
                        source_url: None,
                        source_title: Some(
                            "Owner-supplied offline reference application".to_string(),
                        ),
                        source_publisher: Some("Legacy import".to_string()),
                        source_date: Some(captured.clone()),
                        verification_status: "discovered_unverified".to_string(),
                        first_seen_at: captured.clone(),
                        last_seen_at: captured,
                        notes: "Imported team participation; not independently verified."
                            .to_string(),
                        displayed_season: None,
                        source_season: None,
                        roster_status: None,
                        source_checked_at: None,
                    });
                }
            }
        }
    }

    let mut team_list: Vec<Team> = teams.into_values().collect();
    team_list.sort_by(|a, b| a.team_id.cmp(&b.team_id));
    TeamRegistry {
        generated_at: Utc::now().to_rfc3339(),
        source_registry_version: 1,
        teams: team_list,
        participations,
        rosters: Vec::new(),
    }
}

pub fn apply_curated(registry: &mut TeamRegistry, curated: &CuratedTeamFile) {
    let mut team_index: HashMap<String, usize> = registry
        .teams
        .iter()
        .enumerate()
        .map(|(index, team)| (team.team_id.clone(), index))
        .collect();
    for curated_team in &curated.teams {
        if let Some(index) = team_index.get(&curated_team.team_id).copied() {
            let existing = &mut registry.teams[index];
            existing.display_name = curated_team.display_name.clone();
            existing.thai_name = curated_team
                .thai_name
                .clone()
                .or_else(|| existing.thai_name.clone());
            existing.english_name = curated_team
                .english_name
                .clone()
                .or_else(|| existing.english_name.clone());
            for alias in &curated_team.aliases {
                if !existing.aliases.contains(alias) {
                    existing.aliases.push(alias.clone());
                }
            }
            existing.organization_type = curated_team.organization_type.clone();
            existing.province = curated_team
                .province
                .clone()
                .or_else(|| existing.province.clone());
            existing.verification_status = curated_team.verification_status.clone();
            existing.last_seen_at = curated_team.last_seen_at.clone();
            existing.notes = curated_team.notes.clone();
        } else {
            registry.teams.push(curated_team.clone());
            team_index.insert(curated_team.team_id.clone(), registry.teams.len() - 1);
        }
    }

    let mut participation_index: HashMap<String, usize> = registry
        .participations
        .iter()
        .enumerate()
        .map(|(index, participation)| {
            (
                participation_key(
                    &participation.team_id,
                    &participation.competition_id,
                    &participation.season_id,
                    participation.group_id.as_deref(),
                ),
                index,
            )
        })
        .collect();
    for curated_participation in &curated.participations {
        let key = participation_key(
            &curated_participation.team_id,
            &curated_participation.competition_id,
            &curated_participation.season_id,
            curated_participation.group_id.as_deref(),
        );
        if let Some(index) = participation_index.get(&key).copied() {
            registry.participations[index] = curated_participation.clone();
        } else {
            registry.participations.push(curated_participation.clone());
            participation_index.insert(key, registry.participations.len() - 1);
        }
    }

    let mut roster_index: HashMap<(String, String), usize> = registry
        .rosters
        .iter()
        .enumerate()
        .map(|(index, roster)| {
            (
                (roster.competition_id.clone(), roster.season_id.clone()),
                index,
            )
        })
        .collect();
    for curated_roster in &curated.rosters {
        let key = (
            curated_roster.competition_id.clone(),
            curated_roster.season_id.clone(),
        );
        if let Some(index) = roster_index.get(&key).copied() {
            registry.rosters[index] = curated_roster.clone();
        } else {
            registry.rosters.push(curated_roster.clone());
            roster_index.insert(key, registry.rosters.len() - 1);
        }
    }
}

fn participation_key(
    team_id: &str,
    competition_id: &str,
    season_id: &str,
    group_id: Option<&str>,
) -> String {
    format!(
        "{}|{}|{}|{}",
        team_id,
        competition_id,
        season_id,
        group_id.unwrap_or("overall")
    )
}

pub fn validate(
    seed: &Seed,
    registry: &TeamRegistry,
    source_registry: &SourceRegistry,
    news: &[DiscoveryNews],
    fetches: &[FetchRecord],
) -> ValidationReport {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    let active: HashMap<&str, &Competition> = seed
        .competitions
        .iter()
        .filter(|competition| competition.active)
        .map(|competition| (competition.id.as_str(), competition))
        .collect();
    let known_competition_ids: HashSet<&str> = seed
        .competitions
        .iter()
        .chain(seed.legacy.iter())
        .map(|competition| competition.id.as_str())
        .collect();
    let mut nav_ids = HashSet::new();
    for section in &seed.nav {
        for item in &section.items {
            if let Some(id) = &item.id {
                nav_ids.insert(id.clone());
            }
            for child in &item.children {
                if let Some(id) = &child.id {
                    nav_ids.insert(id.clone());
                }
            }
        }
    }

    let mut team_ids = HashSet::new();
    for team in &registry.teams {
        if !team_ids.insert(team.team_id.clone()) {
            errors.push(format!("duplicate stable team_id: {}", team.team_id));
        }
        if team.display_name.trim().is_empty() {
            errors.push(format!("team {} has empty display_name", team.team_id));
        }
    }
    let mut participation_ids = HashSet::new();
    let mut participation_keys = HashSet::new();
    for participation in &registry.participations {
        if !participation_ids.insert(participation.participation_id.clone()) {
            errors.push(format!(
                "duplicate participation_id: {}",
                participation.participation_id
            ));
        }
        let key = participation_key(
            &participation.team_id,
            &participation.competition_id,
            &participation.season_id,
            participation.group_id.as_deref(),
        );
        if !participation_keys.insert(key) {
            errors.push(format!(
                "duplicate participation for {} / {} / {}",
                participation.team_id, participation.competition_id, participation.season_id
            ));
        }
        if !team_ids.contains(&participation.team_id) {
            errors.push(format!(
                "participation references missing team: {}",
                participation.team_id
            ));
        }
        let Some(competition) = active.get(participation.competition_id.as_str()) else {
            errors.push(format!(
                "participation references inactive/unknown competition: {}",
                participation.competition_id
            ));
            continue;
        };
        let Some(season) = competition
            .seasons
            .iter()
            .find(|season| season.id == participation.season_id)
        else {
            errors.push(format!(
                "participation references missing season: {} / {}",
                participation.competition_id, participation.season_id
            ));
            continue;
        };
        if let Some(expected_age) = season.age_group.as_deref()
            && participation.age_group.as_deref() != Some(expected_age)
        {
            errors.push(format!(
                "age mismatch: {} / {} expected {}, got {:?}",
                participation.competition_id, season.id, expected_age, participation.age_group
            ));
        }
        if participation.verification_status.starts_with("verified")
            && participation.source_url.is_none()
        {
            errors.push(format!(
                "verified participation lacks source_url: {}",
                participation.participation_id
            ));
        }
    }

    let valid_roster_statuses = ["current", "latest_verified", "partial", "unavailable"];
    let mut roster_keys = HashSet::new();
    for roster in &registry.rosters {
        let key = format!("{}|{}", roster.competition_id, roster.season_id);
        if !roster_keys.insert(key) {
            errors.push(format!(
                "duplicate roster metadata for {} / {}",
                roster.competition_id, roster.season_id
            ));
        }
        let Some(competition) = active.get(roster.competition_id.as_str()) else {
            errors.push(format!(
                "roster metadata references inactive/unknown competition: {}",
                roster.competition_id
            ));
            continue;
        };
        let Some(season) = competition
            .seasons
            .iter()
            .find(|season| season.id == roster.season_id)
        else {
            errors.push(format!(
                "roster metadata references missing season: {} / {}",
                roster.competition_id, roster.season_id
            ));
            continue;
        };
        if !valid_roster_statuses.contains(&roster.roster_status.as_str()) {
            errors.push(format!(
                "invalid roster_status '{}' for {} / {}",
                roster.roster_status, roster.competition_id, roster.season_id
            ));
        }
        if roster.displayed_season.trim().is_empty()
            || roster.source_season.trim().is_empty()
            || roster.source_checked_at.trim().is_empty()
        {
            errors.push(format!(
                "roster metadata missing season/check date for {} / {}",
                roster.competition_id, roster.season_id
            ));
        }
        if roster.roster_status != "unavailable" && roster.source_url.is_none() {
            errors.push(format!(
                "roster metadata lacks source_url for {} / {}",
                roster.competition_id, roster.season_id
            ));
        }
        let current_season_year: String = season.label.chars().take(4).collect();
        if roster.roster_status == "latest_verified"
            && roster.displayed_season == current_season_year
        {
            errors.push(format!(
                "latest_verified roster for {} / {} is labelled as the current season {}",
                roster.competition_id, roster.season_id, current_season_year
            ));
        }
        let participation_count = registry
            .participations
            .iter()
            .filter(|participation| {
                participation.competition_id == roster.competition_id
                    && participation.season_id == roster.season_id
            })
            .count();
        if roster.team_count != participation_count {
            errors.push(format!(
                "roster team_count {} does not match {} participations for {} / {}",
                roster.team_count, participation_count, roster.competition_id, roster.season_id
            ));
        }
    }

    for competition in active.values() {
        if competition.id.starts_with("thai-school-league-") {
            let allowed: &[&str] = if competition.id == "thai-school-league-general" {
                &["U18", "U16", "U14", "U12"]
            } else {
                &["U18", "U16", "U14"]
            };
            for season in &competition.seasons {
                if let Some(age) = season.age_group.as_deref()
                    && !allowed.contains(&age)
                {
                    errors.push(format!(
                        "school age {} not allowed for {}",
                        age, competition.id
                    ));
                }
            }
        }
        if competition.id == "thai-college-cup-u19"
            && (!competition.name.contains("U19") || !competition.section.contains("School League"))
        {
            errors.push("Thai College Cup must be U19 under School League".to_string());
        }
        if competition.id == "thai-university-cup-u23"
            && (!competition.name.contains("U23") || !competition.section.contains("School League"))
        {
            errors.push("Thai University Cup must be U23 under School League".to_string());
        }
    }
    for item in news {
        if let Some(competition_id) = item.competition_id.as_deref()
            && !known_competition_ids.contains(competition_id)
        {
            errors.push(format!(
                "news {} references unknown competition {}",
                item.news_id, competition_id
            ));
        }
        for team_id in &item.extracted_team_ids {
            if !team_ids.contains(team_id) {
                errors.push(format!(
                    "news {} references missing team {}",
                    item.news_id, team_id
                ));
            }
        }
    }
    for legacy in &seed.legacy {
        if legacy.active {
            errors.push(format!("legacy competition became active: {}", legacy.id));
        }
    }
    if let Some(super_cup) = seed
        .competitions
        .iter()
        .find(|competition| competition.name.to_lowercase().contains("school super cup"))
        && (!super_cup.active
            || !super_cup.section.contains("School League")
            || super_cup.age_group.as_deref() != Some("U23"))
    {
        errors.push("Thai School Super Cup must be active U23 under School League".to_string());
    }
    for item in source_registry.competitions.iter() {
        if !active.contains_key(item.competition_id.as_str()) {
            errors.push(format!(
                "source registry references non-active competition: {}",
                item.competition_id
            ));
        } else if let Some(competition) = active.get(item.competition_id.as_str())
            && item.organizer_group != competition.section
        {
            errors.push(format!(
                "source registry organizer mismatch for {}: {} != {}",
                item.competition_id, item.organizer_group, competition.section
            ));
        }
        if !matches!(
            item.source_priority.as_str(),
            "official_only" | "official_then_secondary" | "secondary_only"
        ) {
            errors.push(format!(
                "invalid source priority for {}: {}",
                item.competition_id, item.source_priority
            ));
        }
        if item.update_status.trim().is_empty() {
            warnings.push(format!("missing update status for {}", item.competition_id));
        }
    }
    for id in active.keys() {
        if !nav_ids.contains(*id) {
            errors.push(format!("active competition missing from sidebar: {id}"));
        }
    }
    for id in &nav_ids {
        if !active.contains_key(id.as_str()) {
            errors.push(format!("sidebar references inactive competition: {id}"));
        }
    }

    let mut verified_official = 0;
    let mut verified_secondary = 0;
    let mut discovered_unverified = 0;
    let mut conflicting = 0;
    let mut rejected = 0;
    for participation in &registry.participations {
        match participation.verification_status.as_str() {
            "verified_official" => verified_official += 1,
            "verified_secondary" => verified_secondary += 1,
            "discovered_unverified" => discovered_unverified += 1,
            "conflicting" => conflicting += 1,
            "rejected" => rejected += 1,
            _ => warnings.push(format!(
                "unknown verification status: {}",
                participation.verification_status
            )),
        }
    }

    let mut competition_summary = Vec::new();
    for competition in active.values() {
        let rows: Vec<&TeamParticipation> = registry
            .participations
            .iter()
            .filter(|participation| participation.competition_id == competition.id)
            .collect();
        competition_summary.push(CompetitionSummary {
            competition_id: competition.id.clone(),
            display_name: competition.name.clone(),
            organizer_group: competition.section.clone(),
            teams: rows.len(),
            verified_official: rows
                .iter()
                .filter(|row| row.verification_status == "verified_official")
                .count(),
            verified_secondary: rows
                .iter()
                .filter(|row| row.verification_status == "verified_secondary")
                .count(),
            discovered_unverified: rows
                .iter()
                .filter(|row| row.verification_status == "discovered_unverified")
                .count(),
        });
    }
    competition_summary.sort_by(|a, b| a.display_name.cmp(&b.display_name));

    let counts = ValidationCounts {
        teams: registry.teams.len(),
        participations: registry.participations.len(),
        verified_official,
        verified_secondary,
        discovered_unverified,
        conflicting,
        rejected,
        news_items: news.len(),
        team_candidates: fetches.iter().map(|fetch| fetch.candidate_count).sum(),
        fetched_sources: fetches.iter().filter(|fetch| fetch.error.is_none()).count(),
        failed_sources: fetches.iter().filter(|fetch| fetch.error.is_some()).count(),
    };

    ValidationReport {
        generated_at: Utc::now().to_rfc3339(),
        errors,
        warnings,
        counts,
        competition_summary,
    }
}

pub fn build_news_metadata(
    seed: &Seed,
    registry: &TeamRegistry,
    discovered: Vec<DiscoveryNews>,
) -> Vec<DiscoveryNews> {
    let now = Utc::now().to_rfc3339();
    let mut items = Vec::new();
    let mut seen = HashSet::new();
    for article in &seed.news {
        let competition_id = article.competition_ids.first().cloned();
        let competition = competition_id.as_deref().and_then(|id| {
            seed.competitions
                .iter()
                .find(|competition| competition.id == id)
        });
        let season_id = competition
            .and_then(|competition| competition.seasons.iter().find(|season| season.is_current))
            .map(|season| season.id.clone());
        let age_group = competition.and_then(|competition| competition.age_group.clone());
        let text = format!("{} {}", article.title, article.summary);
        let extracted_team_ids = extract_team_ids(&text, registry);
        let title_key = article.title.to_lowercase();
        if seen.insert(title_key) {
            items.push(DiscoveryNews {
                news_id: article.id.clone(),
                title: article.title.clone(),
                source_name: article.source_name.clone(),
                source_url: None,
                published_at: article.published_label.clone(),
                competition_id,
                season_id,
                age_group,
                extracted_team_ids,
                verification_status: "discovered_unverified".to_string(),
                summary: article.summary.clone(),
                discovered_at: now.clone(),
            });
        }
    }
    for mut item in discovered {
        let key = item
            .source_url
            .clone()
            .unwrap_or_else(|| item.title.to_lowercase());
        if item.extracted_team_ids.is_empty() {
            item.extracted_team_ids =
                extract_team_ids(&format!("{} {}", item.title, item.summary), registry);
        }
        if seen.insert(key) {
            items.push(item);
        }
    }
    items
}

pub fn extract_team_ids(text: &str, registry: &TeamRegistry) -> Vec<String> {
    let haystack = text.to_lowercase();
    let mut ids = Vec::new();
    for team in &registry.teams {
        let mut names = vec![team.display_name.clone()];
        if let Some(name) = &team.english_name {
            names.push(name.clone());
        }
        names.extend(team.aliases.iter().cloned());
        if names.iter().any(|name| {
            let normalized = name.trim().to_lowercase();
            normalized.len() >= 5 && haystack.contains(&normalized)
        }) {
            ids.push(team.team_id.clone());
        }
    }
    ids.sort();
    ids.dedup();
    ids
}

pub struct FetchResult {
    pub record: FetchRecord,
    pub rss_news: Vec<DiscoveryNews>,
}

pub async fn fetch_source(
    client: &reqwest::Client,
    competition: &RegistryCompetition,
    url: &str,
) -> FetchResult {
    let fetched_at = Utc::now().to_rfc3339();
    let response = client.get(url).send().await;
    match response {
        Ok(response) => {
            let http_status = Some(response.status().as_u16());
            let content_type = response
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|value| value.to_str().ok())
                .map(ToString::to_string);
            match response.text().await {
                Ok(body) => {
                    let sha256: String = Sha256::digest(body.as_bytes())
                        .iter()
                        .map(|byte| format!("{byte:02x}"))
                        .collect();
                    let title = extract_title(&body);
                    let candidates = extract_candidates(&body);
                    let rss_news = if url.contains("/rss") {
                        parse_rss_news(&body, competition)
                    } else {
                        Vec::new()
                    };
                    FetchResult {
                        record: FetchRecord {
                            competition_id: competition.competition_id.clone(),
                            url: url.to_string(),
                            fetched_at,
                            status: if http_status == Some(200) {
                                "ok".to_string()
                            } else {
                                "http_error".to_string()
                            },
                            http_status,
                            content_type,
                            title,
                            bytes: body.len(),
                            sha256: Some(sha256),
                            candidate_count: candidates.len(),
                            candidates,
                            error: None,
                        },
                        rss_news,
                    }
                }
                Err(error) => FetchResult {
                    record: FetchRecord {
                        competition_id: competition.competition_id.clone(),
                        url: url.to_string(),
                        fetched_at,
                        status: "body_error".to_string(),
                        http_status,
                        content_type,
                        title: None,
                        bytes: 0,
                        sha256: None,
                        candidate_count: 0,
                        candidates: Vec::new(),
                        error: Some(error.to_string()),
                    },
                    rss_news: Vec::new(),
                },
            }
        }
        Err(error) => FetchResult {
            record: FetchRecord {
                competition_id: competition.competition_id.clone(),
                url: url.to_string(),
                fetched_at,
                status: "request_error".to_string(),
                http_status: None,
                content_type: None,
                title: None,
                bytes: 0,
                sha256: None,
                candidate_count: 0,
                candidates: Vec::new(),
                error: Some(error.to_string()),
            },
            rss_news: Vec::new(),
        },
    }
}

fn extract_title(html: &str) -> Option<String> {
    let title_re = Regex::new(r"(?is)<title[^>]*>(.*?)</title>").ok()?;
    let captures = title_re.captures(html)?;
    Some(clean_text(&captures[1]))
}

pub fn extract_candidates(html: &str) -> Vec<String> {
    let Ok(tag_re) =
        Regex::new(r"(?is)<(?:li|td|th|h1|h2|h3|h4)[^>]*>(.*?)</(?:li|td|th|h1|h2|h3|h4)>")
    else {
        return Vec::new();
    };
    let Ok(tag_strip_re) = Regex::new(r"(?is)<[^>]+>") else {
        return Vec::new();
    };
    let team_word_re = Regex::new(
        r"(?i)(F\.?C\.?|United|City|Town|School|College|University|Academy|Sports|Club|Res\.|U1[2468]|วิทยาลัย|มหาวิทยาลัย|โรงเรียน|สมาคม|ทีม)",
    )
    .ok();
    let mut candidates = Vec::new();
    for captures in tag_re.captures_iter(html) {
        let text = clean_text(&tag_strip_re.replace_all(&captures[1], " "));
        if text.len() < 3 || text.len() > 80 {
            continue;
        }
        if text
            .chars()
            .filter(|character| character.is_alphabetic())
            .count()
            < 3
        {
            continue;
        }
        if text.contains("http") || text.contains("Wikipedia") || text.contains("edit") {
            continue;
        }
        if let Some(team_word_re) = &team_word_re
            && !team_word_re.is_match(&text)
        {
            continue;
        }
        if !candidates.contains(&text) {
            candidates.push(text);
        }
        if candidates.len() >= 200 {
            break;
        }
    }
    candidates
}

fn parse_rss_news(xml: &str, competition: &RegistryCompetition) -> Vec<DiscoveryNews> {
    let Ok(item_re) = Regex::new(r"(?is)<item[^>]*>(.*?)</item>") else {
        return Vec::new();
    };
    let Ok(title_re) = Regex::new(r"(?is)<title[^>]*>(.*?)</title>") else {
        return Vec::new();
    };
    let Ok(link_re) = Regex::new(r"(?is)<link[^>]*>(.*?)</link>") else {
        return Vec::new();
    };
    let Ok(date_re) = Regex::new(r"(?is)<pubDate[^>]*>(.*?)</pubDate>") else {
        return Vec::new();
    };
    let Ok(source_re) = Regex::new(r"(?is)<source[^>]*>(.*?)</source>") else {
        return Vec::new();
    };
    let now = Utc::now().to_rfc3339();
    let mut items = Vec::new();
    for item in item_re.captures_iter(xml).take(10) {
        let body = &item[1];
        let title = title_re
            .captures(body)
            .map(|capture| clean_text(&capture[1]))
            .unwrap_or_default();
        if title.is_empty() {
            continue;
        }
        let link = link_re
            .captures(body)
            .map(|capture| clean_text(&capture[1]))
            .unwrap_or_default();
        let published = date_re
            .captures(body)
            .map(|capture| clean_text(&capture[1]))
            .unwrap_or_else(|| "unknown".to_string());
        let source_name = source_re
            .captures(body)
            .map(|capture| clean_text(&capture[1]))
            .unwrap_or_else(|| "Google News RSS".to_string());
        items.push(DiscoveryNews {
            news_id: format!("rss-{}-{}", competition.competition_id, items.len() + 1),
            title,
            source_name,
            source_url: if link.is_empty() { None } else { Some(link) },
            published_at: published,
            competition_id: Some(competition.competition_id.clone()),
            season_id: None,
            age_group: competition.age_group.clone(),
            extracted_team_ids: Vec::new(),
            verification_status: "discovered_unverified".to_string(),
            summary: "Headline discovered via a public news RSS feed; article text was not copied."
                .to_string(),
            discovered_at: now.clone(),
        });
    }
    items
}

fn clean_text(value: &str) -> String {
    value
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string()
}

pub fn load_json<T: for<'de> Deserialize<'de>>(path: &std::path::Path) -> Result<T> {
    let raw =
        std::fs::read_to_string(path).with_context(|| format!("cannot read {}", path.display()))?;
    serde_json::from_str(&raw).with_context(|| format!("cannot parse {}", path.display()))
}
