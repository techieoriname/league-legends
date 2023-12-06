-- Your SQL goes here
CREATE TABLE teams (
                       id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                       league_id UUID NOT NULL REFERENCES leagues(id),
                       user_id UUID NOT NULL REFERENCES users(id),
                       name VARCHAR(255) NOT NULL,
                       created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                       updated_at TIMESTAMP
);

CREATE INDEX idx_teams_name ON teams(name);
