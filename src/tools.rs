use anyhow::{Result, anyhow};
use colored::*;
use dialoguer::Confirm;
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ToolExecutor {
    yolo_mode: bool,
    safe_tools: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolCall {
    pub name: String,
    pub arguments: HashMap<String, serde_json::Value>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolResult {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
}

impl ToolExecutor {
    pub fn new(yolo_mode: bool) -> Self {
        Self {
            yolo_mode,
            safe_tools: vec![
                "list_files".to_string(),
                "read_file".to_string(),
                "get_current_directory".to_string(),
                "git_status".to_string(),
                "git_log".to_string(),
                "which".to_string(),
                "echo".to_string(),
            ],
        }
    }

    pub async fn execute_tool(&self, tool_call: &ToolCall) -> Result<ToolResult> {
        let is_safe = self.is_safe_tool(&tool_call.name);

        if !self.yolo_mode && !is_safe {
            let description = tool_call.description.as_deref()
                .unwrap_or("No description provided");

            println!("\n{}", "Tool Execution Request:".bright_yellow().bold());
            println!("  Tool: {}", tool_call.name.bright_white());
            println!("  Description: {}", description.dimmed());
            println!("  Arguments: {}", serde_json::to_string_pretty(&tool_call.arguments)?);

            let should_execute = Confirm::new()
                .with_prompt("Do you want to execute this tool?")
                .default(false)
                .interact()?;

            if !should_execute {
                return Ok(ToolResult {
                    success: false,
                    output: "Tool execution denied by user".to_string(),
                    error: None,
                });
            }
        }

        match tool_call.name.as_str() {
            "shell" | "bash" | "cmd" => self.execute_shell_command(tool_call).await,
            "list_files" | "ls" => self.list_files(tool_call).await,
            "read_file" | "cat" => self.read_file(tool_call).await,
            "write_file" => self.write_file(tool_call).await,
            "get_current_directory" | "pwd" => self.get_current_directory().await,
            "git_status" => self.git_status().await,
            "git_log" => self.git_log(tool_call).await,
            "which" => self.which_command(tool_call).await,
            "echo" => self.echo(tool_call).await,
            _ => Err(anyhow!("Unknown tool: {}", tool_call.name)),
        }
    }

    fn is_safe_tool(&self, tool_name: &str) -> bool {
        self.safe_tools.contains(&tool_name.to_string())
    }

    async fn execute_shell_command(&self, tool_call: &ToolCall) -> Result<ToolResult> {
        let command = tool_call.arguments.get("command")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing 'command' argument"))?;

        println!("{} {}", "Executing:".bright_blue().bold(), command.bright_white());

        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", command])
                .output()?
        } else {
            Command::new("sh")
                .args(["-c", command])
                .output()?
        };

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if output.status.success() {
            Ok(ToolResult {
                success: true,
                output: stdout,
                error: if stderr.is_empty() { None } else { Some(stderr) },
            })
        } else {
            Ok(ToolResult {
                success: false,
                output: stdout,
                error: Some(stderr),
            })
        }
    }

    async fn list_files(&self, tool_call: &ToolCall) -> Result<ToolResult> {
        let path = tool_call.arguments.get("path")
            .and_then(|v| v.as_str())
            .unwrap_or(".");

        let output = if cfg!(target_os = "windows") {
            Command::new("dir")
                .arg(path)
                .output()?
        } else {
            Command::new("ls")
                .args(["-la", path])
                .output()?
        };

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();

        Ok(ToolResult {
            success: output.status.success(),
            output: stdout,
            error: if output.status.success() { None } else {
                Some(String::from_utf8_lossy(&output.stderr).to_string())
            },
        })
    }

    async fn read_file(&self, tool_call: &ToolCall) -> Result<ToolResult> {
        let file_path = tool_call.arguments.get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing 'path' argument"))?;

        match tokio::fs::read_to_string(file_path).await {
            Ok(content) => Ok(ToolResult {
                success: true,
                output: content,
                error: None,
            }),
            Err(e) => Ok(ToolResult {
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
            }),
        }
    }

    async fn write_file(&self, tool_call: &ToolCall) -> Result<ToolResult> {
        let file_path = tool_call.arguments.get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing 'path' argument"))?;

        let content = tool_call.arguments.get("content")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing 'content' argument"))?;

        match tokio::fs::write(file_path, content).await {
            Ok(_) => Ok(ToolResult {
                success: true,
                output: format!("Successfully wrote to {}", file_path),
                error: None,
            }),
            Err(e) => Ok(ToolResult {
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
            }),
        }
    }

    async fn get_current_directory(&self) -> Result<ToolResult> {
        match std::env::current_dir() {
            Ok(path) => Ok(ToolResult {
                success: true,
                output: path.display().to_string(),
                error: None,
            }),
            Err(e) => Ok(ToolResult {
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
            }),
        }
    }

    async fn git_status(&self) -> Result<ToolResult> {
        let output = Command::new("git")
            .args(["status", "--porcelain"])
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();

        Ok(ToolResult {
            success: output.status.success(),
            output: stdout,
            error: if output.status.success() { None } else {
                Some(String::from_utf8_lossy(&output.stderr).to_string())
            },
        })
    }

    async fn git_log(&self, tool_call: &ToolCall) -> Result<ToolResult> {
        let limit = tool_call.arguments.get("limit")
            .and_then(|v| v.as_u64())
            .unwrap_or(10);

        let output = Command::new("git")
            .args(["log", "--oneline", &format!("-{}", limit)])
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();

        Ok(ToolResult {
            success: output.status.success(),
            output: stdout,
            error: if output.status.success() { None } else {
                Some(String::from_utf8_lossy(&output.stderr).to_string())
            },
        })
    }

    async fn which_command(&self, tool_call: &ToolCall) -> Result<ToolResult> {
        let command = tool_call.arguments.get("command")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing 'command' argument"))?;

        let output = if cfg!(target_os = "windows") {
            Command::new("where")
                .arg(command)
                .output()?
        } else {
            Command::new("which")
                .arg(command)
                .output()?
        };

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();

        Ok(ToolResult {
            success: output.status.success(),
            output: stdout,
            error: if output.status.success() { None } else {
                Some(String::from_utf8_lossy(&output.stderr).to_string())
            },
        })
    }

    async fn echo(&self, tool_call: &ToolCall) -> Result<ToolResult> {
        let message = tool_call.arguments.get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        Ok(ToolResult {
            success: true,
            output: message.to_string(),
            error: None,
        })
    }
}
