-- Your SQL goes here
CREATE TABLE users (
	id         SERIAL PRIMARY KEY,
	email      VARCHAR NOT NULL UNIQUE,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX on users (email);

CREATE TABLE auth (
	id            SERIAL REFERENCES users ON DELETE CASCADE PRIMARY KEY,
	salt          BYTEA NOT NULL,
	password_hash BYTEA NOT NULL,
	created_at    TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at    TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE sessions (
	id             SERIAL PRIMARY KEY,
	proposed_token BYTEA NOT NULL UNIQUE,
	current_token  BYTEA NOT NULL UNIQUE,
	retired_token  BYTEA NOT NULL UNIQUE,
	access_version INTEGER NOT NULL DEFAULT 0,
	user_id        SERIAL REFERENCES users ON DELETE CASCADE,
	started        TIMESTAMP NOT NULL DEFAULT NOW(),
	last_seen      TIMESTAMP NOT NULL DEFAULT NOW(),
	last_ip        BYTEA NOT NULL
);
