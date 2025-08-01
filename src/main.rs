use clap::{Parser, Subcommand};
use anyhow::Result;
use colored::*;

mod cli;
mod config;
mod llm;
mod session;
mod tools;

use cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.run().await {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    }
    
    Ok(())
}