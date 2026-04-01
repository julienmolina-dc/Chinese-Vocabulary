-- Create srs_cards table for spaced repetition tracking
CREATE TABLE IF NOT EXISTS srs_cards (
    word_id INTEGER PRIMARY KEY,
    ease_factor FLOAT NOT NULL DEFAULT 2.5,
    interval INTEGER NOT NULL DEFAULT 0,
    repetitions INTEGER NOT NULL DEFAULT 0,
    next_review BIGINT NOT NULL DEFAULT 0,
    box_level SMALLINT NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create index on next_review for efficient querying of due cards
CREATE INDEX IF NOT EXISTS idx_next_review ON srs_cards(next_review);
CREATE INDEX IF NOT EXISTS idx_box_level ON srs_cards(box_level);
