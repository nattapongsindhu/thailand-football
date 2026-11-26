-- Separate extra-time marker for cup ties whose result was decided after
-- 90 minutes. Scores and penalty shootouts remain in their own columns.
ALTER TABLE cup_ties ADD COLUMN IF NOT EXISTS extra_time BOOLEAN NOT NULL DEFAULT FALSE;
