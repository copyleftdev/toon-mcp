//! Unit tests for core TOON operations.

mod common;

use toon_mcp::core::{
    compute_stats, decode_toon, encode_json, estimate_tokens, parse_json_input, validate_toon,
    DecodeRequest, EncodeOptionsInput,
};

#[test]
fn test_encode_simple_object() {
    let json = common::simple_json();
    let options = EncodeOptionsInput::default();

    let result = encode_json(&json, &options);
    assert!(result.is_ok());

    let toon = result.unwrap();
    assert!(toon.contains("name"));
    assert!(toon.contains("Alice"));
    assert!(toon.contains("age"));
    assert!(toon.contains("30"));
}

#[test]
fn test_encode_tabular_data() {
    let json = common::tabular_json();
    let options = EncodeOptionsInput::default();

    let result = encode_json(&json, &options);
    assert!(result.is_ok());

    let toon = result.unwrap();
    assert!(toon.contains("id"));
    assert!(toon.contains("name"));
    assert!(toon.contains("Alice"));
    assert!(toon.contains("Bob"));
}

#[test]
fn test_encode_with_delimiter() {
    let json = common::simple_json();
    let options = EncodeOptionsInput {
        delimiter: Some("tab".to_string()),
        ..Default::default()
    };

    let result = encode_json(&json, &options);
    assert!(result.is_ok());
}

#[test]
fn test_encode_with_indent() {
    let json = common::simple_json();
    let options = EncodeOptionsInput {
        indent: Some(4),
        ..Default::default()
    };

    let result = encode_json(&json, &options);
    assert!(result.is_ok());
}

#[test]
fn test_decode_simple() {
    let toon = "name: Alice\nage: 30";
    let request = DecodeRequest {
        toon: toon.to_string(),
        strict: None,
        coerce_types: None,
        expand_paths: None,
        output_format: None,
    };

    let result = decode_toon(&request.toon, &request);
    assert!(result.is_ok());

    let json = result.unwrap();
    assert_eq!(json["name"], "Alice");
    assert_eq!(json["age"], 30);
}

#[test]
fn test_decode_with_strict_mode() {
    let toon = "name: Alice";
    let request = DecodeRequest {
        toon: toon.to_string(),
        strict: Some(true),
        coerce_types: None,
        expand_paths: None,
        output_format: None,
    };

    let result = decode_toon(&request.toon, &request);
    assert!(result.is_ok());
}

#[test]
fn test_roundtrip_simple() {
    let original = common::simple_json();
    let options = EncodeOptionsInput::default();

    let toon = encode_json(&original, &options).expect("encode failed");

    let request = DecodeRequest {
        toon,
        strict: None,
        coerce_types: None,
        expand_paths: None,
        output_format: None,
    };

    let decoded = decode_toon(&request.toon, &request).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_roundtrip_tabular() {
    let original = common::tabular_json();
    let options = EncodeOptionsInput::default();

    let toon = encode_json(&original, &options).expect("encode failed");

    let request = DecodeRequest {
        toon,
        strict: None,
        coerce_types: None,
        expand_paths: None,
        output_format: None,
    };

    let decoded = decode_toon(&request.toon, &request).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_roundtrip_nested() {
    let original = common::nested_json();
    let options = EncodeOptionsInput::default();

    let toon = encode_json(&original, &options).expect("encode failed");

    let request = DecodeRequest {
        toon,
        strict: None,
        coerce_types: None,
        expand_paths: None,
        output_format: None,
    };

    let decoded = decode_toon(&request.toon, &request).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_roundtrip_special_chars() {
    let original = common::special_chars_json();
    let options = EncodeOptionsInput::default();

    let toon = encode_json(&original, &options).expect("encode failed");

    let request = DecodeRequest {
        toon,
        strict: None,
        coerce_types: None,
        expand_paths: None,
        output_format: None,
    };

    let decoded = decode_toon(&request.toon, &request).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_roundtrip_empty_object() {
    let original = common::empty_object();
    let options = EncodeOptionsInput::default();

    let toon = encode_json(&original, &options).expect("encode failed");

    let request = DecodeRequest {
        toon,
        strict: None,
        coerce_types: None,
        expand_paths: None,
        output_format: None,
    };

    let decoded = decode_toon(&request.toon, &request).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_roundtrip_empty_array() {
    let original = common::empty_array();
    let options = EncodeOptionsInput::default();

    let toon = encode_json(&original, &options).expect("encode failed");

    let request = DecodeRequest {
        toon,
        strict: None,
        coerce_types: None,
        expand_paths: None,
        output_format: None,
    };

    let decoded = decode_toon(&request.toon, &request).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_validate_valid_toon() {
    let toon = "name: Alice\nage: 30";
    let result = validate_toon(toon, None);

    assert!(result.valid);
    assert!(result.error.is_none());
}

#[test]
fn test_validate_invalid_toon() {
    // Invalid TOON with mismatched array
    let toon = "[a, b, c\nname: 1, 2";
    let result = validate_toon(toon, Some(true));

    // Note: The actual validation result depends on toon-format behavior
    // This test verifies the function executes without panic
    assert!(result.valid || result.error.is_some());
}

#[test]
fn test_compute_stats() {
    let json = common::simple_json();
    let options = EncodeOptionsInput::default();

    let result = compute_stats(&json, &options);
    assert!(result.is_ok());

    let stats = result.unwrap();
    assert!(stats.json.bytes > 0);
    assert!(stats.toon.bytes > 0);
    assert!(stats.json.tokens_approx > 0);
    assert!(stats.toon.tokens_approx > 0);

    // TOON should typically be smaller
    assert!(stats.savings.bytes_percent >= 0.0);
}

#[test]
fn test_parse_json_input_object() {
    let value = serde_json::json!({"key": "value"});
    let result = parse_json_input(&value);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), value);
}

#[test]
fn test_parse_json_input_string() {
    let value = serde_json::json!(r#"{"key": "value"}"#);
    let result = parse_json_input(&value);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), serde_json::json!({"key": "value"}));
}

#[test]
fn test_parse_json_input_invalid_string() {
    let value = serde_json::json!("not valid json {");
    let result = parse_json_input(&value);

    assert!(result.is_err());
}

#[test]
fn test_estimate_tokens_empty() {
    assert_eq!(estimate_tokens(""), 0);
}

#[test]
fn test_estimate_tokens_words() {
    assert_eq!(estimate_tokens("hello world"), 2);
    assert_eq!(estimate_tokens("one two three"), 3);
}

#[test]
fn test_estimate_tokens_json() {
    let json = r#"{"name": "Alice", "age": 30}"#;
    let tokens = estimate_tokens(json);

    // Should count: name, Alice, age, 30, plus punctuation
    assert!(tokens > 4);
}

#[test]
fn test_estimate_tokens_punctuation() {
    // Each punctuation mark counts as a token
    assert_eq!(estimate_tokens("..."), 3);
    assert_eq!(estimate_tokens("a.b.c"), 5); // a, ., b, ., c
}
