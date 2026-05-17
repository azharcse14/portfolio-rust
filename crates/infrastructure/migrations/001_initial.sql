-- Initial schema: 6 tables.
-- UUIDs are stored as TEXT (ISO format). DateTimes as TEXT (ISO 8601, UTC).
-- Arrays (tags, tech_stack) are stored as JSON TEXT.

CREATE TABLE users (
    id              TEXT PRIMARY KEY NOT NULL,
    email           TEXT NOT NULL UNIQUE,
    password_hash   TEXT NOT NULL,
    name            TEXT,
    created_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Profile is a singleton — always exactly one row with id = 1.
CREATE TABLE profile (
    id          INTEGER PRIMARY KEY CHECK (id = 1),
    name        TEXT NOT NULL,
    title       TEXT NOT NULL,
    bio         TEXT NOT NULL,
    photo_url   TEXT,
    email       TEXT,
    github      TEXT,
    linkedin    TEXT,
    twitter     TEXT
);

CREATE TABLE projects (
    id              TEXT PRIMARY KEY NOT NULL,
    title           TEXT NOT NULL,
    slug            TEXT NOT NULL UNIQUE,
    description     TEXT NOT NULL,
    image_url       TEXT,
    tech_stack      TEXT NOT NULL DEFAULT '[]',
    github_url      TEXT,
    live_url        TEXT,
    featured        INTEGER NOT NULL DEFAULT 0,
    display_order   INTEGER NOT NULL DEFAULT 0,
    created_at      TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX idx_projects_featured ON projects(featured, display_order);

CREATE TABLE posts (
    id              TEXT PRIMARY KEY NOT NULL,
    title           TEXT NOT NULL,
    slug            TEXT NOT NULL UNIQUE,
    content_md      TEXT NOT NULL,
    cover_image     TEXT,
    tags            TEXT NOT NULL DEFAULT '[]',
    published       INTEGER NOT NULL DEFAULT 0,
    published_at    TEXT,
    created_at      TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX idx_posts_published ON posts(published, published_at);

CREATE TABLE skills (
    id              TEXT PRIMARY KEY NOT NULL,
    name            TEXT NOT NULL,
    icon            TEXT,
    category        TEXT,
    percentage      INTEGER NOT NULL DEFAULT 0,
    color           TEXT,
    display_order   INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX idx_skills_order ON skills(display_order);

CREATE TABLE messages (
    id          TEXT PRIMARY KEY NOT NULL,
    name        TEXT NOT NULL,
    email       TEXT NOT NULL,
    subject     TEXT,
    body        TEXT NOT NULL,
    read        INTEGER NOT NULL DEFAULT 0,
    created_at  TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX idx_messages_created ON messages(created_at);
