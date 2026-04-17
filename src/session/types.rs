use std::path::PathBuf;
use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::SessionId;
use super::message::ChatMessage;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct MessageCounts {
    pub total: u32,
    pub user: u32,
    pub assistant: u32,
    pub tool_call: u32,
    pub tool_result: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMeta {
    pub id: SessionId,
    pub title: Option<String>,
    pub title_explicit: bool,
    pub cwd: PathBuf,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(with = "super::message::duration_ms_field")]
    pub duration: Duration,
    pub provider: Option<String>,
    pub model: Option<String>,
    pub counts: MessageCounts,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredMessage {
    pub seq: u32,
    pub ts: DateTime<Utc>,
    pub body: ChatMessage,
}

#[derive(Debug, Clone)]
pub struct Session {
    pub meta: SessionMeta,
    pub messages: Vec<StoredMessage>,
}

impl Serialize for SessionId {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for SessionId {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        SessionId::parse(&s).map_err(serde::de::Error::custom)
    }
}
