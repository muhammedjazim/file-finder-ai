-- Add migration script here
CREATE TABLE file_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    event_kind TEXT NOT NULL,
    path TEXT NOT NULL,
    rename_from TEXT,
    rename_to TEXT
);
