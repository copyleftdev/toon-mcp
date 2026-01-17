# Task 005: toon_validate Tool

**Status**: pending  
**Depends On**: 004-decode  

---

## Objective

Implement `toon_validate` tool for syntax checking without full output.

---

## Acceptance Criteria

- [ ] Tool accepts TOON string
- [ ] Tool supports strict option
- [ ] Returns `{"valid": true}` on success
- [ ] Returns `{"valid": false, "error": {...}}` on failure
- [ ] Error includes line, column, suggestion when available

---

## Implementation

### Add to src/tools/mod.rs

```rust
use rmcp::Json;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ValidateRequest {
    pub toon: String,
    #[serde(default)]
    pub strict: Option<bool>,
}

#[derive(Serialize, JsonSchema)]
pub struct ValidateResponse {
    pub valid: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ValidationError>,
}

#[derive(Serialize, JsonSchema)]
pub struct ValidationError {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<String>,
}

// Add to ToonTools impl block:

#[tool(
    name = "toon_validate",
    description = "Validate TOON syntax without full decoding. Returns validity and error details."
)]
async fn toon_validate(
    &self,
    params: Parameters<ValidateRequest>,
) -> Result<Json<ValidateResponse>, McpError> {
    let request = params.0;

    let mut opts = DecodeOptions::new();
    if let Some(strict) = request.strict {
        opts = opts.with_strict(strict);
    }

    let result: Result<serde_json::Value, _> = decode(&request.toon, &opts);

    match result {
        Ok(_) => Ok(Json(ValidateResponse {
            valid: true,
            error: None,
        })),
        Err(e) => Ok(Json(ValidateResponse {
            valid: false,
            error: Some(Self::to_validation_error(e)),
        })),
    }
}

fn to_validation_error(e: ToonError) -> ValidationError {
    match e {
        ToonError::ParseError { line, column, message, context } => {
            ValidationError {
                message,
                line: Some(line),
                column: Some(column),
                suggestion: context.and_then(|c| c.suggestion),
            }
        }
        ToonError::LengthMismatch { expected, found, .. } => {
            ValidationError {
                message: format!("Array length mismatch: expected {}, found {}", expected, found),
                line: None,
                column: None,
                suggestion: Some(format!("Expected {} items but found {}", expected, found)),
            }
        }
        other => ValidationError {
            message: other.to_string(),
            line: None,
            column: None,
            suggestion: None,
        }
    }
}
```

---

## Verification

```bash
# Valid
echo '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"toon_validate","arguments":{"toon":"x: 1"}}}' | ./target/debug/toon-mcp

# Invalid
echo '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"toon_validate","arguments":{"toon":"items[3]: a,b"}}}' | ./target/debug/toon-mcp
```

---

## Next Task

After PASS, proceed to `006-stats`.
