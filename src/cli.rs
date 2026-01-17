//! Command-line interface for toon-mcp server.

use clap::{Parser, ValueEnum};

/// Server mode selection.
#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum ServerMode {
    /// MCP mode using stdio transport (default, for Claude Desktop)
    #[default]
    Mcp,
    /// HTTP REST API mode
    Http,
}

/// TOON MCP Server - Token-efficient JSON encoding for LLM prompts.
///
/// Provides TOON format encoding/decoding via MCP protocol (stdio) or HTTP REST API.
#[derive(Parser, Debug)]
#[command(name = "toon-mcp")]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Server mode: mcp (stdio, default) or http (REST API)
    #[arg(short, long, value_enum, default_value_t = ServerMode::Mcp, env = "TOON_MODE")]
    pub mode: ServerMode,

    /// HTTP server host address
    #[arg(long, default_value = "0.0.0.0", env = "TOON_HOST")]
    pub host: String,

    /// HTTP server port
    #[arg(short, long, default_value_t = 8080, env = "TOON_PORT")]
    pub port: u16,

    /// Enable verbose logging
    #[arg(short, long, default_value_t = false, env = "TOON_VERBOSE")]
    pub verbose: bool,
}

impl Args {
    /// Parse command line arguments.
    pub fn parse_args() -> Self {
        Args::parse()
    }

    /// Get the socket address for HTTP mode.
    pub fn socket_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
