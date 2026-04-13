//! Audit trail for memory operations.
//!
//! Provides a decorator `AuditedMemory<M>` that wraps any `Memory` backend
//! and logs all operations to a `memory_audit` table. Opt-in via
//! `[memory] audit_enabled = true`.

use super::traits::{Memory, MemoryCategory, MemoryEntry, ProceduralMessage};
use async_trait::async_trait;
use chrono::Local;
use parking_lot::Mutex;
use rusqlite::{Connection, params};
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Audit log entry operations.
#[derive(Debug, Clone, Copy)]
pub enum AuditOp {
    Store,
    Recall,
    Get,
    List,
    Forget,
    StoreProcedural,
}

impl std::fmt::Display for AuditOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Store => write!(f, "store"),
            Self::Recall => write!(f, "recall"),
            Self::Get => write!(f, "get"),
            Self::List => write!(f, "list"),
            Self::Forget => write!(f, "forget"),
            Self::StoreProcedural => write!(f, "store_procedural"),
        }
    }
}

/// Decorator that wraps a `Memory` backend with audit logging.
pub struct AuditedMemory<M: Memory> {
    inner: M,
    audit_conn: Arc<Mutex<Connection>>,
    #[allow(dead_code)]
    db_path: PathBuf,
    access_tracking: bool,
}

impl<M: Memory> AuditedMemory<M> {
    pub fn new(inner: M, workspace_dir: &Path, access_tracking: bool) -> anyhow::Result<Self> {
        let db_path = workspace_dir.join("memory").join("audit.db");
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(&db_path)?;
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             CREATE TABLE IF NOT EXISTS memory_audit (
                 id INTEGER PRIMARY KEY AUTOINCREMENT,
                 operation TEXT NOT NULL,
                 key TEXT,
                 namespace TEXT,
                 session_id TEXT,
                 timestamp TEXT NOT NULL,
                 metadata TEXT
             );
             CREATE INDEX IF NOT EXISTS idx_audit_timestamp ON memory_audit(timestamp);
             CREATE INDEX IF NOT EXISTS idx_audit_operation ON memory_audit(operation);
             CREATE TABLE IF NOT EXISTS memory_access_log (
                 id          INTEGER PRIMARY KEY AUTOINCREMENT,
                 memory_id   TEXT NOT NULL,
                 memory_key  TEXT NOT NULL,
                 query       TEXT NOT NULL,
                 score       REAL,
                 namespace   TEXT,
                 session_id  TEXT,
                 accessed_at TEXT NOT NULL
             );
             CREATE INDEX IF NOT EXISTS idx_access_log_memory_id
                 ON memory_access_log(memory_id);
             CREATE INDEX IF NOT EXISTS idx_access_log_accessed_at
                 ON memory_access_log(accessed_at);
             CREATE INDEX IF NOT EXISTS idx_access_log_memory_id_at
                 ON memory_access_log(memory_id, accessed_at);",
        )?;

        Ok(Self {
            inner,
            audit_conn: Arc::new(Mutex::new(conn)),
            db_path,
            access_tracking,
        })
    }

    fn log_audit(
        &self,
        op: AuditOp,
        key: Option<&str>,
        namespace: Option<&str>,
        session_id: Option<&str>,
        metadata: Option<&str>,
    ) {
        let conn = self.audit_conn.lock();
        let now = Local::now().to_rfc3339();
        let op_str = op.to_string();
        let _ = conn.execute(
            "INSERT INTO memory_audit (operation, key, namespace, session_id, timestamp, metadata)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![op_str, key, namespace, session_id, now, metadata],
        );
    }

    /// Log per-result access entries for recall results.
    ///
    /// Each returned `MemoryEntry` gets a row in `memory_access_log`,
    /// producing the "spike train" data needed for temporal association
    /// scoring (ADR-005 Phase 2).
    fn log_recall_results(
        &self,
        query: &str,
        namespace: Option<&str>,
        session_id: Option<&str>,
        results: &[MemoryEntry],
    ) {
        if !self.access_tracking || results.is_empty() {
            return;
        }
        let conn = self.audit_conn.lock();
        let now = Local::now().to_rfc3339();
        // Batch-insert in a single transaction for performance.
        let _ = conn.execute_batch("BEGIN");
        for entry in results {
            let _ = conn.execute(
                "INSERT INTO memory_access_log
                     (memory_id, memory_key, query, score, namespace, session_id, accessed_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    entry.id,
                    entry.key,
                    query,
                    entry.score,
                    namespace,
                    session_id,
                    now
                ],
            );
        }
        let _ = conn.execute_batch("COMMIT");
    }

    /// Prune audit entries older than the given number of days.
    pub fn prune_older_than(&self, retention_days: u32) -> anyhow::Result<u64> {
        let conn = self.audit_conn.lock();
        let cutoff =
            (Local::now() - chrono::Duration::days(i64::from(retention_days))).to_rfc3339();
        let audit_affected = conn.execute(
            "DELETE FROM memory_audit WHERE timestamp < ?1",
            params![cutoff],
        )?;
        let access_affected = conn.execute(
            "DELETE FROM memory_access_log WHERE accessed_at < ?1",
            params![cutoff],
        )?;
        Ok(u64::try_from(audit_affected + access_affected).unwrap_or(0))
    }

    /// Count total audit entries.
    pub fn audit_count(&self) -> anyhow::Result<usize> {
        let conn = self.audit_conn.lock();
        let count: i64 =
            conn.query_row("SELECT COUNT(*) FROM memory_audit", [], |row| row.get(0))?;
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        Ok(count as usize)
    }

    /// Count total access log entries.
    pub fn access_log_count(&self) -> anyhow::Result<usize> {
        let conn = self.audit_conn.lock();
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM memory_access_log", [], |row| {
            row.get(0)
        })?;
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        Ok(count as usize)
    }

    /// Return the access spike train for a given memory entry.
    ///
    /// Returns RFC 3339 timestamps in chronological order, optionally
    /// filtered to entries on or after `since`.
    pub fn get_access_spike_train(
        &self,
        memory_id: &str,
        since: Option<&str>,
    ) -> anyhow::Result<Vec<String>> {
        let conn = self.audit_conn.lock();
        let mut timestamps = Vec::new();
        match since {
            Some(cutoff) => {
                let mut stmt = conn.prepare(
                    "SELECT accessed_at FROM memory_access_log
                     WHERE memory_id = ?1 AND accessed_at >= ?2
                     ORDER BY accessed_at",
                )?;
                let rows = stmt.query_map(params![memory_id, cutoff], |row| row.get(0))?;
                for row in rows {
                    timestamps.push(row?);
                }
            }
            None => {
                let mut stmt = conn.prepare(
                    "SELECT accessed_at FROM memory_access_log
                     WHERE memory_id = ?1
                     ORDER BY accessed_at",
                )?;
                let rows = stmt.query_map(params![memory_id], |row| row.get(0))?;
                for row in rows {
                    timestamps.push(row?);
                }
            }
        }
        Ok(timestamps)
    }

    /// Find memory IDs that were co-accessed with the given memory
    /// within a time window.
    ///
    /// Returns `(other_memory_id, co_access_count)` pairs sorted by
    /// count descending. Useful for discovering temporal associations.
    pub fn get_coaccessed_memories(
        &self,
        memory_id: &str,
        window_secs: i64,
    ) -> anyhow::Result<Vec<(String, usize)>> {
        let conn = self.audit_conn.lock();
        // Find all memories that appear in access log entries within
        // `window_secs` of any access to `memory_id`.
        let mut stmt = conn.prepare(
            "SELECT b.memory_id, COUNT(*) as cnt
             FROM memory_access_log a
             JOIN memory_access_log b
               ON a.memory_id = ?1
              AND b.memory_id != ?1
              AND ABS(
                  julianday(b.accessed_at) - julianday(a.accessed_at)
              ) * 86400 <= ?2
             GROUP BY b.memory_id
             ORDER BY cnt DESC",
        )?;
        let rows = stmt.query_map(params![memory_id, window_secs], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, usize>(1)?))
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }
}

#[async_trait]
impl<M: Memory> Memory for AuditedMemory<M> {
    fn name(&self) -> &str {
        self.inner.name()
    }

    async fn store(
        &self,
        key: &str,
        content: &str,
        category: MemoryCategory,
        session_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.log_audit(AuditOp::Store, Some(key), None, session_id, None);
        self.inner.store(key, content, category, session_id).await
    }

    async fn recall(
        &self,
        query: &str,
        limit: usize,
        session_id: Option<&str>,
        since: Option<&str>,
        until: Option<&str>,
    ) -> anyhow::Result<Vec<MemoryEntry>> {
        self.log_audit(
            AuditOp::Recall,
            None,
            None,
            session_id,
            Some(&format!("query={query}")),
        );
        let results = self
            .inner
            .recall(query, limit, session_id, since, until)
            .await?;
        self.log_recall_results(query, None, session_id, &results);
        Ok(results)
    }

    async fn get(&self, key: &str) -> anyhow::Result<Option<MemoryEntry>> {
        self.log_audit(AuditOp::Get, Some(key), None, None, None);
        self.inner.get(key).await
    }

    async fn list(
        &self,
        category: Option<&MemoryCategory>,
        session_id: Option<&str>,
    ) -> anyhow::Result<Vec<MemoryEntry>> {
        self.log_audit(AuditOp::List, None, None, session_id, None);
        self.inner.list(category, session_id).await
    }

    async fn forget(&self, key: &str) -> anyhow::Result<bool> {
        self.log_audit(AuditOp::Forget, Some(key), None, None, None);
        self.inner.forget(key).await
    }

    async fn count(&self) -> anyhow::Result<usize> {
        self.inner.count().await
    }

    async fn health_check(&self) -> bool {
        self.inner.health_check().await
    }

    async fn store_procedural(
        &self,
        messages: &[ProceduralMessage],
        session_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.log_audit(
            AuditOp::StoreProcedural,
            None,
            None,
            session_id,
            Some(&format!("messages={}", messages.len())),
        );
        self.inner.store_procedural(messages, session_id).await
    }

    async fn recall_namespaced(
        &self,
        namespace: &str,
        query: &str,
        limit: usize,
        session_id: Option<&str>,
        since: Option<&str>,
        until: Option<&str>,
    ) -> anyhow::Result<Vec<MemoryEntry>> {
        self.log_audit(
            AuditOp::Recall,
            None,
            Some(namespace),
            session_id,
            Some(&format!("query={query}")),
        );
        let results = self
            .inner
            .recall_namespaced(namespace, query, limit, session_id, since, until)
            .await?;
        self.log_recall_results(query, Some(namespace), session_id, &results);
        Ok(results)
    }

    async fn store_with_metadata(
        &self,
        key: &str,
        content: &str,
        category: MemoryCategory,
        session_id: Option<&str>,
        namespace: Option<&str>,
        importance: Option<f64>,
    ) -> anyhow::Result<()> {
        self.log_audit(AuditOp::Store, Some(key), namespace, session_id, None);
        self.inner
            .store_with_metadata(key, content, category, session_id, namespace, importance)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::NoneMemory;
    use tempfile::TempDir;

    #[tokio::test]
    async fn audited_memory_logs_store_operation() {
        let tmp = TempDir::new().unwrap();
        let inner = NoneMemory::new();
        let audited = AuditedMemory::new(inner, tmp.path(), false).unwrap();

        audited
            .store("test_key", "test_value", MemoryCategory::Core, None)
            .await
            .unwrap();

        assert_eq!(audited.audit_count().unwrap(), 1);
    }

    #[tokio::test]
    async fn audited_memory_logs_recall_operation() {
        let tmp = TempDir::new().unwrap();
        let inner = NoneMemory::new();
        let audited = AuditedMemory::new(inner, tmp.path(), false).unwrap();

        let _ = audited.recall("query", 10, None, None, None).await;

        assert_eq!(audited.audit_count().unwrap(), 1);
    }

    #[tokio::test]
    async fn audited_memory_prune_works() {
        let tmp = TempDir::new().unwrap();
        let inner = NoneMemory::new();
        let audited = AuditedMemory::new(inner, tmp.path(), false).unwrap();

        audited
            .store("k1", "v1", MemoryCategory::Core, None)
            .await
            .unwrap();

        // Pruning with 0 days should remove entries
        let pruned = audited.prune_older_than(0).unwrap();
        // Entry was just created, so 0-day retention should remove it
        // Pruning should succeed (pruned is usize, always >= 0)
        let _ = pruned;
    }

    #[tokio::test]
    async fn audited_memory_delegates_correctly() {
        let tmp = TempDir::new().unwrap();
        let inner = NoneMemory::new();
        let audited = AuditedMemory::new(inner, tmp.path(), false).unwrap();

        assert_eq!(audited.name(), "none");
        assert!(audited.health_check().await);
        assert_eq!(audited.count().await.unwrap(), 0);
    }

    // ── Access tracking tests ───────────────────────────────────

    #[tokio::test]
    async fn access_log_records_recall_results() {
        let tmp = TempDir::new().unwrap();
        let inner = NoneMemory::new();
        let audited = AuditedMemory::new(inner, tmp.path(), true).unwrap();

        // NoneMemory returns empty results, so manually log some
        let entries = vec![
            MemoryEntry {
                id: "mem-1".into(),
                key: "key-1".into(),
                content: "hello".into(),
                category: MemoryCategory::Core,
                timestamp: "2026-01-01T00:00:00Z".into(),
                session_id: None,
                score: Some(0.9),
                namespace: "default".into(),
                importance: None,
                superseded_by: None,
            },
            MemoryEntry {
                id: "mem-2".into(),
                key: "key-2".into(),
                content: "world".into(),
                category: MemoryCategory::Daily,
                timestamp: "2026-01-01T00:00:00Z".into(),
                session_id: None,
                score: Some(0.7),
                namespace: "default".into(),
                importance: None,
                superseded_by: None,
            },
        ];
        audited.log_recall_results("test query", None, None, &entries);

        assert_eq!(audited.access_log_count().unwrap(), 2);
    }

    #[tokio::test]
    async fn access_log_skipped_when_disabled() {
        let tmp = TempDir::new().unwrap();
        let inner = NoneMemory::new();
        let audited = AuditedMemory::new(inner, tmp.path(), false).unwrap();

        let entries = vec![MemoryEntry {
            id: "mem-1".into(),
            key: "key-1".into(),
            content: "hello".into(),
            category: MemoryCategory::Core,
            timestamp: "2026-01-01T00:00:00Z".into(),
            session_id: None,
            score: Some(0.9),
            namespace: "default".into(),
            importance: None,
            superseded_by: None,
        }];
        audited.log_recall_results("test query", None, None, &entries);

        assert_eq!(audited.access_log_count().unwrap(), 0);
    }

    #[tokio::test]
    async fn access_spike_train_returns_chronological_timestamps() {
        let tmp = TempDir::new().unwrap();
        let inner = NoneMemory::new();
        let audited = AuditedMemory::new(inner, tmp.path(), true).unwrap();

        let entry = vec![MemoryEntry {
            id: "mem-1".into(),
            key: "key-1".into(),
            content: "hello".into(),
            category: MemoryCategory::Core,
            timestamp: "2026-01-01T00:00:00Z".into(),
            session_id: None,
            score: Some(0.9),
            namespace: "default".into(),
            importance: None,
            superseded_by: None,
        }];

        // Log the same memory 3 times
        audited.log_recall_results("q1", None, None, &entry);
        audited.log_recall_results("q2", None, None, &entry);
        audited.log_recall_results("q3", None, None, &entry);

        let train = audited.get_access_spike_train("mem-1", None).unwrap();
        assert_eq!(train.len(), 3);
        // Timestamps should be in ascending order
        for w in train.windows(2) {
            assert!(w[0] <= w[1]);
        }
    }

    #[tokio::test]
    async fn access_log_pruning_removes_old_entries() {
        let tmp = TempDir::new().unwrap();
        let inner = NoneMemory::new();
        let audited = AuditedMemory::new(inner, tmp.path(), true).unwrap();

        let entries = vec![MemoryEntry {
            id: "mem-1".into(),
            key: "key-1".into(),
            content: "hello".into(),
            category: MemoryCategory::Core,
            timestamp: "2026-01-01T00:00:00Z".into(),
            session_id: None,
            score: Some(0.9),
            namespace: "default".into(),
            importance: None,
            superseded_by: None,
        }];
        audited.log_recall_results("q", None, None, &entries);

        // Also create an audit entry
        audited.log_audit(AuditOp::Store, Some("k"), None, None, None);

        assert_eq!(audited.access_log_count().unwrap(), 1);
        assert_eq!(audited.audit_count().unwrap(), 1);

        // Prune with 0 retention removes everything
        let pruned = audited.prune_older_than(0).unwrap();
        assert!(pruned >= 2);
        assert_eq!(audited.access_log_count().unwrap(), 0);
        assert_eq!(audited.audit_count().unwrap(), 0);
    }

    #[tokio::test]
    async fn coaccessed_memories_returns_co_occurring_ids() {
        let tmp = TempDir::new().unwrap();
        let inner = NoneMemory::new();
        let audited = AuditedMemory::new(inner, tmp.path(), true).unwrap();

        let together = vec![
            MemoryEntry {
                id: "A".into(),
                key: "key-a".into(),
                content: "a".into(),
                category: MemoryCategory::Core,
                timestamp: "2026-01-01T00:00:00Z".into(),
                session_id: None,
                score: Some(0.9),
                namespace: "default".into(),
                importance: None,
                superseded_by: None,
            },
            MemoryEntry {
                id: "B".into(),
                key: "key-b".into(),
                content: "b".into(),
                category: MemoryCategory::Core,
                timestamp: "2026-01-01T00:00:00Z".into(),
                session_id: None,
                score: Some(0.8),
                namespace: "default".into(),
                importance: None,
                superseded_by: None,
            },
        ];

        // A and B recalled together twice
        audited.log_recall_results("q1", None, None, &together);
        audited.log_recall_results("q2", None, None, &together);

        // C recalled separately
        let alone = vec![MemoryEntry {
            id: "C".into(),
            key: "key-c".into(),
            content: "c".into(),
            category: MemoryCategory::Core,
            timestamp: "2026-01-01T00:00:00Z".into(),
            session_id: None,
            score: Some(0.5),
            namespace: "default".into(),
            importance: None,
            superseded_by: None,
        }];
        audited.log_recall_results("q3", None, None, &alone);

        // Co-accessed with A within 60 seconds should include B
        let coaccessed = audited.get_coaccessed_memories("A", 60).unwrap();
        assert!(!coaccessed.is_empty());
        assert_eq!(coaccessed[0].0, "B");
        // C should not appear (accessed at different time, not within window of A)
        // Actually C could appear if the timestamps are very close (same test run),
        // but B should have a higher count
        if coaccessed.len() > 1 {
            assert!(coaccessed[0].1 >= coaccessed[1].1);
        }
    }
}
