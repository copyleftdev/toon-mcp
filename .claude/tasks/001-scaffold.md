# Task 001: Project Scaffold

**Status**: pending  
**Depends On**: none  

---

## Objective

Create the Rust project structure for toon-mcp.

---

## Acceptance Criteria

- [ ] `Cargo.toml` exists with all dependencies
- [ ] Directory structure matches spec
- [ ] `cargo check` passes
- [ ] `cargo build` produces binary

---

## Files to Create

### Cargo.toml

```toml
[package]
name = "toon-mcp"
version = "0.1.0"
edition = "2021"
description = "MCP server for TOON format - token-efficient JSON alternative for LLM prompts"
license = "MIT"

[[bin]]
name = "toon-mcp"
path = "src/main.rs"

[features]
default = []
tiktoken = ["dep:tiktoken-rs"]

[dependencies]
toon-format = { version = "0.4", default-features = false }
rmcp = { version = "0.13", features = ["server", "transport-io", "macros"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
schemars = "0.8"
thiserror = "2.0"
anyhow = "1.0"

tiktoken-rs = { version = "0.6", optional = true }

[dev-dependencies]
```

### src/main.rs

```rust
mod error;
mod server;
mod tools;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    eprintln!("toon-mcp server starting...");
    server::run_server().await
}
```

### src/lib.rs

```rust
pub mod error;
pub mod server;
pub mod tools;
```

### src/error.rs

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ToonMcpError {
    #[error("Encoding error: {0}")]
    Encode(String),

    #[error("Decoding error: {0}")]
    Decode(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}
```

### src/server.rs

```rust
pub async fn run_server() -> anyhow::Result<()> {
    // Placeholder - will be implemented in task 002
    Ok(())
}
```

### src/tools/mod.rs

```rust
// Tool implementations will be added in subsequent tasks
```

### tests/integration/mod.rs

```rust
// Integration tests will be added in task 007
```

---

## Verification

```bash
cargo check
cargo build
./target/debug/toon-mcp
# Should print "toon-mcp server starting..." and exit
```

---

## Next Task

After PASS, proceed to `002-bootstrap`.
