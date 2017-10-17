-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE users (
	id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	email       VARCHAR(120) NOT NULL UNIQUE,
	password    BYTEA NOT NULL,
	is_verified BOOLEAN NOT NULL DEFAULT 'f',
	created_at  TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at  TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX on users (email);

CREATE TABLE certification_codes (
	id             SERIAL PRIMARY KEY,
	code           VARCHAR(100) NOT NULL UNIQUE,
	user_id        UUID REFERENCES users ON DELETE CASCADE
);

CREATE TABLE sessions (
	id             SERIAL PRIMARY KEY,
	proposed_token BYTEA NOT NULL UNIQUE,
	current_token  BYTEA NOT NULL UNIQUE,
	retired_token  BYTEA NOT NULL UNIQUE,
	access_version INTEGER NOT NULL DEFAULT 0,
	user_id        UUID REFERENCES users ON DELETE CASCADE,
	started        TIMESTAMP NOT NULL DEFAULT NOW(),
	last_seen      TIMESTAMP NOT NULL DEFAULT NOW(),
	last_ip        BYTEA NOT NULL
);
