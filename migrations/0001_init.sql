-- Thailand Football — initial relational schema.
-- Source snapshots are preserved; imported rows remain explicitly unverified.

CREATE TABLE IF NOT EXISTS sources (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    url TEXT,
    retrieved_at TEXT NOT NULL,
    kind TEXT NOT NULL,
    note TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS competitions (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    short_name TEXT NOT NULL,
    section TEXT NOT NULL,
    category TEXT NOT NULL,
    kind TEXT NOT NULL CHECK (kind IN ('league', 'cup')),
    age_group TEXT,
    sort_order INTEGER NOT NULL,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    official_name TEXT NOT NULL,
    group_label TEXT,
    description TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS competition_aliases (
    competition_id TEXT NOT NULL REFERENCES competitions(id) ON DELETE CASCADE,
    alias TEXT NOT NULL,
    PRIMARY KEY (competition_id, alias)
);

CREATE TABLE IF NOT EXISTS competition_sources (
    competition_id TEXT NOT NULL REFERENCES competitions(id) ON DELETE CASCADE,
    source_id TEXT NOT NULL REFERENCES sources(id) ON DELETE CASCADE,
    PRIMARY KEY (competition_id, source_id)
);

CREATE TABLE IF NOT EXISTS seasons (
    season_key TEXT PRIMARY KEY,
    competition_id TEXT NOT NULL REFERENCES competitions(id) ON DELETE CASCADE,
    season_id TEXT NOT NULL,
    label TEXT NOT NULL,
    is_current BOOLEAN NOT NULL DEFAULT FALSE,
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    captured_at TEXT NOT NULL,
    age_group TEXT,
    UNIQUE (competition_id, season_id)
);

CREATE TABLE IF NOT EXISTS season_sources (
    season_key TEXT NOT NULL REFERENCES seasons(season_key) ON DELETE CASCADE,
    source_id TEXT NOT NULL REFERENCES sources(id) ON DELETE CASCADE,
    PRIMARY KEY (season_key, source_id)
);

CREATE TABLE IF NOT EXISTS season_groups (
    group_key TEXT PRIMARY KEY,
    season_key TEXT NOT NULL REFERENCES seasons(season_key) ON DELETE CASCADE,
    group_id TEXT NOT NULL,
    name TEXT NOT NULL,
    sort_order INTEGER NOT NULL DEFAULT 0,
    validation_status TEXT NOT NULL DEFAULT 'unknown',
    validation_messages TEXT[] NOT NULL DEFAULT '{}',
    UNIQUE (season_key, group_id)
);

CREATE TABLE IF NOT EXISTS teams (
    id TEXT PRIMARY KEY,
    canonical_name TEXT NOT NULL,
    identity_status TEXT NOT NULL DEFAULT 'unresolved',
    notes TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS team_aliases (
    team_id TEXT NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    alias TEXT NOT NULL,
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (team_id, alias)
);

CREATE TABLE IF NOT EXISTS participations (
    season_key TEXT NOT NULL REFERENCES seasons(season_key) ON DELETE CASCADE,
    group_key TEXT NOT NULL REFERENCES season_groups(group_key) ON DELETE CASCADE,
    team_id TEXT NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    display_name TEXT NOT NULL,
    sort_order INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (season_key, group_key, team_id)
);

CREATE TABLE IF NOT EXISTS standings (
    season_key TEXT NOT NULL REFERENCES seasons(season_key) ON DELETE CASCADE,
    group_key TEXT NOT NULL REFERENCES season_groups(group_key) ON DELETE CASCADE,
    team_id TEXT NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    captured_at TEXT NOT NULL,
    rank INTEGER NOT NULL,
    played INTEGER NOT NULL,
    won INTEGER NOT NULL,
    drawn INTEGER NOT NULL,
    lost INTEGER NOT NULL,
    goals_for INTEGER NOT NULL,
    goals_against INTEGER NOT NULL,
    goal_difference INTEGER NOT NULL,
    points INTEGER NOT NULL,
    form TEXT[] NOT NULL DEFAULT '{}',
    zone TEXT NOT NULL DEFAULT '',
    PRIMARY KEY (season_key, group_key, team_id, captured_at)
);

CREATE TABLE IF NOT EXISTS matches (
    id TEXT PRIMARY KEY,
    season_key TEXT NOT NULL REFERENCES seasons(season_key) ON DELETE CASCADE,
    group_key TEXT REFERENCES season_groups(group_key) ON DELETE SET NULL,
    kind TEXT NOT NULL CHECK (kind IN ('result', 'fixture')),
    round_label TEXT NOT NULL,
    date_label TEXT NOT NULL,
    home_team_id TEXT REFERENCES teams(id),
    away_team_id TEXT REFERENCES teams(id),
    home_name TEXT NOT NULL,
    away_name TEXT NOT NULL,
    home_goals INTEGER,
    away_goals INTEGER,
    status TEXT NOT NULL DEFAULT '',
    note TEXT NOT NULL DEFAULT '',
    resolution TEXT NOT NULL DEFAULT 'unresolved',
    issues TEXT[] NOT NULL DEFAULT '{}'
);

CREATE TABLE IF NOT EXISTS cup_editions (
    season_key TEXT PRIMARY KEY REFERENCES seasons(season_key) ON DELETE CASCADE,
    name TEXT NOT NULL,
    sponsor TEXT NOT NULL DEFAULT '',
    teams_count INTEGER,
    prize TEXT NOT NULL DEFAULT '',
    reward TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS cup_rounds (
    id TEXT PRIMARY KEY,
    season_key TEXT NOT NULL REFERENCES seasons(season_key) ON DELETE CASCADE,
    name TEXT NOT NULL,
    date_label TEXT NOT NULL DEFAULT '',
    clubs INTEGER,
    status TEXT NOT NULL DEFAULT '',
    note TEXT NOT NULL DEFAULT '',
    sort_order INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS cup_ties (
    id TEXT PRIMARY KEY,
    season_key TEXT NOT NULL REFERENCES seasons(season_key) ON DELETE CASCADE,
    round_id TEXT REFERENCES cup_rounds(id) ON DELETE SET NULL,
    stage TEXT NOT NULL,
    home_name TEXT NOT NULL,
    away_name TEXT NOT NULL,
    home_goals INTEGER,
    away_goals INTEGER,
    status TEXT NOT NULL DEFAULT '',
    note TEXT NOT NULL DEFAULT '',
    sort_order INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS cup_past_editions (
    id TEXT PRIMARY KEY,
    season_key TEXT NOT NULL REFERENCES seasons(season_key) ON DELETE CASCADE,
    year TEXT NOT NULL,
    home_name TEXT NOT NULL,
    away_name TEXT NOT NULL,
    home_goals INTEGER,
    away_goals INTEGER,
    note TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS scorers (
    id TEXT PRIMARY KEY,
    season_key TEXT NOT NULL REFERENCES seasons(season_key) ON DELETE CASCADE,
    team_id TEXT REFERENCES teams(id),
    rank INTEGER NOT NULL,
    player_name TEXT NOT NULL,
    team_name TEXT NOT NULL,
    nationality TEXT NOT NULL DEFAULT '',
    goals INTEGER NOT NULL,
    appearances INTEGER NOT NULL,
    assists INTEGER NOT NULL,
    resolution TEXT NOT NULL DEFAULT 'unresolved',
    issues TEXT[] NOT NULL DEFAULT '{}'
);

CREATE TABLE IF NOT EXISTS news (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    summary TEXT NOT NULL,
    published_label TEXT NOT NULL,
    source_name TEXT NOT NULL,
    tag TEXT NOT NULL,
    emoji TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS news_competitions (
    news_id TEXT NOT NULL REFERENCES news(id) ON DELETE CASCADE,
    competition_id TEXT NOT NULL REFERENCES competitions(id) ON DELETE CASCADE,
    PRIMARY KEY (news_id, competition_id)
);

CREATE INDEX IF NOT EXISTS idx_seasons_competition ON seasons(competition_id);
CREATE INDEX IF NOT EXISTS idx_standings_season_group ON standings(season_key, group_key);
CREATE INDEX IF NOT EXISTS idx_matches_season_kind ON matches(season_key, kind);
CREATE INDEX IF NOT EXISTS idx_teams_name ON teams(canonical_name);

CREATE TABLE IF NOT EXISTS app_meta (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
