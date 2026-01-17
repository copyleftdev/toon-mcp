use rmcp::{
    ErrorData as McpError,
    Json,
    ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*,
    tool, tool_handler, tool_router,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use toon_format::{encode, decode, EncodeOptions, DecodeOptions, Delimiter, ToonError};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EncodeRequest {
    /// JSON to encode (object, array, or JSON string)
    pub json: serde_json::Value,

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

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
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

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DecodeRequest {
    /// TOON string to decode
    pub toon: String,

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

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ValidateRequest {
    /// TOON string to validate
    pub toon: String,

    /// Strict validation (default: true)
    #[serde(default)]
    pub strict: Option<bool>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct ValidateResponse {
    /// Whether the TOON is valid
    pub valid: bool,

    /// Error details if invalid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ValidationError>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct ValidationError {
    /// Error message
    pub message: String,

    /// Line number where error occurred
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<usize>,

    /// Column number where error occurred
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<usize>,

    /// Suggestion to fix the error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct StatsRequest {
    /// JSON to analyze
    pub json: serde_json::Value,

    /// Encoding options to apply
    #[serde(default)]
    pub encode_options: EncodeOptionsInput,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct StatsResponse {
    /// JSON format statistics
    pub json: FormatStats,

    /// TOON format statistics
    pub toon: FormatStats,

    /// Savings comparison
    pub savings: SavingsStats,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct FormatStats {
    /// Size in bytes
    pub bytes: usize,

    /// Approximate token count
    pub tokens_approx: usize,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct SavingsStats {
    /// Byte savings percentage
    pub bytes_percent: f64,

    /// Token savings percentage
    pub tokens_percent: f64,
}

#[derive(Clone)]
pub struct ToonTools {
    tool_router: ToolRouter<Self>,
}

impl ToonTools {
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
                    message: format!("Parse error at line {}, column {}: {}", line, column, message).into(),
                    data: Some(data),
                }
            }
            ToonError::LengthMismatch { expected, found, .. } => {
                McpError {
                    code: ErrorCode::INVALID_PARAMS,
                    message: format!("Array length mismatch: expected {}, found {}", expected, found).into(),
                    data: Some(serde_json::json!({
                        "expected": expected,
                        "found": found,
                    })),
                }
            }
            other => McpError {
                code: ErrorCode::INVALID_PARAMS,
                message: other.to_string().into(),
                data: None,
            }
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

    #[tool(
        name = "toon_encode",
        description = "Convert JSON to TOON format for reduced token usage. Achieves 18-40% savings."
    )]
    async fn toon_encode(
        &self,
        Parameters(request): Parameters<EncodeRequest>,
    ) -> Result<CallToolResult, McpError> {
        // Handle string input - parse JSON string to value
        let json_value: serde_json::Value = match &request.json {
            serde_json::Value::String(s) => {
                serde_json::from_str(s.as_str()).map_err(|e| McpError {
                    code: ErrorCode::INVALID_PARAMS,
                    message: format!("Invalid JSON string: {}", e).into(),
                    data: None,
                })?
            }
            other => other.clone(),
        };

        // Build options
        let mut opts = EncodeOptions::new();

        if let Some(ref delim) = request.delimiter {
            opts = opts.with_delimiter(match delim.as_str() {
                "tab" => Delimiter::Tab,
                "pipe" => Delimiter::Pipe,
                _ => Delimiter::Comma,
            });
        }

        if let Some(indent) = request.indent {
            let spaces: usize = (indent.min(8)) as usize;
            opts = opts.with_spaces(spaces);
        }

        if request.fold_keys.unwrap_or(false) {
            opts = opts.with_key_folding(toon_format::types::KeyFoldingMode::Safe);
        }

        if let Some(depth) = request.flatten_depth {
            opts = opts.with_flatten_depth(depth);
        }

        // Encode
        let result = encode(&json_value, &opts).map_err(|e| McpError {
            code: ErrorCode::INTERNAL_ERROR,
            message: format!("Encoding failed: {}", e).into(),
            data: None,
        })?;

        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(
        name = "toon_decode",
        description = "Convert TOON format back to JSON. Supports strict validation and type coercion."
    )]
    async fn toon_decode(
        &self,
        Parameters(request): Parameters<DecodeRequest>,
    ) -> Result<CallToolResult, McpError> {
        let mut opts = DecodeOptions::new();

        if let Some(strict) = request.strict {
            opts = opts.with_strict(strict);
        }

        if let Some(coerce) = request.coerce_types {
            opts = opts.with_coerce_types(coerce);
        }

        if request.expand_paths.unwrap_or(false) {
            opts = opts.with_expand_paths(toon_format::types::PathExpansionMode::Safe);
        }

        let json_value: serde_json::Value = decode(&request.toon, &opts)
            .map_err(|e| Self::map_toon_error(e))?;

        let output = match request.output_format.as_deref() {
            Some("json_pretty") => serde_json::to_string_pretty(&json_value),
            _ => serde_json::to_string(&json_value),
        }.map_err(|e| McpError {
            code: ErrorCode::INTERNAL_ERROR,
            message: format!("JSON serialization failed: {}", e).into(),
            data: None,
        })?;

        Ok(CallToolResult::success(vec![Content::text(output)]))
    }

    #[tool(
        name = "toon_validate",
        description = "Validate TOON syntax without full decoding. Returns validity and error details."
    )]
    async fn toon_validate(
        &self,
        Parameters(request): Parameters<ValidateRequest>,
    ) -> Result<Json<ValidateResponse>, McpError> {
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

    #[tool(
        name = "toon_stats",
        description = "Compare token and byte counts between JSON and TOON. Estimates cost savings."
    )]
    async fn toon_stats(
        &self,
        Parameters(request): Parameters<StatsRequest>,
    ) -> Result<Json<StatsResponse>, McpError> {
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
            message: e.to_string().into(),
            data: None,
        })?;

        // Build encode options
        let mut opts = EncodeOptions::new();
        if let Some(ref delim) = request.encode_options.delimiter {
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
            opts = opts.with_key_folding(toon_format::types::KeyFoldingMode::Safe);
        }
        if let Some(depth) = request.encode_options.flatten_depth {
            opts = opts.with_flatten_depth(depth);
        }

        // Generate TOON
        let toon_str = encode(&json_value, &opts).map_err(|e| McpError {
            code: ErrorCode::INTERNAL_ERROR,
            message: e.to_string().into(),
            data: None,
        })?;

        // Calculate stats
        let json_bytes = json_str.len();
        let toon_bytes = toon_str.len();
        let json_tokens = Self::estimate_tokens(&json_str);
        let toon_tokens = Self::estimate_tokens(&toon_str);

        let bytes_pct = if json_bytes > 0 {
            ((json_bytes as f64 - toon_bytes as f64) / json_bytes as f64) * 100.0
        } else {
            0.0
        };

        let tokens_pct = if json_tokens > 0 {
            ((json_tokens as f64 - toon_tokens as f64) / json_tokens as f64) * 100.0
        } else {
            0.0
        };

        Ok(Json(StatsResponse {
            json: FormatStats {
                bytes: json_bytes,
                tokens_approx: json_tokens,
            },
            toon: FormatStats {
                bytes: toon_bytes,
                tokens_approx: toon_tokens,
            },
            savings: SavingsStats {
                bytes_percent: (bytes_pct * 100.0).round() / 100.0,
                tokens_percent: (tokens_pct * 100.0).round() / 100.0,
            },
        }))
    }
}

#[tool_handler]
impl ServerHandler for ToonTools {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "toon-mcp".into(),
                version: env!("CARGO_PKG_VERSION").into(),
                title: None,
                website_url: None,
                icons: None,
            },
            instructions: Some("TOON format encoding/decoding server for LLM cost optimization".into()),
        }
    }
}
