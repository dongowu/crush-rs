use anyhow::Result;
use colored::*;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use dialoguer::{Input, Confirm};
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::path::PathBuf;
use tokio::fs;
use uuid::Uuid;

use crate::{llm::{LlmProvider, ChatMessage, ChatResponse}, tools::ToolExecutor};

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub name: String,
    pub messages: Vec<ChatMessage>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    
    #[serde(skip)]
    provider: Option<LlmProvider>,
    
    #[serde(skip)]
    tool_executor: Option<ToolExecutor>,
    
    #[serde(skip)]
    yolo_mode: bool,
}

impl Session {
    pub async fn new_or_load(
        session_name: Option<String>,
        provider: LlmProvider,
        yolo_mode: bool,
    ) -> Result<Self> {
        let session_name = session_name.unwrap_or_else(|| "default".to_string());
        let session_path = Self::session_path(&session_name);
        
        let mut session = if session_path.exists() {
            let content = fs::read_to_string(&session_path).await?;
            let mut session: Session = serde_json::from_str(&content)?;
            session.provider = Some(provider);
            session.tool_executor = Some(ToolExecutor::new(yolo_mode));
            session.yolo_mode = yolo_mode;
            session
        } else {
            let mut session = Self {
                id: Uuid::new_v4().to_string(),
                name: session_name,
                messages: vec![
                    ChatMessage::system(
                        "You are Crush, a helpful AI coding assistant. You can help with coding tasks, \
                        explain code, suggest improvements, and run tools when needed. Always be concise \
                        and helpful. When you need to run tools or execute commands, ask for permission \
                        unless the user has enabled yolo mode."
                    )
                ],
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                provider: Some(provider),
                tool_executor: Some(ToolExecutor::new(yolo_mode)),
                yolo_mode,
            };
            session.save().await?;
            session
        };
        
        Ok(session)
    }
    
    pub async fn send_message(&mut self, message: &str) -> Result<()> {
        let user_message = ChatMessage::user(message);
        self.messages.push(user_message);
        
        println!("\n{} {}", "You:".bright_cyan().bold(), message);
        
        match self.provider.as_ref().unwrap().send_message(&self.messages).await {
            Ok(response) => {
                let assistant_message = ChatMessage::assistant(&response.content);
                self.messages.push(assistant_message);
                
                println!("\n{} {}", "Crush:".bright_magenta().bold(), response.content);
                
                if let Some(usage) = response.usage {
                    println!("{}", format!(
                        "({} tokens used)", 
                        usage.total_tokens
                    ).dimmed());
                }
                
                self.updated_at = chrono::Utc::now();
                self.save().await?;
            }
            Err(e) => {
                // Remove the user message since we got an error
                self.messages.pop();
                
                println!("\n{} {}", "❌ Error:".red().bold(), e.to_string().bright_red());
                
                // Provide helpful suggestions based on error type
                let error_str = e.to_string();
                if error_str.contains("Cloudflare") {
                    println!("{}", "\n💡 Suggestions:".bright_yellow().bold());
                    println!("   • Check if your base_url is correct");
                    println!("   • Some providers may be blocked by Cloudflare");
                    println!("   • Try using a different provider or VPN");
                } else if error_str.contains("Authentication") || error_str.contains("unauthorized") {
                    println!("{}", "\n💡 Suggestions:".bright_yellow().bold());
                    println!("   • Verify your API key is correct and active");
                    println!("   • Check if your API key has sufficient permissions");
                } else if error_str.contains("Rate limit") {
                    println!("{}", "\n💡 Suggestions:".bright_yellow().bold());
                    println!("   • Wait a moment and try again");
                    println!("   • Consider upgrading your API plan");
                } else if error_str.contains("endpoint not found") {
                    println!("{}", "\n💡 Suggestions:".bright_yellow().bold());
                    println!("   • Check your base_url configuration");
                    println!("   • Verify the API endpoint is correct");
                }
                
                return Err(e);
            }
        }
        
        Ok(())
    }
    
    pub async fn start_interactive_loop(&mut self) -> Result<()> {
        println!("\n{}", "Type your message and press Enter. Type 'exit' to quit.".dimmed());
        println!("{}", "Use Ctrl+C to interrupt at any time.".dimmed());
        
        loop {
            print!("\n{} ", "➤".bright_green().bold());
            io::stdout().flush()?;
            
            let input: String = Input::new()
                .allow_empty(false)
                .interact_text()?;
            
            match input.trim() {
                "exit" | "quit" | ":q" => {
                    println!("{}", "Goodbye! 👋".bright_magenta());
                    break;
                }
                "clear" | ":clear" => {
                    print!("\x1B[2J\x1B[1;1H");
                    continue;
                }
                "help" | ":help" => {
                    self.show_help();
                    continue;
                }
                "status" | ":status" => {
                    self.show_status();
                    continue;
                }
                _ => {
                    match self.send_message(&input).await {
                        Ok(_) => {},
                        Err(_) => {
                            // Error is already displayed in send_message, just continue the loop
                            continue;
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn show_help(&self) {
        println!("\n{}", "Crush Commands:".bright_cyan().bold());
        println!("  {}  Exit the session", "exit, quit, :q".bright_white());
        println!("  {}  Clear the screen", "clear, :clear".bright_white());
        println!("  {}  Show this help", "help, :help".bright_white());
        println!("  {}  Show session status", "status, :status".bright_white());
    }
    
    fn show_status(&self) {
        println!("\n{}", "Session Status:".bright_cyan().bold());
        println!("  Name: {}", self.name.bright_white());
        println!("  ID: {}", self.id.dimmed());
        println!("  Messages: {}", self.messages.len().to_string().bright_white());
        println!("  Provider: {}", 
            self.provider.as_ref()
                .map(|p| p.name.as_str())
                .unwrap_or("None")
                .bright_white()
        );
        println!("  YOLO Mode: {}", 
            if self.yolo_mode { "ON".red().bold() } else { "OFF".green() }
        );
        println!("  Created: {}", self.created_at.format("%Y-%m-%d %H:%M:%S UTC").to_string().dimmed());
        println!("  Updated: {}", self.updated_at.format("%Y-%m-%d %H:%M:%S UTC").to_string().dimmed());
    }
    
    async fn save(&self) -> Result<()> {
        let session_path = Self::session_path(&self.name);
        
        if let Some(parent) = session_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&session_path, content).await?;
        
        Ok(())
    }
    
    pub async fn list_all() -> Result<Vec<String>> {
        let sessions_dir = Self::sessions_dir();
        
        if !sessions_dir.exists() {
            return Ok(vec![]);
        }
        
        let mut sessions = vec![];
        let mut entries = fs::read_dir(&sessions_dir).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".json") {
                    sessions.push(name.trim_end_matches(".json").to_string());
                }
            }
        }
        
        sessions.sort();
        Ok(sessions)
    }
    
    fn session_path(name: &str) -> PathBuf {
        Self::sessions_dir().join(format!("{}.json", name))
    }
    
    fn sessions_dir() -> PathBuf {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("crush")
            .join("sessions")
    }
}