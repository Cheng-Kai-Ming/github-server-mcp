mod github;

use anyhow::Result;
use rmcp::{ServiceExt, transport::stdio};
use github::GitHubService;
use tracing_subscriber::{self, EnvFilter};

/// MCP GitHub CLI Service Server
/// Communicates with client through standard input/output streams
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting MCP GitHub server...");

    // Create GitHub service instance
    let service = GitHubService::new().serve(stdio()).await?;

    // Wait for service to stop
    tracing::info!("Service started, waiting for requests...");
    service.waiting().await?;
    
    tracing::info!("Service stopped");
    Ok(())
}
