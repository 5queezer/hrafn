//! SQLite-backed persistence for sessions.
//!
//! See `docs/superpowers/specs/2026-04-18-tui-sessions-and-polish-design.md`.

use std::path::Path;
use std::sync::Mutex;

use anyhow::{Context, Result, bail};
use rusqlite::{Connection, OptionalExtension, params};

use super::{Session, SessionId, SessionMeta};

const SCHEMA: &str = include_str!("schema.sql");
const SCHEMA_VERSION: i64 = 1;

/// SQLite-backed session store.
///
/// The inner `Connection` is wrapped in a `Mutex` so `Arc<SessionStore>` is
/// `Send + Sync` — needed so `SessionHandle` (held by the TUI running on a
/// `tokio::spawn_blocking` thread) can be sent across the thread boundary.
/// In practice there's only one thread using the store at a time (the TUI
/// worker), so contention on the mutex is negligible.
pub struct SessionStore {
    conn: Mutex<Connection>,
}

impl SessionStore {
    pub fn open(path: &Path) -> Result<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("create {}", parent.display()))?;
        }
        let conn = Connection::open(path).context("open sessions.db")?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "synchronous", "NORMAL")?;
        conn.pragma_update(None, "foreign_keys", "ON")?;
        conn.busy_timeout(std::time::Duration::from_millis(2000))?;
        conn.execute_batch(SCHEMA).context("apply schema")?;

        let v: i64 = conn.query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_version",
            [],
            |r| r.get(0),
        )?;
        if v != SCHEMA_VERSION {
            bail!("unknown schema version {v}; upgrade hrafn");
        }
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    fn conn(&self) -> std::sync::MutexGuard<'_, Connection> {
        self.conn
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    pub fn create(
        &self,
        cwd: &Path,
        title_seed: Option<&str>,
        provider: Option<&str>,
        model: Option<&str>,
    ) -> Result<SessionMeta> {
        let id = SessionId::generate();
        let now = chrono::Utc::now();
        let now_ms = now.timestamp_millis();
        let cwd_str = cwd.to_string_lossy().to_string();
        self.conn()
            .execute(
                "INSERT INTO sessions (id, title, title_explicit, cwd, created_at, updated_at, provider, model)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?5, ?6, ?7)",
                params![
                    id.as_str(),
                    title_seed,
                    i32::from(title_seed.is_some()),
                    cwd_str,
                    now_ms,
                    provider,
                    model,
                ],
            )
            .context("insert session")?;
        Ok(SessionMeta {
            id,
            title: title_seed.map(str::to_string),
            title_explicit: title_seed.is_some(),
            cwd: cwd.to_path_buf(),
            created_at: now,
            updated_at: now,
            duration: std::time::Duration::ZERO,
            provider: provider.map(str::to_string),
            model: model.map(str::to_string),
            counts: super::MessageCounts::default(),
        })
    }

    pub fn load(&self, id: &SessionId) -> Result<Session> {
        let meta = self.load_meta(id)?;
        let conn = self.conn();
        let mut stmt = conn.prepare(
            "SELECT seq, kind, payload, ts FROM messages WHERE session_id = ?1 ORDER BY seq ASC",
        )?;
        let rows = stmt.query_map(params![id.as_str()], |r| {
            let seq: u32 = u32::try_from(r.get::<_, i64>(0)?).unwrap_or(u32::MAX);
            let kind: String = r.get(1)?;
            let payload: String = r.get(2)?;
            let ts_ms: i64 = r.get(3)?;
            Ok((seq, kind, payload, ts_ms))
        })?;
        let mut messages = Vec::new();
        for row in rows {
            let (seq, kind, payload, ts_ms) = row?;
            let ts =
                chrono::DateTime::from_timestamp_millis(ts_ms).unwrap_or_else(chrono::Utc::now);
            let body = match serde_json::from_str::<crate::session::ChatMessage>(&payload) {
                Ok(m) => m,
                Err(e) => {
                    tracing::warn!(id=%id, seq, kind, error=%e, "corrupt payload; substituting placeholder");
                    crate::session::ChatMessage::System {
                        text: format!("[corrupt message @ seq {seq}]"),
                    }
                }
            };
            messages.push(super::StoredMessage { seq, ts, body });
        }
        Ok(Session { meta, messages })
    }

    pub fn append(&self, id: &SessionId, msg: &crate::session::ChatMessage) -> Result<()> {
        let payload = serde_json::to_string(msg).context("serialize ChatMessage")?;
        let kind = match msg {
            crate::session::ChatMessage::User { .. } => "user",
            crate::session::ChatMessage::Assistant { .. } => "assistant",
            crate::session::ChatMessage::ToolCall { .. } => "tool_call",
            crate::session::ChatMessage::ToolResult { .. } => "tool_result",
            crate::session::ChatMessage::System { .. } => "system",
        };
        let now_ms = chrono::Utc::now().timestamp_millis();
        let counter_col = match kind {
            "user" => "msg_user",
            "assistant" => "msg_assistant",
            "tool_call" => "msg_tool_call",
            "tool_result" => "msg_tool_result",
            _ => "msg_total",
        };
        let conn = self.conn();
        let tx = conn.unchecked_transaction()?;
        tx.execute(
            "INSERT INTO messages (session_id, seq, kind, payload, ts)
             VALUES (?1, COALESCE((SELECT MAX(seq) FROM messages WHERE session_id = ?1), 0) + 1, ?2, ?3, ?4)",
            params![id.as_str(), kind, payload, now_ms],
        )?;
        // SQLite evaluates all RHS expressions against the pre-UPDATE row, so when
        // counter_col == "msg_total" the duplicate assignments still bump the column
        // by exactly 1 (last assignment wins — not a double-bump).
        let sql = format!(
            "UPDATE sessions SET {counter_col} = {counter_col} + 1, msg_total = msg_total + 1, updated_at = ?1 WHERE id = ?2"
        );
        tx.execute(&sql, params![now_ms, id.as_str()])?;
        tx.commit()?;
        Ok(())
    }

    pub fn list(&self, limit: usize) -> Result<Vec<SessionMeta>> {
        let limit_i64 = i64::try_from(limit).unwrap_or(i64::MAX);
        // Collect IDs under the lock, then release before calling `load_meta`
        // (which re-acquires the lock for each row).
        let ids: Vec<String> = {
            let conn = self.conn();
            let mut stmt =
                conn.prepare("SELECT id FROM sessions ORDER BY updated_at DESC LIMIT ?1")?;
            let rows = stmt.query_map(params![limit_i64], |r| r.get::<_, String>(0))?;
            rows.collect::<Result<Vec<_>, _>>()?
        };
        let mut out = Vec::new();
        for id in ids {
            let id = SessionId::parse(&id).context("parse id from DB")?;
            out.push(self.load_meta(&id)?);
        }
        Ok(out)
    }

    pub fn find_by_title_fuzzy(&self, needle: &str) -> Result<Option<SessionMeta>> {
        let like = format!("%{needle}%");
        let id: Option<String> = self
            .conn()
            .query_row(
                "SELECT id FROM sessions WHERE title LIKE ?1 COLLATE NOCASE ORDER BY updated_at DESC LIMIT 1",
                params![like],
                |r| r.get(0),
            )
            .optional()?;
        id.map(|s| {
            let id = SessionId::parse(&s)?;
            self.load_meta(&id)
        })
        .transpose()
    }

    pub fn most_recent(&self) -> Result<Option<SessionMeta>> {
        let id: Option<String> = self
            .conn()
            .query_row(
                "SELECT id FROM sessions ORDER BY updated_at DESC LIMIT 1",
                [],
                |r| r.get(0),
            )
            .optional()?;
        id.map(|s| {
            let id = SessionId::parse(&s)?;
            self.load_meta(&id)
        })
        .transpose()
    }

    pub fn set_title(&self, id: &SessionId, title: &str, explicit: bool) -> Result<()> {
        let now_ms = chrono::Utc::now().timestamp_millis();
        // NOTE: explicit=false is a no-op if title is already explicit (used by the
        // first-message auto-title path; should never overwrite a user-set title).
        let sql = if explicit {
            "UPDATE sessions SET title = ?1, title_explicit = 1, updated_at = ?2 WHERE id = ?3"
        } else {
            "UPDATE sessions SET title = ?1, updated_at = ?2 WHERE id = ?3 AND title_explicit = 0"
        };
        self.conn()
            .execute(sql, params![title, now_ms, id.as_str()])?;
        Ok(())
    }

    pub fn add_duration(&self, id: &SessionId, d: std::time::Duration) -> Result<()> {
        let add_ms = i64::try_from(d.as_millis()).unwrap_or(i64::MAX);
        self.conn().execute(
            "UPDATE sessions SET duration_ms = duration_ms + ?1 WHERE id = ?2",
            params![add_ms, id.as_str()],
        )?;
        Ok(())
    }

    pub fn delete(&self, id: &SessionId) -> Result<()> {
        self.conn()
            .execute("DELETE FROM sessions WHERE id = ?1", params![id.as_str()])?;
        Ok(())
    }

    fn load_meta(&self, id: &SessionId) -> Result<SessionMeta> {
        self.conn()
            .query_row(
                "SELECT id, title, title_explicit, cwd, created_at, updated_at, duration_ms, provider, model,
                        msg_total, msg_user, msg_assistant, msg_tool_call, msg_tool_result
                 FROM sessions WHERE id = ?1",
                params![id.as_str()],
                |r| {
                    let id_str: String = r.get(0)?;
                    let created_ms: i64 = r.get(4)?;
                    let updated_ms: i64 = r.get(5)?;
                    let duration_ms: i64 = r.get(6)?;
                    Ok(SessionMeta {
                        id: SessionId::parse(&id_str).map_err(|e| {
                            rusqlite::Error::FromSqlConversionFailure(
                                0,
                                rusqlite::types::Type::Text,
                                Box::new(std::io::Error::other(e.to_string())),
                            )
                        })?,
                        title: r.get(1)?,
                        title_explicit: r.get::<_, i64>(2)? != 0,
                        cwd: std::path::PathBuf::from(r.get::<_, String>(3)?),
                        created_at: chrono::DateTime::from_timestamp_millis(created_ms)
                            .unwrap_or_else(chrono::Utc::now),
                        updated_at: chrono::DateTime::from_timestamp_millis(updated_ms)
                            .unwrap_or_else(chrono::Utc::now),
                        duration: std::time::Duration::from_millis(
                            u64::try_from(duration_ms).unwrap_or(0),
                        ),
                        provider: r.get(7)?,
                        model: r.get(8)?,
                        counts: super::MessageCounts {
                            total: u32::try_from(r.get::<_, i64>(9)?).unwrap_or(u32::MAX),
                            user: u32::try_from(r.get::<_, i64>(10)?).unwrap_or(u32::MAX),
                            assistant: u32::try_from(r.get::<_, i64>(11)?).unwrap_or(u32::MAX),
                            tool_call: u32::try_from(r.get::<_, i64>(12)?).unwrap_or(u32::MAX),
                            tool_result: u32::try_from(r.get::<_, i64>(13)?).unwrap_or(u32::MAX),
                        },
                    })
                },
            )
            .context("load meta")
    }
}

/// PID-based single-writer lock for a session.
///
/// Written as `<db_dir>/<session_id>.lock` containing the owning process's
/// PID. Acquisition fails if the lockfile exists and the recorded PID is
/// still alive; a stale lock (dead PID) is reclaimed. The lockfile is
/// removed on drop (best-effort).
#[derive(Debug)]
pub struct SessionLock {
    path: std::path::PathBuf,
}

impl SessionLock {
    pub fn acquire(db_path: &Path, id: &SessionId) -> Result<Self> {
        let lock_path = Self::lockfile_path(db_path, id);
        if lock_path.exists() {
            let content = std::fs::read_to_string(&lock_path).unwrap_or_default();
            if let Ok(pid) = content.trim().parse::<u32>() {
                if is_pid_alive(pid) {
                    bail!("session {} is locked (pid {})", id, pid);
                }
            }
            let _ = std::fs::remove_file(&lock_path);
        }
        std::fs::write(&lock_path, std::process::id().to_string())
            .with_context(|| format!("write {}", lock_path.display()))?;
        Ok(Self { path: lock_path })
    }

    fn lockfile_path(db_path: &Path, id: &SessionId) -> std::path::PathBuf {
        let dir = db_path
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| std::path::PathBuf::from("."));
        dir.join(format!("{}.lock", id.as_str()))
    }
}

impl Drop for SessionLock {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

#[cfg(unix)]
fn is_pid_alive(pid: u32) -> bool {
    // SAFETY: `kill` with sig=0 only checks process existence; it does not
    // read or mutate any memory, and the PID value is validated by the
    // kernel. No invariants are upheld by this call.
    let rc = unsafe { libc::kill(pid as libc::pid_t, 0) };
    rc == 0
}

#[cfg(not(unix))]
fn is_pid_alive(_pid: u32) -> bool {
    // Conservative: assume alive on non-unix. Safer than racing.
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn store() -> (tempfile::TempDir, SessionStore) {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("sessions.db");
        let store = SessionStore::open(&path).unwrap();
        (dir, store)
    }

    #[test]
    fn open_creates_db_and_schema() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("sessions.db");
        let _store = SessionStore::open(&path).unwrap();
        assert!(path.exists());
    }

    #[test]
    fn open_is_idempotent() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("sessions.db");
        let a = SessionStore::open(&path).unwrap();
        drop(a);
        let _b = SessionStore::open(&path).unwrap();
    }

    #[test]
    fn create_returns_meta_and_persists() {
        let (_d, store) = store();
        let meta = store
            .create(
                &PathBuf::from("/tmp/foo"),
                Some("My title"),
                Some("anthropic"),
                Some("claude-opus-4-7"),
            )
            .unwrap();
        assert_eq!(meta.title.as_deref(), Some("My title"));
        assert!(meta.title_explicit);
        assert_eq!(meta.cwd, PathBuf::from("/tmp/foo"));
        assert_eq!(meta.provider.as_deref(), Some("anthropic"));
        assert_eq!(meta.counts.total, 0);
    }

    #[test]
    fn load_roundtrips_empty_session() {
        let (_d, store) = store();
        let meta = store
            .create(&PathBuf::from("/tmp"), None, None, None)
            .unwrap();
        let s = store.load(&meta.id).unwrap();
        assert_eq!(s.meta.id, meta.id);
        assert!(s.messages.is_empty());
    }

    #[test]
    fn append_increments_counters() {
        let (_d, store) = store();
        let meta = store
            .create(&PathBuf::from("/tmp"), None, None, None)
            .unwrap();
        let id = meta.id;
        store
            .append(
                &id,
                &crate::session::ChatMessage::User { text: "hi".into() },
            )
            .unwrap();
        store
            .append(
                &id,
                &crate::session::ChatMessage::Assistant { text: "hey".into() },
            )
            .unwrap();
        store
            .append(
                &id,
                &crate::session::ChatMessage::ToolCall {
                    name: "shell".into(),
                    args: "{}".into(),
                    status: crate::session::ToolStatus::Done(std::time::Duration::from_millis(5)),
                },
            )
            .unwrap();
        let s = store.load(&id).unwrap();
        assert_eq!(s.meta.counts.total, 3);
        assert_eq!(s.meta.counts.user, 1);
        assert_eq!(s.meta.counts.assistant, 1);
        assert_eq!(s.meta.counts.tool_call, 1);
        assert_eq!(s.messages.len(), 3);
        assert_eq!(s.messages[0].seq, 1);
        assert_eq!(s.messages[2].seq, 3);
    }

    #[test]
    fn list_orders_by_updated_desc() {
        let (_d, store) = store();
        let a = store
            .create(&PathBuf::from("/tmp"), Some("older"), None, None)
            .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(5));
        let b = store
            .create(&PathBuf::from("/tmp"), Some("newer"), None, None)
            .unwrap();
        let list = store.list(10).unwrap();
        assert_eq!(list[0].id, b.id);
        assert_eq!(list[1].id, a.id);
    }

    #[test]
    fn fuzzy_picks_most_recent_match() {
        let (_d, store) = store();
        let _a = store
            .create(&PathBuf::from("/tmp"), Some("Fix ACP bug"), None, None)
            .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(5));
        let b = store
            .create(
                &PathBuf::from("/tmp"),
                Some("Running ACP tests"),
                None,
                None,
            )
            .unwrap();
        let hit = store.find_by_title_fuzzy("acp").unwrap().unwrap();
        assert_eq!(hit.id, b.id);
        assert!(store.find_by_title_fuzzy("nonexistent").unwrap().is_none());
    }

    #[test]
    fn most_recent_returns_newest_or_none() {
        let (_d, store) = store();
        assert!(store.most_recent().unwrap().is_none());
        let _a = store
            .create(&PathBuf::from("/tmp"), None, None, None)
            .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(5));
        let b = store
            .create(&PathBuf::from("/tmp"), None, None, None)
            .unwrap();
        assert_eq!(store.most_recent().unwrap().unwrap().id, b.id);
    }

    #[test]
    fn set_title_updates_explicit_flag() {
        let (_d, store) = store();
        let m = store
            .create(&PathBuf::from("/tmp"), None, None, None)
            .unwrap();
        store.set_title(&m.id, "Explicit", true).unwrap();
        let loaded = store.load(&m.id).unwrap();
        assert_eq!(loaded.meta.title.as_deref(), Some("Explicit"));
        assert!(loaded.meta.title_explicit);
    }

    #[test]
    fn set_title_non_explicit_does_not_override_explicit() {
        let (_d, store) = store();
        let m = store
            .create(&PathBuf::from("/tmp"), None, None, None)
            .unwrap();
        store.set_title(&m.id, "Explicit", true).unwrap();
        store.set_title(&m.id, "Auto", false).unwrap(); // should NOT override
        let loaded = store.load(&m.id).unwrap();
        assert_eq!(loaded.meta.title.as_deref(), Some("Explicit"));
    }

    #[test]
    fn delete_cascades_messages() {
        let (_d, store) = store();
        let m = store
            .create(&PathBuf::from("/tmp"), None, None, None)
            .unwrap();
        store
            .append(
                &m.id,
                &crate::session::ChatMessage::User { text: "hi".into() },
            )
            .unwrap();
        store.delete(&m.id).unwrap();
        assert!(store.load(&m.id).is_err());
    }

    #[test]
    fn lockfile_acquire_and_release() {
        let dir = tempfile::tempdir().unwrap();
        let db = dir.path().join("s.db");
        let id = SessionId::generate();
        let lock = SessionLock::acquire(&db, &id).unwrap();
        let err = SessionLock::acquire(&db, &id).unwrap_err();
        assert!(err.to_string().contains("locked"), "got {err}");
        drop(lock);
        let _lock2 = SessionLock::acquire(&db, &id).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn stale_lockfile_is_reclaimed() {
        let dir = tempfile::tempdir().unwrap();
        let db = dir.path().join("s.db");
        let id = SessionId::generate();
        let lock_path = db.parent().unwrap().join(format!("{}.lock", id.as_str()));
        // Seed with this test process's PID — guaranteed alive. Acquire fails.
        let live_pid = std::process::id();
        std::fs::write(&lock_path, live_pid.to_string()).unwrap();
        let err = SessionLock::acquire(&db, &id).unwrap_err();
        assert!(err.to_string().contains("locked"), "got {err}");
        // Replace with a (hopefully) dead PID. 2^30 is well outside typical PID max.
        std::fs::write(&lock_path, (1u32 << 30).to_string()).unwrap();
        let _lock = SessionLock::acquire(&db, &id).unwrap();
    }

    #[test]
    fn add_duration_accumulates() {
        let (_d, store) = store();
        let m = store
            .create(&PathBuf::from("/tmp"), None, None, None)
            .unwrap();
        store
            .add_duration(&m.id, std::time::Duration::from_secs(3))
            .unwrap();
        store
            .add_duration(&m.id, std::time::Duration::from_secs(2))
            .unwrap();
        let loaded = store.load(&m.id).unwrap();
        assert_eq!(loaded.meta.duration.as_secs(), 5);
    }
}
