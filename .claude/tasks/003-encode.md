# Task 003: toon_encode Tool

**Status**: pending  
**Depends On**: 002-bootstrap  

---

## Objective

Implement `toon_encode` tool that converts JSON to TOON format.

---

## Acceptance Criteria

- [ ] Tool accepts JSON as object, array, or string
- [ ] Tool supports delimiter option (comma/tab/pipe)
- [ ] Tool supports indent option (0-8 spaces)
- [ ] Tool supports fold_keys option
- [ ] Tool supports flatten_depth option
- [ ] Returns TOON string on success
- [ ] Returns error with context on failure

---

## Reference

toon-format API (from `../reference/toon-rust/src/lib.rs`):
```rust
use toon_format::{encode, EncodeOptions, Delimiter, KeyFoldingMode};

let opts = EncodeOptions::new()
    .with_delimiter(Delimiter::Comma)
    .with_spaces(2)
    .with_key_folding(KeyFoldingMode::Safe)
    .with_flatten_depth(3);

let toon = encode(&json_value, &opts)?;
```

---

## Implementation

### Add to src/tools/mod.rs

```rust
use schemars::JsonSchema;
use serde::Deserialize;
use toon_format::{encode, EncodeOptions, Delimiter, KeyFoldingMode};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct EncodeRequest {
    /// JSON to encode (object, array, or JSON string)
    pub json: serde_json::Value,

    #[serde(default)]
    pub options: EncodeOptionsInput,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct EncodeOptionsInput {
    /// Delimiter: "comma" (default), "tab", or "pipe"
    #[serde(default)]
    pub delimiter: Option<String>,

    /// Spaces for indentation (0-8, default: 2)
    #[serde(default)]
    pub indent: Option<u8>,

    /// Enable v1.5 key folding
    #[serde(default)]
    pub fold_keys: Option<bool>,

    /// Max depth for key folding
    #[serde(default)]
    pub flatten_depth: Option<usize>,
}

// Add to ToonTools impl block:

#[tool(
    name = "toon_encode",
    description = "Convert JSON to TOON format for reduced token usage. Achieves 18-40% savings."
)]
async fn toon_encode(
    &self,
    params: Parameters<EncodeRequest>,
) -> Result<CallToolResult, McpError> {
    let request = params.0;

    // Handle string input
    let json_value = match &request.json {
        serde_json::Value::String(s) => {
            serde_json::from_str(s).map_err(|e| McpError {
                code: ErrorCode::INVALID_PARAMS,
                message: format!("Invalid JSON string: {}", e),
                data: None,
            })?
        }
        other => other.clone(),
    };

    // Build options
    let mut opts = EncodeOptions::new();

    if let Some(delim) = &request.options.delimiter {
        opts = opts.with_delimiter(match delim.as_str() {
            "tab" => Delimiter::Tab,
            "pipe" => Delimiter::Pipe,
            _ => Delimiter::Comma,
        });
    }

    if let Some(indent) = request.options.indent {
        opts = opts.with_spaces(indent.min(8) as usize);
    }

    if request.options.fold_keys.unwrap_or(false) {
        opts = opts.with_key_folding(KeyFoldingMode::Safe);
    }

    if let Some(depth) = request.options.flatten_depth {
        opts = opts.with_flatten_depth(depth);
    }

    // Encode
    let result = encode(&json_value, &opts).map_err(|e| McpError {
        code: ErrorCode::INTERNAL_ERROR,
        message: format!("Encoding failed: {}", e),
        data: None,
    })?;

    Ok(CallToolResult::success(vec![Content::text(result)]))
}
```

---

## Test Cases

### Basic object
Input: `{"json": {"name": "Alice", "age": 30}}`
Output contains: `name: Alice` and `age: 30`

### Tabular array
Input: `{"json": {"users": [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]}}`
Output contains: `users[2]{id,name}:`

### With key folding
Input: `{"json": {"a": {"b": {"c": 1}}}, "options": {"fold_keys": true}}`
Output: `a.b.c: 1`

---

## Verification

```bash
cargo test

echo '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"toon_encode","arguments":{"json":{"users":[{"id":1,"name":"Alice"},{"id":2,"name":"Bob"}]}}}}' | ./target/debug/toon-mcp
```

---

## Next Task

After PASS, proceed to `004-decode`.
