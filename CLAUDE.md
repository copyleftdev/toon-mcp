# TOON MCP Server

A Rust MCP server exposing TOON format encoding/decoding for LLM cost optimization.

## Quick Reference

### Build Commands
```bash
cargo check          # Type check
cargo build          # Debug build
cargo build --release # Release build
cargo test           # Run all tests
cargo test round_trip # Run specific test
cargo clippy         # Lint
cargo fmt            # Format
```

### Run Server
```bash
./target/debug/toon-mcp      # Debug
./target/release/toon-mcp    # Release
```

### Test MCP Protocol
```bash
# Initialize
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}' | ./target/debug/toon-mcp

# List tools
echo '{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}' | ./target/debug/toon-mcp
```

---

## Project Structure

```
toon-mcp/
├── Cargo.toml
├── CLAUDE.md              # This file
├── src/
│   ├── main.rs            # Entry point
│   ├── lib.rs             # Library root
│   ├── server.rs          # MCP server setup
│   ├── error.rs           # Error types
│   └── tools/
│       ├── mod.rs         # Tool registry + ToonTools struct
│       ├── encode.rs      # toon_encode
│       ├── decode.rs      # toon_decode
│       ├── validate.rs    # toon_validate
│       └── stats.rs       # toon_stats
├── tests/
│   ├── fixtures/          # TOON/JSON test pairs
│   ├── round_trip.rs      # Encode↔decode tests
│   └── integration/       # MCP protocol tests
└── README.md
```

---

## Key Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `toon-format` | 0.4.x | Core encode/decode |
| `rmcp` | 0.13.x | Official Rust MCP SDK |
| `tokio` | 1.x | Async runtime |
| `serde`/`serde_json` | 1.x | Serialization |
| `schemars` | 0.8.x | JSON Schema for tool params |

---

## Code Style

- **Imports**: Group std, external crates, then local modules
- **Errors**: Use `thiserror` for error types, map to `McpError` at boundaries
- **Async**: All tool handlers are async (required by rmcp)
- **Testing**: Every tool needs round-trip and error-path tests

---

## Tool Implementation Pattern

Each tool follows this structure:

```rust
use rmcp::{tool, Parameters, CallToolResult, ErrorData as McpError};
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct MyToolRequest {
    pub required_field: String,
    #[serde(default)]
    pub optional_field: Option<bool>,
}

#[tool(
    name = "my_tool",
    description = "What this tool does"
)]
async fn my_tool(
    &self,
    params: Parameters<MyToolRequest>,
) -> Result<CallToolResult, McpError> {
    let request = params.0;
    // Implementation
    Ok(CallToolResult::success(vec![Content::text("result")]))
}
```

---

## Reference Implementation

The `toon-format` reference is at `../reference/toon-rust/`.

Key files:
- `src/lib.rs` — Public API exports
- `src/encode/mod.rs` — Encode implementation
- `src/decode/mod.rs` — Decode implementation
- `src/types/options.rs` — EncodeOptions, DecodeOptions
- `src/types/errors.rs` — ToonError variants

---

## Workflow

1. **Read the ticket** in `.claude/tasks/` for current work
2. **Implement** following the spec exactly
3. **Test** with `cargo test` and manual MCP calls
4. **Verify** acceptance criteria are met
5. **Commit** with descriptive message

---

## Important Notes

- **DO NOT** modify files in `../reference/` — that's read-only reference
- **DO** run `cargo check` frequently during implementation
- **DO** run `cargo test` before considering a task complete
- **ALWAYS** preserve existing tests when adding new code
