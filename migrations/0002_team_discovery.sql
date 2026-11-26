-- Team-first discovery metadata and news provenance.

ALTER TABLE teams ADD COLUMN IF NOT EXISTS thai_name TEXT;
ALTER TABLE teams ADD COLUMN IF NOT EXISTS english_name TEXT;
ALTER TABLE teams ADD COLUMN IF NOT EXISTS organization_type TEXT NOT NULL DEFAULT 'unknown';
ALTER TABLE teams ADD COLUMN IF NOT EXISTS province TEXT;
ALTER TABLE teams ADD COLUMN IF NOT EXISTS first_seen_at TEXT;
ALTER TABLE teams ADD COLUMN IF NOT EXISTS last_seen_at TEXT;

ALTER TABLE participations ADD COLUMN IF NOT EXISTS age_group TEXT;
ALTER TABLE participations ADD COLUMN IF NOT EXISTS division_type TEXT;
ALTER TABLE participations ADD COLUMN IF NOT EXISTS source_url TEXT;
ALTER TABLE participations ADD COLUMN IF NOT EXISTS source_title TEXT;
ALTER TABLE participations ADD COLUMN IF NOT EXISTS source_publisher TEXT;
ALTER TABLE participations ADD COLUMN IF NOT EXISTS source_date TEXT;
ALTER TABLE participations ADD COLUMN IF NOT EXISTS verification_status TEXT NOT NULL DEFAULT 'discovered_unverified';
ALTER TABLE participations ADD COLUMN IF NOT EXISTS first_seen_at TEXT;
ALTER TABLE participations ADD COLUMN IF NOT EXISTS last_seen_at TEXT;
ALTER TABLE participations ADD COLUMN IF NOT EXISTS notes TEXT NOT NULL DEFAULT '';

CREATE TABLE IF NOT EXISTS discovery_news (
    news_id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    source_name TEXT NOT NULL,
    source_url TEXT,
    published_at TEXT NOT NULL,
    competition_id TEXT REFERENCES competitions(id) ON DELETE SET NULL,
    season_key TEXT REFERENCES seasons(season_key) ON DELETE SET NULL,
    age_group TEXT,
    extracted_team_ids TEXT[] NOT NULL DEFAULT '{}',
    verification_status TEXT NOT NULL DEFAULT 'discovered_unverified',
    summary TEXT NOT NULL DEFAULT '',
    discovered_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_participations_season_group
    ON participations(season_key, group_key);
CREATE INDEX IF NOT EXISTS idx_participations_verification
    ON participations(verification_status);
CREATE INDEX IF NOT EXISTS idx_discovery_news_competition
    ON discovery_news(competition_id);
