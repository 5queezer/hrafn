//! # ACP (Agent Communication Protocol) — v0.2.0 Implementation
//! Endpoints: /ping, /agents, /runs, /session

use crate::config::AcpCapability;
use axum::{Json, http::StatusCode, response::IntoResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

// ── Error Model ─────────────────────────────────────────────────

/// ACP error codes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AcpErrorCode {
    ServerError,
    InvalidInput,
    NotFound,
}

/// ACP error response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcpError {
    pub code: AcpErrorCode,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl IntoResponse for AcpError {
    fn into_response(self) -> axum::response::Response {
        let status = match self.code {
            AcpErrorCode::ServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AcpErrorCode::InvalidInput => StatusCode::BAD_REQUEST,
            AcpErrorCode::NotFound => StatusCode::NOT_FOUND,
        };
        (status, Json(self)).into_response()
    }
}

// ── Content Encoding ────────────────────────────────────────────

/// Encoding for message part content.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ContentEncoding {
    #[default]
    Plain,
    Base64,
}

// ── Part Metadata ───────────────────────────────────────────────

/// Citation metadata for a message part.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Trajectory metadata for a message part.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrajectoryMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_input: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_output: Option<Value>,
}

/// Tagged metadata for a message part.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum PartMetadata {
    #[serde(rename = "citation")]
    Citation(CitationMetadata),
    #[serde(rename = "trajectory")]
    Trajectory(TrajectoryMetadata),
}

// ── Message Types ───────────────────────────────────────────────

fn default_content_type() -> String {
    "text/plain".to_string()
}

/// A single part within an ACP message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePart {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default = "default_content_type")]
    pub content_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_encoding: Option<ContentEncoding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PartMetadata>,
}

/// An ACP message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub parts: Vec<MessagePart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
}

// ── Run Types ───────────────────────────────────────────────────

/// Status of an ACP run.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum RunStatus {
    Created,
    InProgress,
    Awaiting,
    Cancelling,
    Cancelled,
    Completed,
    Failed,
}

impl RunStatus {
    /// Whether this status represents a terminal state.
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            RunStatus::Completed | RunStatus::Failed | RunStatus::Cancelled
        )
    }
}

/// Mode for run execution.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RunMode {
    Sync,
    Async,
    Stream,
}

/// An ACP run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Run {
    pub agent_name: String,
    pub session_id: String,
    pub run_id: String,
    pub status: RunStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub await_request: Option<Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub output: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<AcpError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<DateTime<Utc>>,
}

/// Request to create a new run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunCreateRequest {
    pub agent_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    pub input: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<RunMode>,
}

/// Request to resume an awaiting run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunResumeRequest {
    pub run_id: String,
    pub await_resume: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<RunMode>,
}

fn default_limit() -> usize {
    10
}

/// Query parameters for listing agents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentsListQuery {
    #[serde(default = "default_limit")]
    pub limit: usize,
    #[serde(default)]
    pub offset: usize,
}

// ── Event Types (SSE) ───────────────────────────────────────────

/// Server-sent event types for ACP streaming.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Event {
    #[serde(rename = "message.created")]
    MessageCreated { message: Message },
    #[serde(rename = "message.part")]
    MessagePart { part: MessagePart },
    #[serde(rename = "message.completed")]
    MessageCompleted { message: Message },
    #[serde(rename = "run.created")]
    RunCreated { run: Run },
    #[serde(rename = "run.in-progress")]
    RunInProgress { run: Run },
    #[serde(rename = "run.awaiting")]
    RunAwaiting { run: Run },
    #[serde(rename = "run.completed")]
    RunCompleted { run: Run },
    #[serde(rename = "run.failed")]
    RunFailed { run: Run },
    #[serde(rename = "run.cancelled")]
    RunCancelled { run: Run },
    #[serde(rename = "error")]
    Error { error: AcpError },
    #[serde(rename = "generic")]
    Generic { data: Value },
}

// ── Response Wrappers ───────────────────────────────────────────

/// Metadata for an agent manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMetadata {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<AcpCapability>,
    #[serde(default)]
    pub framework: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

/// Public agent manifest returned by the agents list endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentManifest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub input_content_types: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub output_content_types: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<AgentMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Response for listing agents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentsListResponse {
    pub agents: Vec<AgentManifest>,
}

/// Response for listing run events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunEventsListResponse {
    pub events: Vec<Event>,
}

/// ACP session state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcpSession {
    pub id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub history: Vec<String>,
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn run_status_serializes_kebab_case() {
        let json = serde_json::to_string(&RunStatus::InProgress).unwrap();
        assert_eq!(json, "\"in-progress\"");
    }

    #[test]
    fn run_status_terminal() {
        assert!(RunStatus::Completed.is_terminal());
        assert!(RunStatus::Failed.is_terminal());
        assert!(RunStatus::Cancelled.is_terminal());
        assert!(!RunStatus::Created.is_terminal());
        assert!(!RunStatus::InProgress.is_terminal());
        assert!(!RunStatus::Awaiting.is_terminal());
        assert!(!RunStatus::Cancelling.is_terminal());
    }

    #[test]
    fn message_part_default_content_type() {
        let part: MessagePart = serde_json::from_str(r#"{"content": "hello"}"#).unwrap();
        assert_eq!(part.content_type, "text/plain");
    }

    #[test]
    fn event_serializes_with_type_tag() {
        let run = Run {
            agent_name: "test".into(),
            session_id: Uuid::new_v4().to_string(),
            run_id: Uuid::new_v4().to_string(),
            status: RunStatus::Created,
            await_request: None,
            output: vec![],
            error: None,
            created_at: None,
            finished_at: None,
        };
        let event = Event::RunCreated { run };
        let val: Value = serde_json::to_value(&event).unwrap();
        assert_eq!(val["type"], "run.created");
    }

    #[test]
    fn acp_error_code_serializes_snake_case() {
        let json = serde_json::to_string(&AcpErrorCode::InvalidInput).unwrap();
        assert_eq!(json, "\"invalid_input\"");
    }

    #[test]
    fn trajectory_metadata_tagged() {
        let meta = PartMetadata::Trajectory(TrajectoryMetadata {
            message: Some("step 1".into()),
            tool_name: None,
            tool_input: None,
            tool_output: None,
        });
        let val: Value = serde_json::to_value(&meta).unwrap();
        assert_eq!(val["kind"], "trajectory");
    }
}
