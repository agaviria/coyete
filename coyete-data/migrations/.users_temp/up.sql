-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
	id					SERIAL PRIMARY KEY,
	email				VARCHAR UNIQUE NOT NULL,
	uuid_				UUID UNIQUE NOT NULL,
	level_			INTEGER NOT NULL CHECK (level_ >= 0),
	created_at	TIMESTAMPTZ DEFAULT NOW() NOT NULL,
	updated_at	TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

CREATE UNIQUE INDEX on users (uuid_);
CREATE UNIQUE INDEX on users (email);

CREATE TABLE pending_email_confirms (
	secret VARCHAR PRIMARY KEY,
	email VARCHAR NOT NULL,
	groups INTEGER[] NOT NULL,
	added TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

CREATE TABLE auth (
	id						SERIAL REFERENCES users ON DELETE CASCADE PRIMARY KEY,
	salt          BYTEA NOT NULL,
	password_hash	BYTEA NOT NULL,
	created_at		TIMESTAMPTZ DEFAULT NOW() NOT NULL,
	updated_at		TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

CREATE TABLE sessions (
	id							SERIAL PRIMARY KEY,
	proposed_token	BYTEA NOT NULL UNIQUE,
	current_token		BYTEA NOT NULL UNIQUE,
	retired_token		BYTEA NOT NULL UNIQUE,
	access_version	INTEGER NOT NULL DEFAULT 0,
	user_id					SERIAL REFERENCES users ON DELETE CASCADE,
	started					TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	last_seen				TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	last_ip					BYTEA NOT NULL
);