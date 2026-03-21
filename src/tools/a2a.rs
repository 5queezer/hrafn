//! A2A (Agent-to-Agent) protocol client tool.
//!
//! Enables the agent to discover and communicate with remote A2A-compatible
//! agents via the A2A open standard (Linux Foundation). Supports four actions:
//! `discover`, `send`, `status`, and `result`.

use super::traits::{Tool, ToolResult};
use crate::security::SecurityPolicy;
use async_trait::async_trait;
use serde_json::json;
use std::sync::Arc;

/// Outbound A2A client tool — discovers remote agents and sends/retrieves tasks.
pub struct A2aTool {
    security: Arc<SecurityPolicy>,
    timeout_secs: u64,
}

impl A2aTool {
    pub fn new(security: Arc<SecurityPolicy>, timeout_secs: u64) -> Self {
        Self {
            security,
            timeout_secs,
        }
    }

    fn build_client(&self) -> anyhow::Result<reqwest::Client> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(self.timeout_secs))
            .build()?;
        Ok(client)
    }

    fn validate_url(url: &str) -> anyhow::Result<reqwest::Url> {
        let parsed = reqwest::Url::parse(url)?;
        match parsed.scheme() {
            "http" | "https" => Ok(parsed),
            scheme => anyhow::bail!("Unsupported URL scheme: {scheme} (only http/https allowed)"),
        }
    }

    async fn action_discover(
        &self,
        url: &str,
        bearer_token: Option<&str>,
    ) -> anyhow::Result<ToolResult> {
        let base = Self::validate_url(url)?;
        let card_url = base.join("/.well-known/agent-card.json")?;
        let client = self.build_client()?;

        let mut req = client.get(card_url);
        if let Some(token) = bearer_token {
            req = req.bearer_auth(token);
        }

        let resp = req.send().await?;
        let status = resp.status();
        let body = resp.text().await?;

        if status.is_success() {
            Ok(ToolResult {
                success: true,
                output: body,
                error: None,
            })
        } else {
            Ok(ToolResult {
                success: false,
                output: String::new(),
                error: Some(format!("HTTP {status}: {body}")),
            })
        }
    }

    async fn action_send(
        &self,
        url: &str,
        bearer_token: Option<&str>,
        message: &str,
    ) -> anyhow::Result<ToolResult> {
        let base = Self::validate_url(url)?;
        let rpc_url = base.join("/a2a")?;
        let client = self.build_client()?;
        let request_id = uuid::Uuid::new_v4().to_string();

        let body = json!({
            "jsonrpc": "2.0",
            "id": request_id,
            "method": "tasks/send",
            "params": {
                "message": {
                    "role": "user",
                    "parts": [{ "type": "text", "text": message }]
                }
            }
        });

        let mut req = client.post(rpc_url).json(&body);
        if let Some(token) = bearer_token {
            req = req.bearer_auth(token);
        }

        let resp = req.send().await?;
        let status = resp.status();
        let resp_body = resp.text().await?;

        if status.is_success() {
            Ok(ToolResult {
                success: true,
                output: resp_body,
                error: None,
            })
        } else {
            Ok(ToolResult {
                success: false,
                output: String::new(),
                error: Some(format!("HTTP {status}: {resp_body}")),
            })
        }
    }

    async fn action_get_task(
        &self,
        url: &str,
        bearer_token: Option<&str>,
        task_id: &str,
    ) -> anyhow::Result<serde_json::Value> {
        let base = Self::validate_url(url)?;
        let rpc_url = base.join("/a2a")?;
        let client = self.build_client()?;
        let request_id = uuid::Uuid::new_v4().to_string();

        let body = json!({
            "jsonrpc": "2.0",
            "id": request_id,
            "method": "tasks/get",
            "params": { "id": task_id }
        });

        let mut req = client.post(rpc_url).json(&body);
        if let Some(token) = bearer_token {
            req = req.bearer_auth(token);
        }

        let resp = req.send().await?;
        let status = resp.status();
        let resp_body = resp.text().await?;

        if status.is_success() {
            let parsed: serde_json::Value = serde_json::from_str(&resp_body)?;
            Ok(parsed)
        } else {
            anyhow::bail!("HTTP {status}: {resp_body}");
        }
    }

    async fn action_status(
        &self,
        url: &str,
        bearer_token: Option<&str>,
        task_id: &str,
    ) -> anyhow::Result<ToolResult> {
        match self.action_get_task(url, bearer_token, task_id).await {
            Ok(resp) => Ok(ToolResult {
                success: true,
                output: serde_json::to_string_pretty(&resp)?,
                error: None,
            }),
            Err(e) => Ok(ToolResult {
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
            }),
        }
    }

    async fn action_result(
        &self,
        url: &str,
        bearer_token: Option<&str>,
        task_id: &str,
    ) -> anyhow::Result<ToolResult> {
        match self.action_get_task(url, bearer_token, task_id).await {
            Ok(resp) => {
                // Extract artifacts from the task response
                let artifacts = resp
                    .pointer("/result/artifacts")
                    .or_else(|| resp.pointer("/artifacts"))
                    .cloned()
                    .unwrap_or(json!([]));
                Ok(ToolResult {
                    success: true,
                    output: serde_json::to_string_pretty(&artifacts)?,
                    error: None,
                })
            }
            Err(e) => Ok(ToolResult {
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
            }),
        }
    }
}

#[async_trait]
impl Tool for A2aTool {
    fn name(&self) -> &str {
        "a2a"
    }

    fn description(&self) -> &str {
        "Communicate with remote agents via the A2A (Agent-to-Agent) protocol. \
         Supports four actions: 'discover' to fetch a remote agent's capability card, \
         'send' to dispatch a task message, 'status' to check task progress, and \
         'result' to retrieve task output artifacts."
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["discover", "send", "status", "result"],
                    "description": "A2A operation to perform"
                },
                "url": {
                    "type": "string",
                    "description": "Base URL of the remote agent (e.g. http://host:port)"
                },
                "bearer_token": {
                    "type": "string",
                    "description": "Bearer token for authentication with the remote agent"
                },
                "task_id": {
                    "type": "string",
                    "description": "Task ID (required for status/result actions)"
                },
                "message": {
                    "type": "string",
                    "description": "Message to send to the remote agent (required for send action)"
                }
            },
            "required": ["action", "url"]
        })
    }

    async fn execute(&self, args: serde_json::Value) -> anyhow::Result<ToolResult> {
        self.security.record_action();

        let action = args
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let url = args
            .get("url")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let bearer_token = args
            .get("bearer_token")
            .and_then(|v| v.as_str())
            .map(String::from);
        let task_id = args
            .get("task_id")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let message = args
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        if url.is_empty() {
            return Ok(ToolResult {
                success: false,
                output: String::new(),
                error: Some("Missing required parameter: url".into()),
            });
        }

        match action.as_str() {
            "discover" => self.action_discover(&url, bearer_token.as_deref()).await,
            "send" => {
                if message.is_empty() {
                    return Ok(ToolResult {
                        success: false,
                        output: String::new(),
                        error: Some("Missing required parameter: message".into()),
                    });
                }
                self.action_send(&url, bearer_token.as_deref(), &message)
                    .await
            }
            "status" => {
                if task_id.is_empty() {
                    return Ok(ToolResult {
                        success: false,
                        output: String::new(),
                        error: Some("Missing required parameter: task_id".into()),
                    });
                }
                self.action_status(&url, bearer_token.as_deref(), &task_id)
                    .await
            }
            "result" => {
                if task_id.is_empty() {
                    return Ok(ToolResult {
                        success: false,
                        output: String::new(),
                        error: Some("Missing required parameter: task_id".into()),
                    });
                }
                self.action_result(&url, bearer_token.as_deref(), &task_id)
                    .await
            }
            other => Ok(ToolResult {
                success: false,
                output: String::new(),
                error: Some(format!(
                    "Unknown action: '{other}'. Valid actions: discover, send, status, result"
                )),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::security::SecurityPolicy;

    fn test_tool() -> A2aTool {
        let security = Arc::new(SecurityPolicy::default());
        A2aTool::new(security, 30)
    }

    #[test]
    fn tool_metadata() {
        let tool = test_tool();
        assert_eq!(tool.name(), "a2a");
        assert!(!tool.description().is_empty());

        let schema = tool.parameters_schema();
        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["action"].is_object());
        assert!(schema["properties"]["url"].is_object());

        let required = schema["required"].as_array().unwrap();
        assert!(required.contains(&json!("action")));
        assert!(required.contains(&json!("url")));
    }

    #[test]
    fn validate_url_accepts_http() {
        assert!(A2aTool::validate_url("http://localhost:8080").is_ok());
        assert!(A2aTool::validate_url("https://agent.example.com").is_ok());
    }

    #[test]
    fn validate_url_rejects_non_http() {
        assert!(A2aTool::validate_url("ftp://host").is_err());
        assert!(A2aTool::validate_url("file:///etc/passwd").is_err());
    }

    #[tokio::test]
    async fn missing_url_returns_error() {
        let tool = test_tool();
        let result = tool.execute(json!({"action": "discover"})).await.unwrap();
        assert!(!result.success);
        assert!(result.error.as_deref().unwrap().contains("url"));
    }

    #[tokio::test]
    async fn unknown_action_returns_error() {
        let tool = test_tool();
        let result = tool
            .execute(json!({"action": "invalid", "url": "http://localhost"}))
            .await
            .unwrap();
        assert!(!result.success);
        assert!(result.error.as_deref().unwrap().contains("Unknown action"));
    }

    #[tokio::test]
    async fn send_missing_message_returns_error() {
        let tool = test_tool();
        let result = tool
            .execute(json!({"action": "send", "url": "http://localhost"}))
            .await
            .unwrap();
        assert!(!result.success);
        assert!(result.error.as_deref().unwrap().contains("message"));
    }

    #[tokio::test]
    async fn status_missing_task_id_returns_error() {
        let tool = test_tool();
        let result = tool
            .execute(json!({"action": "status", "url": "http://localhost"}))
            .await
            .unwrap();
        assert!(!result.success);
        assert!(result.error.as_deref().unwrap().contains("task_id"));
    }
}
