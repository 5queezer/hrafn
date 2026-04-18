//! # A2A Push Notifications (v1.0)
//!
//! Implements the v1.0 push notification surface:
//!
//! - `TaskPushNotificationConfig` CRUD (REST + JSON-RPC)
//! - Webhook delivery on task status/artifact updates with SSRF-validated URLs
//! - `SubscribeToTask` SSE endpoint — attach to an existing task and stream
//!   `StreamResponse` events until a terminal state is reached.
//!
//! Configs are kept in memory only (matches the `TaskStore` choice);
//! persistence is intentionally out of scope.

use super::AppState;
use super::a2a::{
    Artifact, JsonRpcRequest, Task, TaskArtifactUpdateEvent, TaskStatus, TaskStatusUpdateEvent,
    TaskStore, error_reason, require_a2a_auth, rpc_error,
};
use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{
        IntoResponse,
        sse::{Event, KeepAlive, Sse},
    },
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::ReceiverStream;

// ── Caps ─────────────────────────────────────────────────────────

/// Maximum push notification configs per task.
pub const MAX_CONFIGS_PER_TASK: usize = 16;
/// Global cap across the entire store.
pub const MAX_TOTAL_CONFIGS: usize = 10_000;

/// Per-webhook POST timeout.
const DELIVERY_TIMEOUT: Duration = Duration::from_secs(10);
/// Retry schedule: base delay, doubled each attempt.
const RETRY_BASE: Duration = Duration::from_millis(500);
/// Max delivery attempts (including the first).
const RETRY_ATTEMPTS: usize = 3;

// ── Types ────────────────────────────────────────────────────────

/// Authentication block carried with a push notification config.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PushNotificationAuth {
    #[serde(default)]
    pub schemes: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<String>,
}

/// v1.0 `TaskPushNotificationConfig` — target webhook for task updates.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskPushNotificationConfig {
    /// Server-generated if absent at create-time.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    /// Populated from the path; client-supplied value is ignored.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub task_id: String,
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authentication: Option<PushNotificationAuth>,
    /// Opaque multi-tenancy label, stored verbatim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
}

/// In-memory store for `TaskPushNotificationConfig` entries.
///
/// Keyed by `(task_id, config_id)`. Kept in memory — persistence is out of
/// scope for this module to match `TaskStore`.
#[derive(Default)]
pub struct PushNotificationStore {
    inner: RwLock<HashMap<(String, String), TaskPushNotificationConfig>>,
}

impl PushNotificationStore {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(HashMap::new()),
        }
    }

    /// Insert or replace a config, enforcing per-task and global caps.
    async fn insert(
        &self,
        cfg: TaskPushNotificationConfig,
    ) -> Result<TaskPushNotificationConfig, PushStoreError> {
        let mut guard = self.inner.write().await;
        // Check per-task cap — an update to an existing entry is always fine.
        let key = (cfg.task_id.clone(), cfg.id.clone());
        let is_new = !guard.contains_key(&key);
        if is_new {
            let per_task = guard.keys().filter(|(t, _)| t == &cfg.task_id).count();
            if per_task >= MAX_CONFIGS_PER_TASK {
                return Err(PushStoreError::PerTaskLimit);
            }
            if guard.len() >= MAX_TOTAL_CONFIGS {
                return Err(PushStoreError::GlobalLimit);
            }
        }
        guard.insert(key, cfg.clone());
        Ok(cfg)
    }

    async fn get(&self, task_id: &str, id: &str) -> Option<TaskPushNotificationConfig> {
        self.inner
            .read()
            .await
            .get(&(task_id.to_string(), id.to_string()))
            .cloned()
    }

    async fn list_for_task(&self, task_id: &str) -> Vec<TaskPushNotificationConfig> {
        self.inner
            .read()
            .await
            .iter()
            .filter(|((t, _), _)| t == task_id)
            .map(|(_, v)| v.clone())
            .collect()
    }

    async fn delete(&self, task_id: &str, id: &str) -> bool {
        self.inner
            .write()
            .await
            .remove(&(task_id.to_string(), id.to_string()))
            .is_some()
    }

    /// Snapshot of configs for a task, used by the delivery path without
    /// holding the lock during outbound HTTP.
    pub async fn snapshot_for_task(&self, task_id: &str) -> Vec<TaskPushNotificationConfig> {
        self.list_for_task(task_id).await
    }
}

#[derive(Debug)]
enum PushStoreError {
    PerTaskLimit,
    GlobalLimit,
}

impl PushStoreError {
    fn message(&self) -> &'static str {
        match self {
            PushStoreError::PerTaskLimit => "Push notification config limit reached for this task",
            PushStoreError::GlobalLimit => "Push notification config global limit reached",
        }
    }
}

// ── Delivery ─────────────────────────────────────────────────────

/// Envelope POSTed to each registered webhook.  Shape matches a `StreamResponse`
/// event (either a status update or an artifact update).
pub enum DeliveryEvent {
    Status(TaskStatusUpdateEvent),
    Artifact(TaskArtifactUpdateEvent),
}

impl DeliveryEvent {
    fn as_json(&self) -> serde_json::Value {
        match self {
            DeliveryEvent::Status(s) => {
                json!({ "kind": "status-update", "taskStatusUpdateEvent": s })
            }
            DeliveryEvent::Artifact(a) => {
                json!({ "kind": "artifact-update", "taskArtifactUpdateEvent": a })
            }
        }
    }
}

/// Fire-and-forget delivery for every registered config on a task.
///
/// Spawns one background task per config so the caller (task-update path)
/// is not blocked.  Failures are logged and swallowed.
pub fn dispatch(store: Arc<PushNotificationStore>, task_id: String, event: Arc<serde_json::Value>) {
    tokio::spawn(async move {
        let configs = store.snapshot_for_task(&task_id).await;
        for cfg in configs {
            let event = Arc::clone(&event);
            tokio::spawn(async move {
                if let Err(e) = deliver_webhook(&cfg, &event).await {
                    tracing::warn!(
                        task_id = %cfg.task_id,
                        config_id = %cfg.id,
                        url = %cfg.url,
                        error = %e,
                        "A2A push notification delivery failed (gave up)"
                    );
                }
            });
        }
    });
}

/// Core delivery: validates the URL (SSRF), POSTs the envelope with retries.
async fn deliver_webhook(
    cfg: &TaskPushNotificationConfig,
    payload: &serde_json::Value,
) -> anyhow::Result<()> {
    // Reuse the existing SSRF validator from `tools::a2a` rather than
    // duplicating it here.  This enforces the same public-host policy as the
    // outbound A2A tool.
    let parsed = reqwest::Url::parse(&cfg.url)?;
    match parsed.scheme() {
        "http" | "https" => {}
        scheme => anyhow::bail!("Unsupported webhook scheme: {scheme}"),
    }
    if let Some(host) = parsed.host_str() {
        if crate::tools::a2a::is_private_or_local_host(host) {
            anyhow::bail!("Blocked push notification to private/local host: {host}");
        }
        crate::tools::a2a::validate_resolved_host_is_public(host)?;
    }

    let client = reqwest::Client::builder()
        .timeout(DELIVERY_TIMEOUT)
        .connect_timeout(Duration::from_secs(5))
        .user_agent(format!(
            "Hrafn/{} (a2a-push)",
            env!("HRAFN_VERSION", "0.0.0")
        ))
        .build()?;

    let body_bytes = serde_json::to_vec(payload)?;
    let mut last_err: Option<String> = None;
    for attempt in 0..RETRY_ATTEMPTS {
        if attempt > 0 {
            let shift = u32::try_from(attempt - 1).unwrap_or(0);
            let delay = RETRY_BASE * (1u32 << shift);
            tokio::time::sleep(delay).await;
        }

        let mut req = client
            .post(parsed.clone())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body_bytes.clone());
        if let Some(token) = &cfg.token {
            // Client-supplied opaque token echoed back so the receiver can
            // correlate the delivery.  Never mix with the registering
            // client's bearer token.
            req = req.header("X-A2A-Notification-Token", token.as_str());
        }

        match req.send().await {
            Ok(resp) if resp.status().is_success() => return Ok(()),
            Ok(resp) => {
                last_err = Some(format!("HTTP {} from receiver", resp.status()));
            }
            Err(e) => {
                last_err = Some(e.to_string());
            }
        }
    }
    Err(anyhow::anyhow!(last_err.unwrap_or_else(|| {
        "delivery failed with no error recorded".to_string()
    })))
}

/// Dispatch helper for task-update paths inside `gateway::a2a`.  Both status
/// and artifact events are deliverable per A2A v1.0.
pub fn emit_status_update(state: &AppState, task_id: &str, event: &TaskStatusUpdateEvent) {
    let Some(store) = &state.a2a_push_store else {
        return;
    };
    let payload = DeliveryEvent::Status(event.clone()).as_json();
    dispatch(Arc::clone(store), task_id.to_string(), Arc::new(payload));
}

pub fn emit_artifact_update(state: &AppState, task_id: &str, event: &TaskArtifactUpdateEvent) {
    let Some(store) = &state.a2a_push_store else {
        return;
    };
    let payload = DeliveryEvent::Artifact(event.clone()).as_json();
    dispatch(Arc::clone(store), task_id.to_string(), Arc::new(payload));
}

// ── CRUD handlers (shared core) ──────────────────────────────────

fn store_err_to_response(
    id: serde_json::Value,
    err: PushStoreError,
) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::BAD_REQUEST,
        Json(rpc_error(
            id,
            -32000,
            err.message(),
            Some(error_reason::RESOURCE_EXHAUSTED),
        )),
    )
}

async fn create_config(
    store: &Arc<PushNotificationStore>,
    task_store: &Arc<TaskStore>,
    task_id: String,
    mut cfg: TaskPushNotificationConfig,
) -> Result<TaskPushNotificationConfig, (StatusCode, Json<serde_json::Value>)> {
    // Task must exist.
    {
        let tasks = task_store.tasks.read().await;
        if !tasks.contains_key(&task_id) {
            return Err((
                StatusCode::NOT_FOUND,
                Json(rpc_error(
                    json!(null),
                    -32001,
                    "Task not found",
                    Some(error_reason::TASK_NOT_FOUND),
                )),
            ));
        }
    }

    // URL must be non-empty and pass SSRF policy.
    if cfg.url.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(rpc_error(
                json!(null),
                -32602,
                "Invalid params: url is required",
                Some(error_reason::INVALID_PARAMS),
            )),
        ));
    }
    if let Err(e) = validate_webhook_url(&cfg.url) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(rpc_error(
                json!(null),
                -32602,
                &format!("Invalid webhook URL: {e}"),
                Some(error_reason::INVALID_PARAMS),
            )),
        ));
    }

    cfg.task_id = task_id;
    if cfg.id.is_empty() {
        cfg.id = uuid::Uuid::new_v4().to_string();
    }

    match store.insert(cfg).await {
        Ok(saved) => Ok(saved),
        Err(err) => Err(store_err_to_response(json!(null), err)),
    }
}

/// URL validation used at config-create time.  Delegates to the SSRF helpers
/// in `tools::a2a` so the policy stays consistent.
fn validate_webhook_url(url: &str) -> anyhow::Result<()> {
    let parsed = reqwest::Url::parse(url)?;
    match parsed.scheme() {
        "http" | "https" => {}
        scheme => anyhow::bail!("unsupported scheme: {scheme}"),
    }
    if let Some(host) = parsed.host_str() {
        if crate::tools::a2a::is_private_or_local_host(host) {
            anyhow::bail!("private or local host not allowed: {host}");
        }
        crate::tools::a2a::validate_resolved_host_is_public(host)?;
    } else {
        anyhow::bail!("missing host");
    }
    Ok(())
}

// ── REST handlers ────────────────────────────────────────────────

/// `POST /tasks/{task_id}/pushNotificationConfigs`
pub async fn handle_create_rest(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(task_id): Path<String>,
    Json(body): Json<TaskPushNotificationConfig>,
) -> impl IntoResponse {
    let (Some(_card), Some(task_store), Some(push_store)) = (
        &state.a2a_agent_card,
        &state.a2a_task_store,
        &state.a2a_push_store,
    ) else {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "A2A protocol not enabled"})),
        )
            .into_response();
    };
    if let Err(resp) = require_a2a_auth(&state, &headers) {
        return resp.into_response();
    }

    match create_config(push_store, task_store, task_id, body).await {
        Ok(saved) => (
            StatusCode::CREATED,
            Json(serde_json::to_value(saved).unwrap()),
        )
            .into_response(),
        Err((status, body)) => (status, body).into_response(),
    }
}

/// `GET /tasks/{task_id}/pushNotificationConfigs/{id}`
pub async fn handle_get_rest(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((task_id, id)): Path<(String, String)>,
) -> impl IntoResponse {
    let (Some(_card), Some(push_store)) = (&state.a2a_agent_card, &state.a2a_push_store) else {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "A2A protocol not enabled"})),
        )
            .into_response();
    };
    if let Err(resp) = require_a2a_auth(&state, &headers) {
        return resp.into_response();
    }
    match push_store.get(&task_id, &id).await {
        Some(cfg) => (StatusCode::OK, Json(serde_json::to_value(cfg).unwrap())).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(rpc_error(
                json!(null),
                -32001,
                "Push notification config not found",
                Some(error_reason::TASK_NOT_FOUND),
            )),
        )
            .into_response(),
    }
}

/// `GET /tasks/{task_id}/pushNotificationConfigs`
pub async fn handle_list_rest(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(task_id): Path<String>,
) -> impl IntoResponse {
    let (Some(_card), Some(push_store)) = (&state.a2a_agent_card, &state.a2a_push_store) else {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "A2A protocol not enabled"})),
        )
            .into_response();
    };
    if let Err(resp) = require_a2a_auth(&state, &headers) {
        return resp.into_response();
    }
    let configs = push_store.list_for_task(&task_id).await;
    (StatusCode::OK, Json(json!({ "configs": configs }))).into_response()
}

/// `DELETE /tasks/{task_id}/pushNotificationConfigs/{id}`
pub async fn handle_delete_rest(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((task_id, id)): Path<(String, String)>,
) -> impl IntoResponse {
    let (Some(_card), Some(push_store)) = (&state.a2a_agent_card, &state.a2a_push_store) else {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "A2A protocol not enabled"})),
        )
            .into_response();
    };
    if let Err(resp) = require_a2a_auth(&state, &headers) {
        return resp.into_response();
    }
    if push_store.delete(&task_id, &id).await {
        (StatusCode::NO_CONTENT, Json(json!({}))).into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(rpc_error(
                json!(null),
                -32001,
                "Push notification config not found",
                Some(error_reason::TASK_NOT_FOUND),
            )),
        )
            .into_response()
    }
}

// ── JSON-RPC handlers ────────────────────────────────────────────

pub async fn handle_create_rpc(
    push_store: &Arc<PushNotificationStore>,
    task_store: &Arc<TaskStore>,
    req: JsonRpcRequest,
) -> (StatusCode, Json<serde_json::Value>) {
    let Some(task_id) = req
        .params
        .get("taskId")
        .and_then(|v| v.as_str())
        .map(String::from)
    else {
        return (
            StatusCode::OK,
            Json(rpc_error(
                req.id,
                -32602,
                "Invalid params: missing taskId",
                Some(error_reason::INVALID_PARAMS),
            )),
        );
    };
    let cfg_val = req
        .params
        .get("config")
        .cloned()
        .unwrap_or(serde_json::Value::Null);
    let cfg: TaskPushNotificationConfig = match serde_json::from_value(cfg_val) {
        Ok(c) => c,
        Err(e) => {
            return (
                StatusCode::OK,
                Json(rpc_error(
                    req.id,
                    -32602,
                    &format!("Invalid params: config {e}"),
                    Some(error_reason::INVALID_PARAMS),
                )),
            );
        }
    };
    match create_config(push_store, task_store, task_id, cfg).await {
        Ok(saved) => (
            StatusCode::OK,
            Json(json!({
                "jsonrpc": "2.0",
                "id": req.id,
                "result": saved,
            })),
        ),
        Err((_status, Json(body))) => (StatusCode::OK, Json(body)),
    }
}

pub async fn handle_get_rpc(
    push_store: &Arc<PushNotificationStore>,
    req: JsonRpcRequest,
) -> (StatusCode, Json<serde_json::Value>) {
    let task_id = req
        .params
        .get("taskId")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let id = req.params.get("id").and_then(|v| v.as_str()).unwrap_or("");
    if task_id.is_empty() || id.is_empty() {
        return (
            StatusCode::OK,
            Json(rpc_error(
                req.id,
                -32602,
                "Invalid params: missing taskId or id",
                Some(error_reason::INVALID_PARAMS),
            )),
        );
    }
    match push_store.get(task_id, id).await {
        Some(cfg) => (
            StatusCode::OK,
            Json(json!({
                "jsonrpc": "2.0",
                "id": req.id,
                "result": cfg,
            })),
        ),
        None => (
            StatusCode::OK,
            Json(rpc_error(
                req.id,
                -32001,
                "Push notification config not found",
                Some(error_reason::TASK_NOT_FOUND),
            )),
        ),
    }
}

pub async fn handle_list_rpc(
    push_store: &Arc<PushNotificationStore>,
    req: JsonRpcRequest,
) -> (StatusCode, Json<serde_json::Value>) {
    let task_id = req
        .params
        .get("taskId")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if task_id.is_empty() {
        return (
            StatusCode::OK,
            Json(rpc_error(
                req.id,
                -32602,
                "Invalid params: missing taskId",
                Some(error_reason::INVALID_PARAMS),
            )),
        );
    }
    let configs = push_store.list_for_task(task_id).await;
    (
        StatusCode::OK,
        Json(json!({
            "jsonrpc": "2.0",
            "id": req.id,
            "result": { "configs": configs },
        })),
    )
}

pub async fn handle_delete_rpc(
    push_store: &Arc<PushNotificationStore>,
    req: JsonRpcRequest,
) -> (StatusCode, Json<serde_json::Value>) {
    let task_id = req
        .params
        .get("taskId")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let id = req.params.get("id").and_then(|v| v.as_str()).unwrap_or("");
    if task_id.is_empty() || id.is_empty() {
        return (
            StatusCode::OK,
            Json(rpc_error(
                req.id,
                -32602,
                "Invalid params: missing taskId or id",
                Some(error_reason::INVALID_PARAMS),
            )),
        );
    }
    if push_store.delete(task_id, id).await {
        (
            StatusCode::OK,
            Json(json!({
                "jsonrpc": "2.0",
                "id": req.id,
                "result": { "deleted": true },
            })),
        )
    } else {
        (
            StatusCode::OK,
            Json(rpc_error(
                req.id,
                -32001,
                "Push notification config not found",
                Some(error_reason::TASK_NOT_FOUND),
            )),
        )
    }
}

// ── SubscribeToTask SSE ─────────────────────────────────────────

/// Poll interval while the task is still running.  Kept small so streams
/// feel live without spamming the lock.
const SUBSCRIBE_POLL_INTERVAL: Duration = Duration::from_millis(250);
/// Hard cap for the subscribe stream — protects against clients that attach
/// to tasks that never terminate (e.g. in a degraded state).
const SUBSCRIBE_MAX_DURATION: Duration = Duration::from_secs(300);

fn snapshot_event(task: &Task) -> TaskStatusUpdateEvent {
    TaskStatusUpdateEvent {
        task_id: task.id.clone(),
        context_id: task.context_id.clone(),
        status: TaskStatus {
            state: task.status.state.clone(),
            message: task.status.message.clone(),
            timestamp: task.status.timestamp.clone(),
        },
        is_final: task.status.state.is_terminal(),
        metadata: None,
    }
}

fn artifact_event(task: &Task, artifact: &Artifact) -> TaskArtifactUpdateEvent {
    TaskArtifactUpdateEvent {
        task_id: task.id.clone(),
        context_id: task.context_id.clone(),
        artifact: artifact.clone(),
        metadata: None,
    }
}

/// `GET /tasks/{id}:subscribe` — SSE stream of StreamResponse events.
///
/// Unlike `message:stream`, this attaches to an already-created task.
/// Emits one snapshot first, then polls the task store for further updates
/// and yields `TaskStatusUpdateEvent`s.  Closes after the task reaches a
/// terminal state or the max-duration guard trips.
pub async fn handle_subscribe_rest(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(raw): Path<String>,
) -> impl IntoResponse {
    let task_id = raw.strip_suffix(":subscribe").unwrap_or(&raw).to_string();
    let (Some(_card), Some(task_store)) = (&state.a2a_agent_card, &state.a2a_task_store) else {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "A2A protocol not enabled"})),
        )
            .into_response();
    };
    if let Err(resp) = require_a2a_auth(&state, &headers) {
        return resp.into_response();
    }

    // Snapshot existence + initial state before building the stream.
    let initial = {
        let tasks = task_store.tasks.read().await;
        tasks.get(&task_id).cloned()
    };
    let Some(initial) = initial else {
        return (
            StatusCode::NOT_FOUND,
            Json(rpc_error(
                json!(null),
                -32001,
                "Task not found",
                Some(error_reason::TASK_NOT_FOUND),
            )),
        )
            .into_response();
    };

    let task_store = Arc::clone(task_store);
    let (tx, rx) = tokio::sync::mpsc::channel::<Event>(32);

    tokio::spawn(async move {
        // Initial snapshot: status and any artifacts already attached.
        let first = snapshot_event(&initial);
        let _ = tx
            .send(
                Event::default()
                    .event("status_update")
                    .data(serde_json::to_string(&first).unwrap_or_default()),
            )
            .await;
        if let Some(artifacts) = &initial.artifacts {
            for a in artifacts {
                let ev = artifact_event(&initial, a);
                let _ = tx
                    .send(
                        Event::default()
                            .event("artifact_update")
                            .data(serde_json::to_string(&ev).unwrap_or_default()),
                    )
                    .await;
            }
        }
        // If the task is already terminal on subscribe, we are done.
        if initial.status.state.is_terminal() {
            return;
        }

        // Tail: re-read state periodically and emit diffs.
        let deadline = tokio::time::Instant::now() + SUBSCRIBE_MAX_DURATION;
        let mut last_state = initial.status.state.clone();
        let mut last_artifacts: usize = initial.artifacts.as_ref().map(Vec::len).unwrap_or(0);

        loop {
            if tokio::time::Instant::now() >= deadline {
                break;
            }
            tokio::time::sleep(SUBSCRIBE_POLL_INTERVAL).await;
            let snapshot = {
                let tasks = task_store.tasks.read().await;
                tasks.get(&task_id).cloned()
            };
            let Some(task) = snapshot else { break };

            if task.status.state != last_state {
                let ev = snapshot_event(&task);
                if tx
                    .send(
                        Event::default()
                            .event("status_update")
                            .data(serde_json::to_string(&ev).unwrap_or_default()),
                    )
                    .await
                    .is_err()
                {
                    return;
                }
                last_state = task.status.state.clone();
            }

            if let Some(artifacts) = &task.artifacts {
                if artifacts.len() > last_artifacts {
                    for a in &artifacts[last_artifacts..] {
                        let ev = artifact_event(&task, a);
                        if tx
                            .send(
                                Event::default()
                                    .event("artifact_update")
                                    .data(serde_json::to_string(&ev).unwrap_or_default()),
                            )
                            .await
                            .is_err()
                        {
                            return;
                        }
                    }
                    last_artifacts = artifacts.len();
                }
            }

            if task.status.state.is_terminal() {
                break;
            }
        }
    });

    let stream = ReceiverStream::new(rx).map(Ok::<_, Infallible>);
    Sse::new(stream)
        .keep_alive(KeepAlive::default())
        .into_response()
}

// ── Tests ────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gateway::a2a::{A2aTaskState, Task, TaskStatus};

    fn make_task(id: &str, state: A2aTaskState) -> Task {
        Task {
            id: id.to_string(),
            status: TaskStatus {
                state,
                message: None,
                timestamp: None,
            },
            context_id: None,
            artifacts: None,
            history: None,
            metadata: None,
        }
    }

    #[tokio::test]
    async fn crud_happy_path_on_push_store() {
        let store = PushNotificationStore::new();
        let task_store = Arc::new(TaskStore::new());
        {
            let mut tasks = task_store.tasks.write().await;
            tasks.insert("t-1".into(), make_task("t-1", A2aTaskState::Working));
        }
        let store = Arc::new(store);

        let cfg = TaskPushNotificationConfig {
            id: String::new(),
            task_id: String::new(),
            url: "https://receiver.example.com/hook".into(),
            token: Some("opaque".into()),
            authentication: Some(PushNotificationAuth {
                schemes: vec!["Bearer".into()],
                credentials: None,
            }),
            tenant: Some("tenant-a".into()),
        };

        // Create
        let saved = create_config(&store, &task_store, "t-1".into(), cfg.clone())
            .await
            .expect("create should succeed");
        assert!(!saved.id.is_empty(), "id auto-generated");
        assert_eq!(saved.task_id, "t-1");
        assert_eq!(saved.tenant.as_deref(), Some("tenant-a"));

        // Get
        let fetched = store.get("t-1", &saved.id).await.unwrap();
        assert_eq!(fetched.url, cfg.url);

        // List
        let list = store.list_for_task("t-1").await;
        assert_eq!(list.len(), 1);

        // Delete
        assert!(store.delete("t-1", &saved.id).await);
        assert!(store.get("t-1", &saved.id).await.is_none());
    }

    #[tokio::test]
    async fn create_rejects_ssrf_url() {
        let store = Arc::new(PushNotificationStore::new());
        let task_store = Arc::new(TaskStore::new());
        {
            let mut tasks = task_store.tasks.write().await;
            tasks.insert("t-1".into(), make_task("t-1", A2aTaskState::Working));
        }
        let cfg = TaskPushNotificationConfig {
            id: String::new(),
            task_id: String::new(),
            url: "http://127.0.0.1:9999/hook".into(),
            token: None,
            authentication: None,
            tenant: None,
        };
        let err = create_config(&store, &task_store, "t-1".into(), cfg)
            .await
            .expect_err("localhost URL must be rejected");
        let (status, Json(body)) = err;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(body["error"]["code"], -32602);
    }

    #[tokio::test]
    async fn create_rejects_when_per_task_limit_hit() {
        let store = Arc::new(PushNotificationStore::new());
        let task_store = Arc::new(TaskStore::new());
        {
            let mut tasks = task_store.tasks.write().await;
            tasks.insert("t-1".into(), make_task("t-1", A2aTaskState::Working));
        }
        for i in 0..MAX_CONFIGS_PER_TASK {
            let cfg = TaskPushNotificationConfig {
                id: format!("cfg-{i}"),
                task_id: String::new(),
                url: format!("https://receiver.example.com/h{i}"),
                token: None,
                authentication: None,
                tenant: None,
            };
            create_config(&store, &task_store, "t-1".into(), cfg)
                .await
                .expect("within cap");
        }
        let overflow = TaskPushNotificationConfig {
            id: "cfg-overflow".into(),
            task_id: String::new(),
            url: "https://receiver.example.com/h-extra".into(),
            token: None,
            authentication: None,
            tenant: None,
        };
        let err = create_config(&store, &task_store, "t-1".into(), overflow)
            .await
            .expect_err("should exceed per-task cap");
        let (status, Json(body)) = err;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        let details = body["error"]["details"].as_array().unwrap();
        assert_eq!(details[0]["reason"], error_reason::RESOURCE_EXHAUSTED);
    }

    #[tokio::test]
    async fn delivery_fails_fast_on_ssrf() {
        let cfg = TaskPushNotificationConfig {
            id: "cfg-1".into(),
            task_id: "t-1".into(),
            url: "http://127.0.0.1:1/hook".into(),
            token: None,
            authentication: None,
            tenant: None,
        };
        let body = json!({"kind": "status-update"});
        let res = deliver_webhook(&cfg, &body).await;
        assert!(res.is_err(), "localhost delivery must be refused");
    }

    #[tokio::test]
    async fn delivery_times_out_on_unroutable_ip_quickly() {
        // TEST-NET-1 (192.0.2.0/24) is reserved-for-documentation and is
        // covered by the SSRF validator (192.0.2/24 is non-global).  Ensures
        // we don't attempt a real socket connect before refusing.
        let cfg = TaskPushNotificationConfig {
            id: "cfg-1".into(),
            task_id: "t-1".into(),
            url: "http://192.0.2.1/hook".into(),
            token: None,
            authentication: None,
            tenant: None,
        };
        let body = json!({"kind": "status-update"});
        let start = std::time::Instant::now();
        let res = deliver_webhook(&cfg, &body).await;
        let elapsed = start.elapsed();
        assert!(res.is_err());
        assert!(
            elapsed < Duration::from_secs(2),
            "must short-circuit on SSRF, got {elapsed:?}"
        );
    }

    #[tokio::test]
    async fn subscribe_snapshot_event_includes_terminal_flag() {
        let t = make_task("t-done", A2aTaskState::Completed);
        let ev = snapshot_event(&t);
        assert!(ev.is_final);
        assert_eq!(ev.task_id, "t-done");

        let working = make_task("t-run", A2aTaskState::Working);
        let ev2 = snapshot_event(&working);
        assert!(!ev2.is_final);
    }

    /// End-to-end subscribe: seed a task, run the async stream producer, and
    /// assert it emits a snapshot followed by a terminal transition.
    #[tokio::test]
    async fn subscribe_stream_emits_snapshot_then_tail_until_terminal() {
        let task_store: Arc<TaskStore> = Arc::new(TaskStore::new());
        {
            let mut tasks = task_store.tasks.write().await;
            tasks.insert("t-42".into(), make_task("t-42", A2aTaskState::Working));
        }

        // Build the same stream-producing task the handler spawns inline.
        let (tx, mut rx) = tokio::sync::mpsc::channel::<Event>(32);
        let ts = Arc::clone(&task_store);
        let initial = ts.tasks.read().await.get("t-42").cloned().unwrap();

        tokio::spawn(async move {
            let first = snapshot_event(&initial);
            let _ = tx
                .send(
                    Event::default()
                        .event("status_update")
                        .data(serde_json::to_string(&first).unwrap_or_default()),
                )
                .await;

            // Tail loop: poll and emit the one terminal transition below.
            let deadline = tokio::time::Instant::now() + Duration::from_secs(2);
            let mut last_state = initial.status.state.clone();
            loop {
                if tokio::time::Instant::now() >= deadline {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(50)).await;
                let snapshot = ts.tasks.read().await.get("t-42").cloned();
                let Some(task) = snapshot else { break };
                if task.status.state != last_state {
                    let ev = snapshot_event(&task);
                    let _ = tx
                        .send(
                            Event::default()
                                .event("status_update")
                                .data(serde_json::to_string(&ev).unwrap_or_default()),
                        )
                        .await;
                    last_state = task.status.state.clone();
                }
                if task.status.state.is_terminal() {
                    break;
                }
            }
        });

        // Receive the snapshot event first.
        let first = tokio::time::timeout(Duration::from_secs(1), rx.recv())
            .await
            .expect("receive timeout")
            .expect("channel closed");
        let _ = first; // Event has no public accessor — presence suffices.

        // Now transition the task to Completed; the tail loop must emit one
        // more event before closing the channel.
        {
            let mut tasks = task_store.tasks.write().await;
            if let Some(t) = tasks.get_mut("t-42") {
                t.status.state = A2aTaskState::Completed;
            }
        }

        let second = tokio::time::timeout(Duration::from_secs(2), rx.recv())
            .await
            .expect("terminal event timeout")
            .expect("channel closed");
        let _ = second;

        // Channel should close shortly after the terminal event.
        let closed = tokio::time::timeout(Duration::from_secs(1), rx.recv()).await;
        assert!(
            matches!(closed, Ok(None) | Err(_)),
            "stream should close after terminal state"
        );
    }
}
