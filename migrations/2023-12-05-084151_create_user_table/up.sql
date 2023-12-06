-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create User Role Type
CREATE TYPE user_role AS ENUM ('admin', 'league_manager', 'player');

CREATE TABLE users (
                       id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                       first_name VARCHAR(255),
                       middle_name VARCHAR(255),
                       last_name VARCHAR(255),
                       date_of_birth DATE,
                       gender VARCHAR(50),
                       phone VARCHAR(20) NOT NULL UNIQUE,
                       email VARCHAR(255) NOT NULL UNIQUE,
                       password VARCHAR(255) NOT NULL,
                       role user_role DEFAULT 'player',
                       created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                       updated_at TIMESTAMP
);

-- Indexes
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_phone ON users(phone);