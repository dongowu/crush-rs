use anyhow::Result;
use clap::Parser;
use tracing::info;

mod config;
mod session;
mod providers;
mod lsp;
mod mcp;

/// Crush - AI coding assistant for your terminal
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,

    /// Skip permission prompts (use with caution!)
    #[arg(short, long)]
    yolo: bool,

    /// Specify a session name
    #[arg(short, long)]
    session: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(if args.debug {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        })
        .init();

    info!("Starting Crush-RS");
    info!("Debug mode: {}", args.debug);
    info!("YOLO mode: {}", args.yolo);

    // Load configuration
    let config = config::load_config().await?;
    info!("Configuration loaded");

    // Initialize session
    let session_name = args.session.unwrap_or_else(|| "default".to_string());
    let mut session = session::Session::new(&session_name, config, args.yolo).await?;
    info!("Session '{}' initialized", session_name);

    // Start REPL
    session.run().await?;

    info!("Crush-RS exiting");
    Ok(())
}
// ```

// This sets up the basic structure of our application with:
// 1. Command-line argument parsing
// 2. Logging initialization
// 3. Configuration loading
// 4. Session management
// 5. REPL interface

// Next, let's create the configuration module.
