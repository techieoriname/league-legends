-- Your SQL goes here
CREATE TABLE leagues (
                         id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                         name VARCHAR(255) NOT NULL,
                         creator_id UUID NOT NULL REFERENCES users(id),
                         created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                         updated_at TIMESTAMP
);

CREATE INDEX idx_leagues_name ON leagues(name);
