# Task 007: Testing & Fixtures

**Status**: pending  
**Depends On**: 003-encode, 004-decode, 005-validate, 006-stats  

---

## Objective

Implement test suite with round-trip tests and fixtures.

---

## Acceptance Criteria

- [ ] Round-trip tests pass (10+ cases)
- [ ] Fixture tests pass
- [ ] `cargo test` succeeds with no failures

---

## Files to Create

### tests/round_trip.rs

```rust
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
fn empty() {
    round_trip(json!({}));
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
```

### tests/fixtures/valid/simple.json

```json
{"name": "Alice", "age": 30}
```

### tests/fixtures/valid/simple.toon

```
name: Alice
age: 30
```

### tests/fixtures/valid/tabular.json

```json
{"users": [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]}
```

### tests/fixtures/valid/tabular.toon

```
users[2]{id,name}:
  1,Alice
  2,Bob
```

---

## Verification

```bash
cargo test
cargo test round_trip
cargo test -- --nocapture
```

---

## Next Task

After PASS, proceed to `008-docs`.
