# Task 006: toon_stats Tool

**Status**: pending  
**Depends On**: 003-encode  

---

## Objective

Implement `toon_stats` tool to compare JSON vs TOON token/byte counts.

---

## Acceptance Criteria

- [ ] Tool accepts JSON input
- [ ] Tool applies encode options
- [ ] Returns byte counts for both formats
- [ ] Returns approximate token counts
- [ ] Returns savings percentages

---

## Implementation

### Add to src/tools/mod.rs

```rust
#[derive(Debug, Deserialize, JsonSchema)]
pub struct StatsRequest {
    pub json: serde_json::Value,
    #[serde(default)]
    pub encode_options: EncodeOptionsInput,
}

#[derive(Serialize, JsonSchema)]
pub struct StatsResponse {
    pub json: FormatStats,
    pub toon: FormatStats,
    pub savings: SavingsStats,
}

#[derive(Serialize, JsonSchema)]
pub struct FormatStats {
    pub bytes: usize,
    pub tokens_approx: usize,
}

#[derive(Serialize, JsonSchema)]
pub struct SavingsStats {
    pub bytes_percent: f64,
    pub tokens_percent: f64,
}

// Add to ToonTools impl block:

#[tool(
    name = "toon_stats",
    description = "Compare token and byte counts between JSON and TOON. Estimates cost savings."
)]
async fn toon_stats(
    &self,
    params: Parameters<StatsRequest>,
) -> Result<Json<StatsResponse>, McpError> {
    let request = params.0;

    // Handle string input
    let json_value = match &request.json {
        serde_json::Value::String(s) => {
            serde_json::from_str(s).unwrap_or(request.json.clone())
        }
        other => other.clone(),
    };

    // Generate JSON string
    let json_str = serde_json::to_string(&json_value).map_err(|e| McpError {
        code: ErrorCode::INTERNAL_ERROR,
        message: e.to_string(),
        data: None,
    })?;

    // Build encode options
    let mut opts = EncodeOptions::new();
    if let Some(delim) = &request.encode_options.delimiter {
        opts = opts.with_delimiter(match delim.as_str() {
            "tab" => Delimiter::Tab,
            "pipe" => Delimiter::Pipe,
            _ => Delimiter::Comma,
        });
    }
    if let Some(indent) = request.encode_options.indent {
        opts = opts.with_spaces(indent.min(8) as usize);
    }
    if request.encode_options.fold_keys.unwrap_or(false) {
        opts = opts.with_key_folding(KeyFoldingMode::Safe);
    }
    if let Some(depth) = request.encode_options.flatten_depth {
        opts = opts.with_flatten_depth(depth);
    }

    // Generate TOON
    let toon_str = encode(&json_value, &opts).map_err(|e| McpError {
        code: ErrorCode::INTERNAL_ERROR,
        message: e.to_string(),
        data: None,
    })?;

    // Calculate stats
    let json_bytes = json_str.len();
    let toon_bytes = toon_str.len();
    let json_tokens = Self::estimate_tokens(&json_str);
    let toon_tokens = Self::estimate_tokens(&toon_str);

    let bytes_pct = if json_bytes > 0 {
        ((json_bytes as f64 - toon_bytes as f64) / json_bytes as f64) * 100.0
    } else { 0.0 };

    let tokens_pct = if json_tokens > 0 {
        ((json_tokens as f64 - toon_tokens as f64) / json_tokens as f64) * 100.0
    } else { 0.0 };

    Ok(Json(StatsResponse {
        json: FormatStats { bytes: json_bytes, tokens_approx: json_tokens },
        toon: FormatStats { bytes: toon_bytes, tokens_approx: toon_tokens },
        savings: SavingsStats {
            bytes_percent: (bytes_pct * 100.0).round() / 100.0,
            tokens_percent: (tokens_pct * 100.0).round() / 100.0,
        },
    }))
}

fn estimate_tokens(text: &str) -> usize {
    let mut count = 0;
    let mut in_word = false;
    for c in text.chars() {
        if c.is_alphanumeric() || c == '_' {
            if !in_word {
                count += 1;
                in_word = true;
            }
        } else {
            in_word = false;
            if !c.is_whitespace() {
                count += 1;
            }
        }
    }
    count
}
```

---

## Verification

```bash
echo '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"toon_stats","arguments":{"json":{"users":[{"id":1,"name":"Alice"},{"id":2,"name":"Bob"}]}}}}' | ./target/debug/toon-mcp
```

---

## Next Task

After PASS, proceed to `007-testing`.
