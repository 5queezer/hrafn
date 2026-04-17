//! Chat message types persisted in a session.
//!
//! Lives under `session` (not `tui`) so non-TUI code paths (e.g. the CLI
//! `--list-sessions` subcommand) can depend on `Session`/`StoredMessage`
//! without pulling in the `tui` feature. Re-exported from `crate::tui` for
//! backwards compatibility.

use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

/// Status of a tool execution displayed in chat.
///
/// `Running(Instant)` is a transient UI state; at persist time the custom
/// `Serialize` collapses `Running(started)` into `Done(started.elapsed())`.
/// `Running` is never emitted to disk, so the on-wire representation
/// (`ToolStatusWire`) only carries `Done` and `Failed`.
#[derive(Debug, Clone)]
pub enum ToolStatus {
    Running(Instant),
    Done(Duration),
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "state", content = "value", rename_all = "snake_case")]
enum ToolStatusWire {
    Done(#[serde(with = "duration_ms_field")] Duration),
    Failed(String),
}

impl From<ToolStatusWire> for ToolStatus {
    fn from(w: ToolStatusWire) -> Self {
        match w {
            ToolStatusWire::Done(d) => ToolStatus::Done(d),
            ToolStatusWire::Failed(s) => ToolStatus::Failed(s),
        }
    }
}

impl Serialize for ToolStatus {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        // Collapse transient Running(Instant) into Done(elapsed) for persistence.
        let wire = match self {
            ToolStatus::Running(started) => ToolStatusWire::Done(started.elapsed()),
            ToolStatus::Done(d) => ToolStatusWire::Done(*d),
            ToolStatus::Failed(msg) => ToolStatusWire::Failed(msg.clone()),
        };
        wire.serialize(s)
    }
}

impl<'de> Deserialize<'de> for ToolStatus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        ToolStatusWire::deserialize(d).map(ToolStatus::from)
    }
}

/// A single message in the chat area.
#[derive(Debug, Clone)]
pub enum ChatMessage {
    User {
        text: String,
    },
    Assistant {
        text: String,
    },
    ToolCall {
        name: String,
        args: String,
        status: ToolStatus,
    },
    ToolResult {
        name: String,
        output: String,
    },
    System {
        text: String,
    },
}

impl Serialize for ChatMessage {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        match self {
            ChatMessage::User { text } => {
                let mut m = s.serialize_map(Some(2))?;
                m.serialize_entry("kind", "user")?;
                m.serialize_entry("text", text)?;
                m.end()
            }
            ChatMessage::Assistant { text } => {
                let mut m = s.serialize_map(Some(2))?;
                m.serialize_entry("kind", "assistant")?;
                m.serialize_entry("text", text)?;
                m.end()
            }
            ChatMessage::ToolCall { name, args, status } => {
                let mut m = s.serialize_map(Some(4))?;
                m.serialize_entry("kind", "tool_call")?;
                m.serialize_entry("name", name)?;
                m.serialize_entry("args", args)?;
                // ToolStatus::Serialize already collapses Running → Done(elapsed).
                m.serialize_entry("status", status)?;
                m.end()
            }
            ChatMessage::ToolResult { name, output } => {
                let mut m = s.serialize_map(Some(3))?;
                m.serialize_entry("kind", "tool_result")?;
                m.serialize_entry("name", name)?;
                m.serialize_entry("output", output)?;
                m.end()
            }
            ChatMessage::System { text } => {
                let mut m = s.serialize_map(Some(2))?;
                m.serialize_entry("kind", "system")?;
                m.serialize_entry("text", text)?;
                m.end()
            }
        }
    }
}

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum ChatMessageWire {
    User {
        text: String,
    },
    Assistant {
        text: String,
    },
    ToolCall {
        name: String,
        args: String,
        status: ToolStatus,
    },
    ToolResult {
        name: String,
        output: String,
    },
    System {
        text: String,
    },
}

impl<'de> Deserialize<'de> for ChatMessage {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(match ChatMessageWire::deserialize(d)? {
            ChatMessageWire::User { text } => ChatMessage::User { text },
            ChatMessageWire::Assistant { text } => ChatMessage::Assistant { text },
            ChatMessageWire::ToolCall { name, args, status } => {
                ChatMessage::ToolCall { name, args, status }
            }
            ChatMessageWire::ToolResult { name, output } => {
                ChatMessage::ToolResult { name, output }
            }
            ChatMessageWire::System { text } => ChatMessage::System { text },
        })
    }
}

/// Serde adaptor — store `Duration` as milliseconds.
///
/// Single shared copy used by both `ToolStatusWire` and `SessionMeta`
/// (re-exported via `super::duration_ms`).
pub(super) mod duration_ms_field {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::time::Duration;

    pub fn serialize<S: Serializer>(d: &Duration, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_u64(u64::try_from(d.as_millis()).unwrap_or(u64::MAX))
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Duration, D::Error> {
        let ms: u64 = Deserialize::deserialize(d)?;
        Ok(Duration::from_millis(ms))
    }
}

#[cfg(test)]
mod serde_tests {
    use super::*;
    use std::time::{Duration, Instant};

    #[test]
    fn roundtrip_user() {
        let m = ChatMessage::User { text: "hi".into() };
        let j = serde_json::to_string(&m).unwrap();
        let m2: ChatMessage = serde_json::from_str(&j).unwrap();
        match m2 {
            ChatMessage::User { text } => assert_eq!(text, "hi"),
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn roundtrip_tool_call_running_collapses_to_done() {
        let started = Instant::now()
            .checked_sub(Duration::from_secs(2))
            .expect("two seconds before now is representable");
        let m = ChatMessage::ToolCall {
            name: "shell".into(),
            args: "{}".into(),
            status: ToolStatus::Running(started),
        };
        let j = serde_json::to_string(&m).unwrap();
        let m2: ChatMessage = serde_json::from_str(&j).unwrap();
        match m2 {
            ChatMessage::ToolCall {
                status: ToolStatus::Done(d),
                ..
            } => {
                assert!(d.as_secs() >= 2, "elapsed preserved");
            }
            other => panic!("expected Done, got {other:?}"),
        }
    }

    #[test]
    fn roundtrip_tool_result_and_system_and_assistant() {
        for m in [
            ChatMessage::Assistant { text: "yo".into() },
            ChatMessage::ToolResult {
                name: "echo".into(),
                output: "hi".into(),
            },
            ChatMessage::System {
                text: "boot".into(),
            },
        ] {
            let j = serde_json::to_string(&m).unwrap();
            let _: ChatMessage = serde_json::from_str(&j).unwrap();
        }
    }
}
