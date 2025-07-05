CREATE TABLE IF NOT EXISTS bugs (
  bug_id       INTEGER PRIMARY KEY AUTOINCREMENT,
  title        TEXT    NOT NULL,
  description  TEXT    NOT NULL,
  reported_by  TEXT    NOT NULL,
  severity     TEXT    NOT NULL,
  status       TEXT    NOT NULL DEFAULT 'open',
  project      TEXT,
  developer_id INTEGER
);

CREATE TABLE IF NOT EXISTS users (
  user_id  INTEGER PRIMARY KEY,
  username TEXT    UNIQUE NOT NULL,
  hash     TEXT    NOT NULL
);
