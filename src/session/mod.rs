//! Persistent session storage for the TUI.
//!
//! A session captures one conversation (messages + metadata) in SQLite so it
//! can be resumed later. See `docs/superpowers/specs/2026-04-18-tui-sessions-and-polish-design.md`.

pub mod id;
pub mod message;
pub mod store;
pub mod types;

pub use id::SessionId;
pub use message::{ChatMessage, ToolStatus};
pub use store::SessionStore;
pub use types::{MessageCounts, Session, SessionMeta, StoredMessage};
