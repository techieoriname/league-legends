-- Your SQL goes here
CREATE TABLE challenges (
                            id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                            challenger_id UUID NOT NULL REFERENCES users(id),
                            opponent_id UUID NOT NULL REFERENCES users(id),
                            match_date TIMESTAMP NOT NULL,
                            status VARCHAR(50) NOT NULL, -- e.g., "scheduled", "completed", "accepted", "declined"
                            result TEXT, -- Details about the result or scoring
                            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                            updated_at TIMESTAMP
);

CREATE INDEX idx_challenges_match_date ON challenges(match_date);
