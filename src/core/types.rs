//! Shared types for TOON operations across HTTP and MCP transports.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use toon_format::ToonError;

/// Error type for core TOON operations.
#[derive(Error, Debug)]
pub enum ToonCoreError {
    #[error("Parse error at line {line}, column {column}: {message}")]
    ParseError {
        message: String,
        line: usize,
        column: usize,
        suggestion: Option<String>,
    },

    #[error("Array length mismatch: expected {expected}, found {found}")]
    LengthMismatch { expected: usize, found: usize },

    #[error("Encoding failed: {0}")]
    EncodeError(String),

    #[error("Decoding failed: {0}")]
    DecodeError(String),

    #[error("Invalid JSON: {0}")]
    InvalidJson(String),

    #[error("Serialization failed: {0}")]
    SerializationError(String),
}

impl From<ToonError> for ToonCoreError {
    fn from(e: ToonError) -> Self {
        match e {
            ToonError::ParseError {
                line,
                column,
                message,
                context,
            } => ToonCoreError::ParseError {
                message,
                line,
                column,
                suggestion: context.and_then(|c| c.suggestion),
            },
            ToonError::LengthMismatch {
                expected, found, ..
            } => ToonCoreError::LengthMismatch { expected, found },
            other => ToonCoreError::DecodeError(other.to_string()),
        }
    }
}

impl From<serde_json::Error> for ToonCoreError {
    fn from(e: serde_json::Error) -> Self {
        ToonCoreError::InvalidJson(e.to_string())
    }
}

/// Request to encode JSON to TOON format.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "http", derive(utoipa::ToSchema))]
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

/// Encoding options input for stats and other operations.
#[derive(Debug, Default, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "http", derive(utoipa::ToSchema))]
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

/// Request to decode TOON to JSON format.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "http", derive(utoipa::ToSchema))]
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

/// Request to validate TOON syntax.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "http", derive(utoipa::ToSchema))]
pub struct ValidateRequest {
    /// TOON string to validate
    pub toon: String,

    /// Strict validation (default: true)
    #[serde(default)]
    pub strict: Option<bool>,
}

/// Response from validation.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "http", derive(utoipa::ToSchema))]
pub struct ValidateResponse {
    /// Whether the TOON is valid
    pub valid: bool,

    /// Error details if invalid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ValidationError>,
}

/// Validation error details.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "http", derive(utoipa::ToSchema))]
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

impl From<ToonCoreError> for ValidationError {
    fn from(e: ToonCoreError) -> Self {
        match e {
            ToonCoreError::ParseError {
                message,
                line,
                column,
                suggestion,
            } => ValidationError {
                message,
                line: Some(line),
                column: Some(column),
                suggestion,
            },
            ToonCoreError::LengthMismatch { expected, found } => ValidationError {
                message: format!("Array length mismatch: expected {}, found {}", expected, found),
                line: None,
                column: None,
                suggestion: Some(format!("Expected {} items but found {}", expected, found)),
            },
            other => ValidationError {
                message: other.to_string(),
                line: None,
                column: None,
                suggestion: None,
            },
        }
    }
}

/// Request to compute statistics.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "http", derive(utoipa::ToSchema))]
pub struct StatsRequest {
    /// JSON to analyze
    pub json: serde_json::Value,

    /// Encoding options to apply
    #[serde(default)]
    pub encode_options: EncodeOptionsInput,
}

/// Response with format statistics.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "http", derive(utoipa::ToSchema))]
pub struct StatsResponse {
    /// JSON format statistics
    pub json: FormatStats,

    /// TOON format statistics
    pub toon: FormatStats,

    /// Savings comparison
    pub savings: SavingsStats,
}

/// Statistics for a single format.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "http", derive(utoipa::ToSchema))]
pub struct FormatStats {
    /// Size in bytes
    pub bytes: usize,

    /// Approximate token count
    pub tokens_approx: usize,
}

/// Savings comparison between formats.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "http", derive(utoipa::ToSchema))]
pub struct SavingsStats {
    /// Byte savings percentage
    pub bytes_percent: f64,

    /// Token savings percentage
    pub tokens_percent: f64,
}

/// Simple encode response for HTTP API.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "http", derive(utoipa::ToSchema))]
pub struct EncodeResponse {
    /// The encoded TOON string
    pub toon: String,
}

/// Simple decode response for HTTP API.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "http", derive(utoipa::ToSchema))]
pub struct DecodeResponse {
    /// The decoded JSON value
    pub json: serde_json::Value,
}

/// Health check response.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "http", derive(utoipa::ToSchema))]
pub struct HealthResponse {
    /// Service status
    pub status: String,

    /// Service version
    pub version: String,
}
