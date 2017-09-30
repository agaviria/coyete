-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
	id         SERIAL PRIMARY KEY,
	email      VARCHAR NOT NULL UNIQUE,
	uuid_      UUID NOT NULL UNIQUE,
	level_     INTEGER NOT NULL CHECK (level_ >= 0),
	created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
	updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);

CREATE UNIQUE INDEX on users (uuid_);
CREATE UNIQUE INDEX on users (email);

CREATE TABLE auth (
	id            SERIAL REFERENCES users ON DELETE CASCADE PRIMARY KEY,
	salt          BYTEA NOT NULL,
	password_hash BYTEA NOT NULL,
	created_at    TIMESTAMP NOT NULL DEFAULT current_timestamp,
	updated_at    TIMESTAMP NOT NULL DEFAULT current_timestamp
);

CREATE TABLE sessions (
	id             SERIAL PRIMARY KEY,
	proposed_token BYTEA NOT NULL UNIQUE,
	current_token  BYTEA NOT NULL UNIQUE,
	retired_token  BYTEA NOT NULL UNIQUE,
	access_version INTEGER NOT NULL DEFAULT 0,
	user_id        SERIAL REFERENCES users ON DELETE CASCADE,
	started        TIMESTAMP NOT NULL DEFAULT current_timestamp,
	last_seen      TIMESTAMP NOT NULL DEFAULT current_timestamp,
	last_ip        BYTEA NOT NULL
);
