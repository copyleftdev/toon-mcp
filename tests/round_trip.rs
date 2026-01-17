use serde_json::{json, Value};
use toon_format::{encode_default, decode_default};

fn round_trip(input: Value) {
    let encoded = encode_default(&input).expect("encode failed");
    let decoded: Value = decode_default(&encoded).expect("decode failed");
    assert_eq!(input, decoded);
}

#[test]
fn simple_object() {
    round_trip(json!({"name": "Alice", "age": 30}));
}

#[test]
fn array() {
    round_trip(json!({"tags": ["a", "b", "c"]}));
}

#[test]
fn tabular() {
    round_trip(json!({
        "users": [
            {"id": 1, "name": "Alice"},
            {"id": 2, "name": "Bob"}
        ]
    }));
}

#[test]
fn nested() {
    round_trip(json!({"a": {"b": {"c": 1}}}));
}

#[test]
fn empty_object() {
    round_trip(json!({}));
}

#[test]
fn empty_array() {
    round_trip(json!([]));
}

#[test]
fn special_chars() {
    round_trip(json!({"msg": "Hello, \"World\"!"}));
}

#[test]
fn numbers() {
    round_trip(json!({"int": 42, "float": 3.14}));
}

#[test]
fn booleans_null() {
    round_trip(json!({"a": true, "b": false, "c": null}));
}

#[test]
fn mixed_array() {
    round_trip(json!({"items": [1, "two", true]}));
}

#[test]
fn deep_nesting() {
    round_trip(json!({"a": {"b": {"c": {"d": {"e": 1}}}}}));
}

#[test]
fn unicode_strings() {
    round_trip(json!({"greeting": "こんにちは", "emoji": "🎉"}));
}

#[test]
fn negative_numbers() {
    round_trip(json!({"temp": -40, "balance": -123.45}));
}
