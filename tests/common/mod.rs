//! Shared test utilities.

use serde_json::Value;

/// Create a simple test JSON object.
pub fn simple_json() -> Value {
    serde_json::json!({
        "name": "Alice",
        "age": 30
    })
}

/// Create a tabular test JSON (array of objects).
pub fn tabular_json() -> Value {
    serde_json::json!([
        {"id": 1, "name": "Alice", "active": true},
        {"id": 2, "name": "Bob", "active": false},
        {"id": 3, "name": "Charlie", "active": true}
    ])
}

/// Create a nested test JSON.
pub fn nested_json() -> Value {
    serde_json::json!({
        "user": {
            "profile": {
                "name": "Alice",
                "settings": {
                    "theme": "dark",
                    "notifications": true
                }
            }
        }
    })
}

/// Create a JSON with special characters.
pub fn special_chars_json() -> Value {
    serde_json::json!({
        "message": "Hello \"World\"!",
        "path": "/home/user/file.txt",
        "unicode": "日本語 emoji: 🎉"
    })
}

/// Create an empty object.
pub fn empty_object() -> Value {
    serde_json::json!({})
}

/// Create an empty array.
pub fn empty_array() -> Value {
    serde_json::json!([])
}
