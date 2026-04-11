//! # spawn_agent Tool
//! Allows the LLM to define and execute ephemeral agent personas at runtime.

use super::traits::{Tool, ToolResult};
use crate::config::Config;
use crate::security::SecurityPolicy;
use crate::security::policy::ToolOperation;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

/// Definition of a single ephemeral agent to spawn.
#[derive(Debug, Deserialize)]
struct EphemeralAgentDef {
    /// Human-readable name for the agent.
    name: String,
    /// System prompt that defines the agent's persona.
    system_prompt: String,
    /// Optional model override (e.g. "anthropic/claude-sonnet-4-6").
    #[serde(default)]
    model: Option<String>,
    /// Tools this agent is allowed to use. Must be a subset of parent_tools.
    #[serde(default)]
    tools: Vec<String>,
    /// Input message for the agent to process.
    input: String,
}

/// Top-level arguments parsed from the tool call.
#[derive(Debug, Deserialize)]
struct SpawnArgs {
    agents: Vec<EphemeralAgentDef>,
    #[serde(default = "default_parallel")]
    parallel: bool,
}

fn default_parallel() -> bool {
    true
}

/// Tool that lets the LLM define and execute ephemeral agent personas at
/// runtime for parallel execution.
pub struct SpawnAgentTool {
    security: Arc<SecurityPolicy>,
    config: Config,
    parent_tools: Vec<String>,
    max_depth: usize,
    max_concurrent: usize,
    current_depth: usize,
}

impl SpawnAgentTool {
    pub fn new(
        security: Arc<SecurityPolicy>,
        config: Config,
        parent_tools: Vec<String>,
        max_depth: usize,
        max_concurrent: usize,
        current_depth: usize,
    ) -> Self {
        Self {
            security,
            config,
            parent_tools,
            max_depth,
            max_concurrent,
            current_depth,
        }
    }

    /// Validate spawn arguments against depth, concurrency, and tool-subset constraints.
    fn validate(&self, args: &SpawnArgs) -> Result<(), ToolResult> {
        if self.current_depth >= self.max_depth {
            return Err(ToolResult {
                success: false,
                output: String::new(),
                error: Some(format!(
                    "Maximum spawn depth exceeded (current: {}, max: {})",
                    self.current_depth, self.max_depth
                )),
            });
        }

        if args.agents.len() > self.max_concurrent {
            return Err(ToolResult {
                success: false,
                output: String::new(),
                error: Some(format!(
                    "Too many agents requested ({}, max: {})",
                    args.agents.len(),
                    self.max_concurrent
                )),
            });
        }

        for agent_def in &args.agents {
            for tool_name in &agent_def.tools {
                if !self.parent_tools.contains(tool_name) {
                    return Err(ToolResult {
                        success: false,
                        output: String::new(),
                        error: Some(format!(
                            "Tool '{}' is not available in parent scope for agent '{}'",
                            tool_name, agent_def.name
                        )),
                    });
                }
            }
        }

        Ok(())
    }
}

#[async_trait]
impl Tool for SpawnAgentTool {
    fn name(&self) -> &str {
        "spawn_agent"
    }

    fn description(&self) -> &str {
        "Spawn one or more ephemeral agent personas for parallel or sequential execution. \
         Each agent receives a system prompt, optional model override, a tool subset, \
         and an input message. Results are collected and returned as JSON."
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "agents": {
                    "type": "array",
                    "description": "Array of ephemeral agent definitions to spawn.",
                    "items": {
                        "type": "object",
                        "properties": {
                            "name": {
                                "type": "string",
                                "description": "Human-readable name for the agent."
                            },
                            "system_prompt": {
                                "type": "string",
                                "description": "System prompt defining the agent's persona and behavior."
                            },
                            "model": {
                                "type": "string",
                                "description": "Optional model override (e.g. 'anthropic/claude-sonnet-4-6')."
                            },
                            "tools": {
                                "type": "array",
                                "items": { "type": "string" },
                                "description": "Tools this agent is allowed to use (must be subset of parent tools)."
                            },
                            "input": {
                                "type": "string",
                                "description": "Input message for the agent to process."
                            }
                        },
                        "required": ["name", "system_prompt", "input"]
                    }
                },
                "parallel": {
                    "type": "boolean",
                    "description": "Whether to run agents in parallel (default: true)."
                }
            },
            "required": ["agents"]
        })
    }

    async fn execute(&self, args: serde_json::Value) -> anyhow::Result<ToolResult> {
        // Security gate
        if let Err(error) = self
            .security
            .enforce_tool_operation(ToolOperation::Act, "spawn_agent")
        {
            return Ok(ToolResult {
                success: false,
                output: String::new(),
                error: Some(error),
            });
        }

        let spawn_args: SpawnArgs = serde_json::from_value(args)
            .map_err(|e| anyhow::anyhow!("Invalid spawn_agent arguments: {e}"))?;

        if let Err(result) = self.validate(&spawn_args) {
            return Ok(result);
        }

        // Build and execute ephemeral agents
        let mut results = Vec::new();
        let _config = &self.config;

        if spawn_args.parallel {
            let mut handles = Vec::new();
            for agent_def in &spawn_args.agents {
                let name = agent_def.name.clone();
                let _system_prompt = agent_def.system_prompt.clone();
                let _model = agent_def.model.clone();
                let _input = agent_def.input.clone();

                // TODO: Build Agent::from_config with customised system_prompt,
                // model, and tool subset, then call agent.turn(&input).
                // For now, return a placeholder indicating the agent was validated
                // but full execution requires integration wiring.
                let handle = tokio::spawn(async move {
                    json!({
                        "agent": name,
                        "status": "validated",
                        "note": "Full agent execution requires integration wiring"
                    })
                });
                handles.push(handle);
            }

            for handle in handles {
                match handle.await {
                    Ok(result) => results.push(result),
                    Err(e) => results.push(json!({
                        "error": format!("Agent task failed: {e}")
                    })),
                }
            }
        } else {
            for agent_def in &spawn_args.agents {
                // Sequential execution
                results.push(json!({
                    "agent": agent_def.name,
                    "status": "validated",
                    "note": "Full agent execution requires integration wiring"
                }));
            }
        }

        Ok(ToolResult {
            success: true,
            output: serde_json::to_string_pretty(&json!({
                "spawned": results.len(),
                "parallel": spawn_args.parallel,
                "results": results
            }))?,
            error: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_tool() -> SpawnAgentTool {
        SpawnAgentTool::new(
            Arc::new(SecurityPolicy::default()),
            Config::default(),
            vec!["shell".to_string(), "memory".to_string()],
            3,
            10,
            0,
        )
    }

    #[tokio::test]
    async fn rejects_depth_exceeded() {
        let tool = SpawnAgentTool::new(
            Arc::new(SecurityPolicy::default()),
            Config::default(),
            vec!["shell".to_string()],
            3,
            10,
            3, // at max depth
        );
        let result = tool
            .execute(json!({
                "agents": [{
                    "name": "test",
                    "system_prompt": "You are a test agent.",
                    "input": "hello"
                }]
            }))
            .await
            .unwrap();
        assert!(!result.success);
        assert!(result.error.as_deref().unwrap_or("").contains("depth"));
    }

    #[tokio::test]
    async fn rejects_too_many_agents() {
        let tool = SpawnAgentTool::new(
            Arc::new(SecurityPolicy::default()),
            Config::default(),
            vec!["shell".to_string()],
            3,
            2, // max 2 concurrent
            0,
        );
        let result = tool
            .execute(json!({
                "agents": [
                    { "name": "a1", "system_prompt": "p1", "input": "i1" },
                    { "name": "a2", "system_prompt": "p2", "input": "i2" },
                    { "name": "a3", "system_prompt": "p3", "input": "i3" }
                ]
            }))
            .await
            .unwrap();
        assert!(!result.success);
        assert!(result.error.as_deref().unwrap_or("").contains("Too many"));
    }

    #[tokio::test]
    async fn rejects_unauthorized_tool() {
        let tool = test_tool();
        let result = tool
            .execute(json!({
                "agents": [{
                    "name": "test",
                    "system_prompt": "You are a test agent.",
                    "tools": ["shell", "dangerous_tool"],
                    "input": "hello"
                }]
            }))
            .await
            .unwrap();
        assert!(!result.success);
        assert!(
            result
                .error
                .as_deref()
                .unwrap_or("")
                .contains("not available")
        );
    }

    #[test]
    fn tool_metadata() {
        let tool = test_tool();
        assert_eq!(tool.name(), "spawn_agent");
        assert!(!tool.description().is_empty());
        let schema = tool.parameters_schema();
        assert!(schema.get("properties").unwrap().get("agents").is_some());
    }
}
