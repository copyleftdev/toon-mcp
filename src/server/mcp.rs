//! MCP server implementation using stdio transport.

use crate::tools::ToonTools;
use rmcp::{transport::stdio, ServiceExt};

/// Run the MCP server with stdio transport.
pub async fn run_mcp_server() -> anyhow::Result<()> {
    eprintln!("toon-mcp server starting in MCP mode...");
    let service = ToonTools::new().serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
