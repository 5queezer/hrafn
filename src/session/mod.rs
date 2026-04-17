//! Persistent session storage for the TUI.
//!
//! A session captures one conversation (messages + metadata) in SQLite so it
//! can be resumed later. See `docs/superpowers/specs/2026-04-18-tui-sessions-and-polish-design.md`.

pub mod id;

pub use id::SessionId;
