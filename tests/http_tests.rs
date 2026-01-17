//! Integration tests for HTTP endpoints.
//!
//! These tests require the `http` feature to be enabled.

#![cfg(feature = "http")]

mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::util::ServiceExt;

use toon_mcp::server::http::build_router;

#[tokio::test]
async fn test_health_endpoint() {
    let app = build_router();

    let response = app
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["status"], "ok");
    assert!(json["version"].is_string());
}

#[tokio::test]
async fn test_encode_endpoint_simple() {
    let app = build_router();

    let body = serde_json::json!({
        "json": {"name": "Alice", "age": 30}
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/encode")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert!(json["toon"].is_string());
    let toon = json["toon"].as_str().unwrap();
    assert!(toon.contains("name"));
    assert!(toon.contains("Alice"));
}

#[tokio::test]
async fn test_encode_endpoint_with_options() {
    let app = build_router();

    let body = serde_json::json!({
        "json": {"name": "Alice"},
        "delimiter": "tab",
        "indent": 4
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/encode")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_encode_endpoint_string_json() {
    let app = build_router();

    let body = serde_json::json!({
        "json": r#"{"name": "Alice"}"#
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/encode")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_decode_endpoint_simple() {
    let app = build_router();

    let body = serde_json::json!({
        "toon": "name: Alice\nage: 30"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/decode")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["json"]["name"], "Alice");
    assert_eq!(json["json"]["age"], 30);
}

#[tokio::test]
async fn test_decode_endpoint_with_options() {
    let app = build_router();

    let body = serde_json::json!({
        "toon": "name: Alice",
        "strict": true,
        "coerce_types": true
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/decode")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_validate_endpoint_valid() {
    let app = build_router();

    let body = serde_json::json!({
        "toon": "name: Alice\nage: 30"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/validate")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["valid"], true);
    assert!(json["error"].is_null());
}

#[tokio::test]
async fn test_validate_endpoint_with_strict() {
    let app = build_router();

    let body = serde_json::json!({
        "toon": "name: Alice",
        "strict": true
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/validate")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_stats_endpoint() {
    let app = build_router();

    let body = serde_json::json!({
        "json": {"name": "Alice", "age": 30, "email": "alice@example.com"},
        "encode_options": {}
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/stats")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    // Verify structure
    assert!(json["json"]["bytes"].is_number());
    assert!(json["json"]["tokens_approx"].is_number());
    assert!(json["toon"]["bytes"].is_number());
    assert!(json["toon"]["tokens_approx"].is_number());
    assert!(json["savings"]["bytes_percent"].is_number());
    assert!(json["savings"]["tokens_percent"].is_number());
}

#[tokio::test]
async fn test_stats_endpoint_with_options() {
    let app = build_router();

    let body = serde_json::json!({
        "json": {"name": "Alice"},
        "encode_options": {
            "delimiter": "tab",
            "indent": 0
        }
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/stats")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_roundtrip_via_http() {
    let app = build_router();

    let original = serde_json::json!({
        "users": [
            {"id": 1, "name": "Alice"},
            {"id": 2, "name": "Bob"}
        ]
    });

    // Step 1: Encode
    let encode_body = serde_json::json!({
        "json": original
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/encode")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&encode_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let encode_result: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let toon = encode_result["toon"].as_str().unwrap();

    // Step 2: Decode
    let decode_body = serde_json::json!({
        "toon": toon
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/decode")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&decode_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let decode_result: serde_json::Value = serde_json::from_slice(&body).unwrap();

    // Verify roundtrip
    assert_eq!(decode_result["json"], original);
}

#[tokio::test]
async fn test_encode_invalid_json_string() {
    let app = build_router();

    let body = serde_json::json!({
        "json": "not valid json {"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/encode")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_cors_headers() {
    let app = build_router();

    let response = app
        .oneshot(
            Request::builder()
                .method("OPTIONS")
                .uri("/api/v1/encode")
                .header("origin", "http://localhost:3000")
                .header("access-control-request-method", "POST")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // CORS preflight should be handled
    assert!(response.status().is_success() || response.status() == StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn test_swagger_ui_accessible() {
    let app = build_router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/swagger-ui/")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Swagger UI should redirect or return content
    assert!(
        response.status().is_success()
            || response.status().is_redirection()
    );
}
