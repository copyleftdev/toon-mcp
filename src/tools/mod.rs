//! MCP tool implementations for TOON operations.
//!
//! These tools wrap the core business logic with MCP-specific
//! error handling and response formatting.

use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*,
    tool, tool_handler, tool_router, ErrorData as McpError, Json, ServerHandler,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::core::{
    self, DecodeRequest, EncodeOptionsInput, StatsRequest, ToonCoreError, ValidateRequest,
    ValidateResponse,
};

/// MCP-specific encode request (re-exported for schema generation).
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

impl EncodeRequest {
    fn to_options(&self) -> EncodeOptionsInput {
        EncodeOptionsInput {
            delimiter: self.delimiter.clone(),
            indent: self.indent,
            fold_keys: self.fold_keys,
            flatten_depth: self.flatten_depth,
        }
    }
}

/// Stats response types (re-exported for MCP schema).
pub use crate::core::{FormatStats, SavingsStats, StatsResponse, ValidationError};

#[derive(Clone)]
pub struct ToonTools {
    tool_router: ToolRouter<Self>,
}

impl ToonTools {
    fn map_core_error(e: ToonCoreError) -> McpError {
        match e {
            ToonCoreError::ParseError {
                message,
                line,
                column,
                suggestion,
            } => {
                let mut data = serde_json::json!({
                    "line": line,
                    "column": column,
                });
                if let Some(s) = suggestion {
                    data["suggestion"] = serde_json::json!(s);
                }
                McpError {
                    code: ErrorCode::INVALID_PARAMS,
                    message: format!("Parse error at line {}, column {}: {}", line, column, message)
                        .into(),
                    data: Some(data),
                }
            }
            ToonCoreError::LengthMismatch { expected, found } => McpError {
                code: ErrorCode::INVALID_PARAMS,
                message: format!(
                    "Array length mismatch: expected {}, found {}",
                    expected, found
                )
                .into(),
                data: Some(serde_json::json!({
                    "expected": expected,
                    "found": found,
                })),
            },
            ToonCoreError::InvalidJson(msg) => McpError {
                code: ErrorCode::INVALID_PARAMS,
                message: format!("Invalid JSON: {}", msg).into(),
                data: None,
            },
            other => McpError {
                code: ErrorCode::INTERNAL_ERROR,
                message: other.to_string().into(),
                data: None,
            },
        }
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
        // Parse JSON input (handles string-wrapped JSON)
        let json_value = core::parse_json_input(&request.json).map_err(Self::map_core_error)?;

        // Encode to TOON
        let options = request.to_options();
        let result = core::encode_json(&json_value, &options).map_err(Self::map_core_error)?;

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
        // Decode TOON to JSON value
        let json_value = core::decode_toon(&request.toon, &request).map_err(Self::map_core_error)?;

        // Format output
        let output = core::format_json_output(&json_value, request.output_format.as_deref())
            .map_err(Self::map_core_error)?;

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
        let result = core::validate_toon(&request.toon, request.strict);
        Ok(Json(result))
    }

    #[tool(
        name = "toon_stats",
        description = "Compare token and byte counts between JSON and TOON. Estimates cost savings."
    )]
    async fn toon_stats(
        &self,
        Parameters(request): Parameters<StatsRequest>,
    ) -> Result<Json<StatsResponse>, McpError> {
        // Parse JSON input
        let json_value = core::parse_json_input(&request.json).map_err(Self::map_core_error)?;

        // Compute stats
        let stats =
            core::compute_stats(&json_value, &request.encode_options).map_err(Self::map_core_error)?;

        Ok(Json(stats))
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
            instructions: Some(
                "TOON format encoding/decoding server for LLM cost optimization".into(),
            ),
        }
    }
}
