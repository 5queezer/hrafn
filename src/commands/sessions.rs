//! `hrafn --list-sessions` and `hrafn --delete-session` handlers.
//!
//! See `docs/superpowers/specs/2026-04-18-tui-sessions-and-polish-design.md`.

use anyhow::Result;

use crate::session::{SessionMeta, SessionStore};

#[allow(clippy::unused_async)]
pub async fn list(as_json: bool) -> Result<()> {
    let path = default_db_path()?;
    if !path.exists() {
        if as_json {
            println!("[]");
        } else {
            println!("No sessions yet.");
        }
        return Ok(());
    }
    let store = SessionStore::open(&path)?;
    let sessions = store.list(1000)?;
    if sessions.is_empty() {
        if as_json {
            println!("[]");
        } else {
            println!("No sessions yet.");
        }
        return Ok(());
    }
    if as_json {
        println!("{}", serde_json::to_string_pretty(&sessions)?);
    } else {
        print_table(&sessions);
    }
    Ok(())
}

#[allow(clippy::unused_async)]
pub async fn delete(id_str: &str, yes: bool) -> Result<()> {
    let path = default_db_path()?;
    let store = SessionStore::open(&path)?;
    let id = crate::session::SessionId::parse(id_str)?;
    let _loaded = store
        .load(&id)
        .map_err(|_| anyhow::anyhow!("session not found: {id_str}"))?;
    if !yes {
        use std::io::Write;
        eprint!("Delete session {id_str}? [y/N] ");
        std::io::stderr().flush().ok();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if !matches!(input.trim().to_lowercase().as_str(), "y" | "yes") {
            eprintln!("aborted.");
            return Ok(());
        }
    }
    store.delete(&id)?;
    eprintln!("deleted {id_str}");
    Ok(())
}

pub fn default_db_path() -> Result<std::path::PathBuf> {
    let data = dirs::data_dir()
        .ok_or_else(|| anyhow::anyhow!("could not resolve XDG_DATA_HOME or fallback"))?;
    Ok(data.join("hrafn").join("sessions.db"))
}

fn print_table(sessions: &[SessionMeta]) {
    println!(
        "  {:<24} {:<12} {:>5} {:>5} {:>5}  TITLE",
        "ID", "UPDATED", "MSGS", "USER", "TOOL"
    );
    for s in sessions {
        let rel = relative_time(s.updated_at);
        let title = s.title.as_deref().unwrap_or("—");
        println!(
            "  {:<24} {:<12} {:>5} {:>5} {:>5}  {}",
            s.id.as_str(),
            rel,
            s.counts.total,
            s.counts.user,
            s.counts.tool_call,
            title,
        );
    }
}

fn relative_time(then: chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let d = now.signed_duration_since(then);
    let secs = d.num_seconds().max(0);
    if secs < 60 {
        format!("{secs}s ago")
    } else if secs < 3600 {
        format!("{}m ago", secs / 60)
    } else if secs < 86_400 {
        format!("{}h ago", secs / 3600)
    } else if secs < 604_800 {
        format!("{}d ago", secs / 86_400)
    } else {
        format!("{}w ago", secs / 604_800)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relative_time_buckets() {
        let now = chrono::Utc::now();
        let one_sec_ago = now - chrono::Duration::seconds(1);
        let one_min_ago = now - chrono::Duration::seconds(120);
        let one_hour_ago = now - chrono::Duration::seconds(7200);
        let one_day_ago = now - chrono::Duration::seconds(2 * 86_400);
        let one_week_ago = now - chrono::Duration::seconds(2 * 604_800);
        assert!(relative_time(one_sec_ago).ends_with("s ago"));
        assert!(relative_time(one_min_ago).ends_with("m ago"));
        assert!(relative_time(one_hour_ago).ends_with("h ago"));
        assert!(relative_time(one_day_ago).ends_with("d ago"));
        assert!(relative_time(one_week_ago).ends_with("w ago"));
    }

    #[test]
    fn empty_fallback_to_data_dir() {
        // Smoke: default_db_path doesn't panic on a reasonable host.
        let _ = default_db_path();
    }
}
