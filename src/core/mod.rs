//! Core business logic for TOON encoding/decoding operations.
//!
//! This module contains pure functions that are shared between
//! the MCP and HTTP transport layers.

pub mod types;

pub use types::*;

use toon_format::{decode, encode, DecodeOptions, Delimiter, EncodeOptions};

/// Encode JSON value to TOON format.
pub fn encode_json(
    json: &serde_json::Value,
    options: &EncodeOptionsInput,
) -> Result<String, ToonCoreError> {
    let opts = build_encode_options(options);
    encode(json, &opts).map_err(|e| ToonCoreError::EncodeError(e.to_string()))
}

/// Decode TOON string to JSON value.
pub fn decode_toon(toon: &str, request: &DecodeRequest) -> Result<serde_json::Value, ToonCoreError> {
    let opts = build_decode_options(request);
    decode(toon, &opts).map_err(ToonCoreError::from)
}

/// Validate TOON syntax without returning the decoded value.
pub fn validate_toon(toon: &str, strict: Option<bool>) -> ValidateResponse {
    let mut opts = DecodeOptions::new();
    if let Some(s) = strict {
        opts = opts.with_strict(s);
    }

    match decode::<serde_json::Value>(toon, &opts) {
        Ok(_) => ValidateResponse {
            valid: true,
            error: None,
        },
        Err(e) => {
            let core_error: ToonCoreError = e.into();
            ValidateResponse {
                valid: false,
                error: Some(core_error.into()),
            }
        }
    }
}

/// Compute statistics comparing JSON and TOON formats.
pub fn compute_stats(
    json: &serde_json::Value,
    options: &EncodeOptionsInput,
) -> Result<StatsResponse, ToonCoreError> {
    // Generate JSON string
    let json_str =
        serde_json::to_string(json).map_err(|e| ToonCoreError::SerializationError(e.to_string()))?;

    // Generate TOON string
    let toon_str = encode_json(json, options)?;

    // Calculate stats
    let json_bytes = json_str.len();
    let toon_bytes = toon_str.len();
    let json_tokens = estimate_tokens(&json_str);
    let toon_tokens = estimate_tokens(&toon_str);

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

    Ok(StatsResponse {
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
    })
}

/// Parse JSON value from request, handling both direct values and JSON strings.
pub fn parse_json_input(value: &serde_json::Value) -> Result<serde_json::Value, ToonCoreError> {
    match value {
        serde_json::Value::String(s) => {
            serde_json::from_str(s).map_err(|e| ToonCoreError::InvalidJson(e.to_string()))
        }
        other => Ok(other.clone()),
    }
}

/// Estimate token count for a string.
/// Simple approximation: count alphanumeric words plus non-whitespace punctuation.
pub fn estimate_tokens(text: &str) -> usize {
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

/// Build EncodeOptions from EncodeOptionsInput.
pub fn build_encode_options(input: &EncodeOptionsInput) -> EncodeOptions {
    let mut opts = EncodeOptions::new();

    if let Some(ref delim) = input.delimiter {
        opts = opts.with_delimiter(match delim.as_str() {
            "tab" => Delimiter::Tab,
            "pipe" => Delimiter::Pipe,
            _ => Delimiter::Comma,
        });
    }

    if let Some(indent) = input.indent {
        let spaces: usize = (indent.min(8)) as usize;
        opts = opts.with_spaces(spaces);
    }

    if input.fold_keys.unwrap_or(false) {
        opts = opts.with_key_folding(toon_format::types::KeyFoldingMode::Safe);
    }

    if let Some(depth) = input.flatten_depth {
        opts = opts.with_flatten_depth(depth);
    }

    opts
}

/// Build DecodeOptions from DecodeRequest.
pub fn build_decode_options(request: &DecodeRequest) -> DecodeOptions {
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

    opts
}

/// Format decoded JSON according to output format preference.
pub fn format_json_output(
    value: &serde_json::Value,
    output_format: Option<&str>,
) -> Result<String, ToonCoreError> {
    match output_format {
        Some("json_pretty") => serde_json::to_string_pretty(value),
        _ => serde_json::to_string(value),
    }
    .map_err(|e| ToonCoreError::SerializationError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate_tokens_simple() {
        assert_eq!(estimate_tokens("hello world"), 2);
        assert_eq!(estimate_tokens("a b c"), 3);
    }

    #[test]
    fn test_estimate_tokens_json() {
        // {"name": "Alice"}
        let json = r#"{"name": "Alice"}"#;
        let tokens = estimate_tokens(json);
        assert!(tokens > 0);
    }

    #[test]
    fn test_parse_json_input_object() {
        let value = serde_json::json!({"key": "value"});
        let result = parse_json_input(&value).unwrap();
        assert_eq!(result, value);
    }

    #[test]
    fn test_parse_json_input_string() {
        let value = serde_json::json!(r#"{"key": "value"}"#);
        let result = parse_json_input(&value).unwrap();
        assert_eq!(result, serde_json::json!({"key": "value"}));
    }

    #[test]
    fn test_validate_toon_valid() {
        let toon = "name: Alice\nage: 30";
        let result = validate_toon(toon, None);
        assert!(result.valid);
        assert!(result.error.is_none());
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let json = serde_json::json!({"name": "Alice", "age": 30});
        let options = EncodeOptionsInput::default();

        let toon = encode_json(&json, &options).unwrap();

        let decode_req = DecodeRequest {
            toon,
            strict: None,
            coerce_types: None,
            expand_paths: None,
            output_format: None,
        };

        let decoded = decode_toon(&decode_req.toon, &decode_req).unwrap();
        assert_eq!(json, decoded);
    }
}
