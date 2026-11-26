-- Structured tie metadata for cups whose public records have unavailable
-- scores, separately stored penalty shootouts, or inferred winners.
ALTER TABLE cup_ties ADD COLUMN IF NOT EXISTS score_status TEXT;
ALTER TABLE cup_ties ADD COLUMN IF NOT EXISTS home_penalties INTEGER;
ALTER TABLE cup_ties ADD COLUMN IF NOT EXISTS away_penalties INTEGER;
ALTER TABLE cup_ties ADD COLUMN IF NOT EXISTS winner_name TEXT;
ALTER TABLE cup_ties ADD COLUMN IF NOT EXISTS winner_basis TEXT;
