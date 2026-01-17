# Task 002: MCP Server Bootstrap

**Status**: pending  
**Depends On**: 001-scaffold  

---

## Objective

Implement MCP server using `rmcp` with stdio transport and a placeholder tool.

---

## Acceptance Criteria

- [ ] Server starts and listens on stdio
- [ ] Server responds to MCP `initialize` request
- [ ] Server lists `toon_ping` tool
- [ ] Calling `toon_ping` returns "pong"

---

## Reference

rmcp docs: https://docs.rs/rmcp/latest/rmcp/

Key patterns:
- `#[tool_router]` macro on impl block
- `#[tool]` macro on async methods
- `ServerHandler` trait for server info
- `serve_server()` with transport

---

## Files to Modify

### src/server.rs

```rust
use rmcp::{
    transport::io::stdio::StdioTransport,
    service::serve_server,
};
use crate::tools::ToonTools;

pub async fn run_server() -> anyhow::Result<()> {
    let tools = ToonTools::new();
    let transport = StdioTransport::default();
    
    let server = serve_server(tools, transport).await?;
    server.waiting().await?;
    
    Ok(())
}
```

### src/tools/mod.rs

```rust
use rmcp::{
    ErrorData as McpError,
    ServerHandler,
    handler::server::tool::ToolRouter,
    model::*,
    tool, tool_router,
};

#[derive(Clone)]
pub struct ToonTools {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl ToonTools {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Ping the TOON MCP server to verify connectivity")]
    async fn toon_ping(&self) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![Content::text(
            "pong - toon-mcp server is running",
        )]))
    }
}

impl ServerHandler for ToonTools {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            name: "toon-mcp".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            ..Default::default()
        }
    }
}
```

---

## Verification

```bash
cargo build

# Test initialize
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}' | ./target/debug/toon-mcp

# Test tools/list
echo '{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}' | ./target/debug/toon-mcp

# Test ping
echo '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"toon_ping","arguments":{}}}' | ./target/debug/toon-mcp
```

---

## Notes

If rmcp API differs from this spec, check:
1. https://github.com/modelcontextprotocol/rust-sdk/tree/main/examples
2. Adjust imports/traits as needed

---

## Next Task

After PASS, proceed to `003-encode`.
