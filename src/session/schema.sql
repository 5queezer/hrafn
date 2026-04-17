CREATE TABLE IF NOT EXISTS sessions (
    id               TEXT PRIMARY KEY,
    title            TEXT,
    title_explicit   INTEGER NOT NULL DEFAULT 0,
    cwd              TEXT NOT NULL,
    created_at       INTEGER NOT NULL,
    updated_at       INTEGER NOT NULL,
    duration_ms      INTEGER NOT NULL DEFAULT 0,
    provider         TEXT,
    model            TEXT,
    msg_total        INTEGER NOT NULL DEFAULT 0,
    msg_user         INTEGER NOT NULL DEFAULT 0,
    msg_assistant    INTEGER NOT NULL DEFAULT 0,
    msg_tool_call    INTEGER NOT NULL DEFAULT 0,
    msg_tool_result  INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_sessions_updated ON sessions(updated_at DESC);

CREATE TABLE IF NOT EXISTS messages (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id  TEXT NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    seq         INTEGER NOT NULL,
    kind        TEXT NOT NULL,
    payload     TEXT NOT NULL,
    ts          INTEGER NOT NULL,
    UNIQUE(session_id, seq)
);
CREATE INDEX IF NOT EXISTS idx_messages_session ON messages(session_id, seq);

CREATE TABLE IF NOT EXISTS schema_version (version INTEGER PRIMARY KEY);
INSERT OR IGNORE INTO schema_version VALUES (1);
