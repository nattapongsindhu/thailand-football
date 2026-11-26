use std::collections::{HashMap, HashSet};
use std::path::Path;

use anyhow::{Context, Result};

use crate::model::{
    Competition, Group, NewsArticle, RosterMeta, Season, Seed, Source, Team, TeamParticipation,
    TeamRegistry,
};

#[derive(Debug, Clone)]
pub struct SearchEntry {
    pub team_id: String,
    pub team_name: String,
    pub competition_id: String,
    pub competition_name: String,
    pub competition_short: String,
    pub group_id: String,
    pub group_name: String,
    pub season_id: String,
    pub season_label: String,
}

#[derive(Debug, Clone)]
pub struct AppData {
    pub seed: Seed,
    pub competitions: HashMap<String, Competition>,
    pub news: HashMap<String, NewsArticle>,
    pub sources: HashMap<String, Source>,
    pub team_registry: TeamRegistry,
    pub search_index: Vec<SearchEntry>,
}

impl AppData {
    pub fn from_path(path: &Path) -> Result<Self> {
        let raw = std::fs::read_to_string(path)
            .with_context(|| format!("cannot read seed file {}", path.display()))?;
        let seed: Seed = serde_json::from_str(&raw)
            .with_context(|| format!("cannot parse seed JSON {}", path.display()))?;
        let registry_path = path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join("team_registry.json");
        let team_registry = if registry_path.exists() {
            let registry_raw = std::fs::read_to_string(&registry_path)
                .with_context(|| format!("cannot read {}", registry_path.display()))?;
            serde_json::from_str(&registry_raw)
                .with_context(|| format!("cannot parse {}", registry_path.display()))?
        } else {
            TeamRegistry::default()
        };
        Ok(Self::from_seed_with_registry(seed, team_registry))
    }

    pub fn from_seed(seed: Seed) -> Self {
        Self::from_seed_with_registry(seed, TeamRegistry::default())
    }

    pub fn from_seed_with_registry(seed: Seed, team_registry: TeamRegistry) -> Self {
        let competitions: HashMap<String, Competition> = seed
            .competitions
            .iter()
            .chain(seed.legacy.iter())
            .cloned()
            .map(|c| (c.id.clone(), c))
            .collect();
        let news = seed
            .news
            .iter()
            .cloned()
            .map(|n| (n.id.clone(), n))
            .collect();
        let sources = seed
            .sources
            .iter()
            .cloned()
            .map(|s| (s.id.clone(), s))
            .collect();

        let teams_by_id: HashMap<&str, &Team> = team_registry
            .teams
            .iter()
            .map(|team| (team.team_id.as_str(), team))
            .collect();
        let mut search_index = Vec::new();
        let mut seen = HashSet::new();
        for participation in &team_registry.participations {
            let Some(competition) = competitions.get(&participation.competition_id) else {
                continue;
            };
            if !competition.active {
                continue;
            }
            let Some(season) = competition
                .seasons
                .iter()
                .find(|season| season.id == participation.season_id)
            else {
                continue;
            };
            let Some(team) = teams_by_id.get(participation.team_id.as_str()) else {
                continue;
            };
            let group_id = participation
                .group_id
                .clone()
                .unwrap_or_else(|| "overall".to_string());
            let group_name = season
                .groups
                .iter()
                .find(|group| group.id == group_id)
                .map(|group| group.name.clone())
                .unwrap_or_else(|| "Team list".to_string());
            let key = format!(
                "{}|{}|{}|{}",
                team.team_id, competition.id, season.id, group_id
            );
            if !seen.insert(key) {
                continue;
            }
            search_index.push(SearchEntry {
                team_id: team.team_id.clone(),
                team_name: team.display_name.clone(),
                competition_id: competition.id.clone(),
                competition_name: competition.name.clone(),
                competition_short: competition.short_name.clone(),
                group_id,
                group_name,
                season_id: season.id.clone(),
                season_label: season.label.clone(),
            });
        }

        // Fallback for installations that have not yet generated a team registry.
        if search_index.is_empty() {
            for competition in seed.competitions.iter().filter(|c| c.active) {
                for season in &competition.seasons {
                    for group in &season.groups {
                        for row in &group.standings {
                            search_index.push(SearchEntry {
                                team_id: row.team_id.clone(),
                                team_name: row.team_name.clone(),
                                competition_id: competition.id.clone(),
                                competition_name: competition.name.clone(),
                                competition_short: competition.short_name.clone(),
                                group_id: group.id.clone(),
                                group_name: group.name.clone(),
                                season_id: season.id.clone(),
                                season_label: season.label.clone(),
                            });
                        }
                    }
                }
            }
        }
        search_index.sort_by(|a, b| {
            a.team_name
                .to_lowercase()
                .cmp(&b.team_name.to_lowercase())
                .then_with(|| a.competition_name.cmp(&b.competition_name))
                .then_with(|| a.group_name.cmp(&b.group_name))
        });

        Self {
            seed,
            competitions,
            news,
            sources,
            team_registry,
            search_index,
        }
    }

    pub fn competition(&self, id: &str) -> Option<&Competition> {
        self.competitions.get(id)
    }

    pub fn active_competitions(&self) -> impl Iterator<Item = &Competition> {
        self.seed.competitions.iter().filter(|c| c.active)
    }

    pub fn season<'a>(
        &self,
        competition: &'a Competition,
        requested: Option<&str>,
    ) -> Option<&'a Season> {
        requested
            .and_then(|id| competition.seasons.iter().find(|s| s.id == id))
            .or_else(|| competition.seasons.iter().find(|s| s.is_current))
            .or_else(|| competition.seasons.first())
    }

    pub fn news_for(&self, season: &Season) -> Vec<&NewsArticle> {
        season
            .news_ids
            .iter()
            .filter_map(|id| self.news.get(id))
            .collect()
    }

    pub fn sources_for(&self, competition: &Competition, season: &Season) -> Vec<&Source> {
        let mut ids = competition.source_ids.clone();
        ids.extend(season.source_ids.iter().cloned());
        ids.sort();
        ids.dedup();
        ids.iter().filter_map(|id| self.sources.get(id)).collect()
    }

    pub fn group<'a>(&self, season: &'a Season, requested: Option<&str>) -> Option<&'a Group> {
        requested
            .and_then(|id| season.groups.iter().find(|g| g.id == id))
            .or_else(|| season.groups.first())
    }

    pub fn roster_for(&self, competition_id: &str, season_id: &str) -> Option<&RosterMeta> {
        self.team_registry
            .rosters
            .iter()
            .find(|roster| roster.competition_id == competition_id && roster.season_id == season_id)
    }

    pub fn teams_for<'a>(
        &'a self,
        competition_id: &str,
        season_id: &str,
        group_id: Option<&str>,
    ) -> Vec<(&'a Team, &'a TeamParticipation)> {
        let teams_by_id: HashMap<&str, &Team> = self
            .team_registry
            .teams
            .iter()
            .map(|team| (team.team_id.as_str(), team))
            .collect();
        let mut rows: Vec<(&Team, &TeamParticipation)> = self
            .team_registry
            .participations
            .iter()
            .filter(|participation| {
                participation.competition_id == competition_id
                    && participation.season_id == season_id
                    && group_id.is_none_or(|wanted| {
                        participation.group_id.as_deref().unwrap_or("overall") == wanted
                    })
            })
            .filter_map(|participation| {
                teams_by_id
                    .get(participation.team_id.as_str())
                    .map(|team| (*team, participation))
            })
            .collect();
        rows.sort_by(|a, b| {
            a.0.display_name
                .to_lowercase()
                .cmp(&b.0.display_name.to_lowercase())
        });
        rows
    }

    pub fn search(&self, query: &str) -> Vec<&SearchEntry> {
        let q = query.trim().to_lowercase();
        if q.len() < 2 {
            return Vec::new();
        }
        let competition_order = |entry: &SearchEntry| {
            self.competitions
                .get(&entry.competition_id)
                .map(|competition| competition.sort_order)
                .unwrap_or(i32::MAX)
        };
        let is_current_season = |entry: &SearchEntry| {
            self.competitions
                .get(&entry.competition_id)
                .and_then(|competition| {
                    competition
                        .seasons
                        .iter()
                        .find(|season| season.id == entry.season_id)
                })
                .is_some_and(|season| season.is_current)
        };
        let mut matches: Vec<&SearchEntry> = self
            .search_index
            .iter()
            .filter(|entry| entry.team_name.to_lowercase().contains(&q))
            .collect();
        matches.sort_by(|a, b| {
            let a_prefix = a.team_name.to_lowercase().starts_with(&q);
            let b_prefix = b.team_name.to_lowercase().starts_with(&q);
            b_prefix
                .cmp(&a_prefix)
                .then_with(|| competition_order(a).cmp(&competition_order(b)))
                .then_with(|| is_current_season(b).cmp(&is_current_season(a)))
                .then_with(|| a.team_name.to_lowercase().cmp(&b.team_name.to_lowercase()))
                .then_with(|| a.group_name.cmp(&b.group_name))
        });
        matches.truncate(20);
        matches
    }
}
