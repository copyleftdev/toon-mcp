//! TOON MCP Server - Token-efficient JSON encoding for LLM prompts.

mod cli;
mod core;
mod error;
mod server;

#[cfg(feature = "mcp")]
mod tools;

use cli::{Args, ServerMode};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse_args();

    match args.mode {
        ServerMode::Mcp => {
            #[cfg(feature = "mcp")]
            {
                server::run_mcp_server().await
            }
            #[cfg(not(feature = "mcp"))]
            {
                anyhow::bail!("MCP mode not available. Build with --features mcp")
            }
        }
        ServerMode::Http => {
            #[cfg(feature = "http")]
            {
                let addr = args.socket_addr();
                server::run_http_server(&addr).await
            }
            #[cfg(not(feature = "http"))]
            {
                anyhow::bail!("HTTP mode not available. Build with --features http")
            }
        }
    }
}
