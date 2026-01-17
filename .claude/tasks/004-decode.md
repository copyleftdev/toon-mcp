# Task 004: toon_decode Tool

**Status**: pending  
**Depends On**: 002-bootstrap  

---

## Objective

Implement `toon_decode` tool that converts TOON back to JSON.

---

## Acceptance Criteria

- [ ] Tool accepts TOON string
- [ ] Tool supports strict mode option
- [ ] Tool supports coerce_types option
- [ ] Tool supports expand_paths option
- [ ] Tool supports output_format (json/json_pretty)
- [ ] Returns JSON on success
- [ ] Returns error with line/column on failure

---

## Reference

toon-format API:
```rust
use toon_format::{decode, DecodeOptions, PathExpansionMode};

let opts = DecodeOptions::new()
    .with_strict(true)
    .with_coerce_types(true)
    .with_expand_paths(PathExpansionMode::Safe);

let json: serde_json::Value = decode(&toon_str, &opts)?;
```

---

## Implementation

### Add to src/tools/mod.rs

```rust
use toon_format::{decode, DecodeOptions, PathExpansionMode, ToonError};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DecodeRequest {
    /// TOON string to decode
    pub toon: String,

    #[serde(default)]
    pub options: DecodeOptionsInput,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct DecodeOptionsInput {
    /// Strict validation (default: true)
    #[serde(default)]
    pub strict: Option<bool>,

    /// Type coercion (default: true)
    #[serde(default)]
    pub coerce_types: Option<bool>,

    /// Path expansion (default: false)
    #[serde(default)]
    pub expand_paths: Option<bool>,

    /// Output: "json" or "json_pretty" (default: "json")
    #[serde(default)]
    pub output_format: Option<String>,
}

// Add to ToonTools impl block:

#[tool(
    name = "toon_decode",
    description = "Convert TOON format back to JSON. Supports strict validation and type coercion."
)]
async fn toon_decode(
    &self,
    params: Parameters<DecodeRequest>,
) -> Result<CallToolResult, McpError> {
    let request = params.0;

    let mut opts = DecodeOptions::new();

    if let Some(strict) = request.options.strict {
        opts = opts.with_strict(strict);
    }

    if let Some(coerce) = request.options.coerce_types {
        opts = opts.with_coerce_types(coerce);
    }

    if request.options.expand_paths.unwrap_or(false) {
        opts = opts.with_expand_paths(PathExpansionMode::Safe);
    }

    let json_value: serde_json::Value = decode(&request.toon, &opts)
        .map_err(|e| Self::map_toon_error(e))?;

    let output = match request.options.output_format.as_deref() {
        Some("json_pretty") => serde_json::to_string_pretty(&json_value),
        _ => serde_json::to_string(&json_value),
    }.map_err(|e| McpError {
        code: ErrorCode::INTERNAL_ERROR,
        message: format!("JSON serialization failed: {}", e),
        data: None,
    })?;

    Ok(CallToolResult::success(vec![Content::text(output)]))
}

fn map_toon_error(e: ToonError) -> McpError {
    match e {
        ToonError::ParseError { line, column, message, context } => {
            let mut data = serde_json::json!({
                "line": line,
                "column": column,
            });
            if let Some(ctx) = context {
                if let Some(s) = ctx.suggestion {
                    data["suggestion"] = serde_json::json!(s);
                }
            }
            McpError {
                code: ErrorCode::INVALID_PARAMS,
                message: format!("Parse error at line {}, column {}: {}", line, column, message),
                data: Some(data),
            }
        }
        ToonError::LengthMismatch { expected, found, .. } => {
            McpError {
                code: ErrorCode::INVALID_PARAMS,
                message: format!("Array length mismatch: expected {}, found {}", expected, found),
                data: Some(serde_json::json!({
                    "expected": expected,
                    "found": found,
                })),
            }
        }
        other => McpError {
            code: ErrorCode::INVALID_PARAMS,
            message: other.to_string(),
            data: None,
        }
    }
}
```

---

## Verification

```bash
cargo test

echo '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"toon_decode","arguments":{"toon":"name: Alice\nage: 30"}}}' | ./target/debug/toon-mcp
```

---

## Next Task

After PASS, proceed to `005-validate`.
