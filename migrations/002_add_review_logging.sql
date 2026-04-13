-- Migration 002: Add review logging and relearning tracking
-- Add is_relearning column to srs_cards table
ALTER TABLE srs_cards ADD COLUMN IF NOT EXISTS is_relearning BOOLEAN NOT NULL DEFAULT FALSE;

-- Create review_log table for tracking daily activity and statistics
CREATE TABLE IF NOT EXISTS review_log (
    id SERIAL PRIMARY KEY,
    word_id INTEGER NOT NULL,
    rating INTEGER NOT NULL CHECK (rating >= 1 AND rating <= 4),
    reviewed_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    previous_box_level SMALLINT,
    new_box_level SMALLINT,
    FOREIGN KEY (word_id) REFERENCES srs_cards(word_id) ON DELETE CASCADE
);

-- Create indexes for efficient querying
CREATE INDEX IF NOT EXISTS idx_review_log_word_id ON review_log(word_id);
CREATE INDEX IF NOT EXISTS idx_review_log_reviewed_at ON review_log(reviewed_at);
CREATE INDEX IF NOT EXISTS idx_review_log_rating ON review_log(rating);

-- Update the updated_at timestamp for existing records
UPDATE srs_cards SET updated_at = CURRENT_TIMESTAMP WHERE updated_at IS NULL;