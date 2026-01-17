//! Server implementations for toon-mcp.
//!
//! This module provides both MCP (stdio) and HTTP transports for TOON operations.

#[cfg(feature = "mcp")]
pub mod mcp;

#[cfg(feature = "http")]
pub mod http;

#[cfg(feature = "mcp")]
pub use mcp::run_mcp_server;

#[cfg(feature = "http")]
pub use http::run_http_server;
