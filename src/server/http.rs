//! HTTP REST API server implementation using axum.

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::core::{
    self, DecodeRequest, DecodeResponse, EncodeOptionsInput, EncodeRequest, EncodeResponse,
    HealthResponse, StatsRequest, StatsResponse, ToonCoreError, ValidateRequest, ValidateResponse,
};

/// Application state shared across handlers.
#[derive(Clone)]
pub struct AppState {
    pub version: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

/// API error response.
#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
pub struct ApiError {
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ErrorDetails>,
}

/// Error details for parse errors.
#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
pub struct ErrorDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<String>,
}

impl From<ToonCoreError> for ApiError {
    fn from(e: ToonCoreError) -> Self {
        match e {
            ToonCoreError::ParseError {
                message,
                line,
                column,
                suggestion,
            } => ApiError {
                error: message,
                details: Some(ErrorDetails {
                    line: Some(line),
                    column: Some(column),
                    suggestion,
                }),
            },
            ToonCoreError::LengthMismatch { expected, found } => ApiError {
                error: format!("Array length mismatch: expected {}, found {}", expected, found),
                details: None,
            },
            other => ApiError {
                error: other.to_string(),
                details: None,
            },
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, Json(self)).into_response()
    }
}

/// OpenAPI documentation.
#[derive(OpenApi)]
#[openapi(
    paths(
        health,
        encode,
        decode,
        validate,
        stats,
    ),
    components(
        schemas(
            HealthResponse,
            EncodeRequest,
            EncodeResponse,
            DecodeRequest,
            DecodeResponse,
            ValidateRequest,
            ValidateResponse,
            StatsRequest,
            StatsResponse,
            crate::core::FormatStats,
            crate::core::SavingsStats,
            crate::core::ValidationError,
            crate::core::EncodeOptionsInput,
            ApiError,
            ErrorDetails,
        )
    ),
    tags(
        (name = "toon", description = "TOON format encoding/decoding operations")
    ),
    info(
        title = "TOON MCP HTTP API",
        version = "0.1.0",
        description = "REST API for TOON format encoding/decoding - achieving 18-40% token savings for LLM prompts",
        license(name = "MIT", url = "https://opensource.org/licenses/MIT"),
    )
)]
struct ApiDoc;

/// Build the HTTP router.
pub fn build_router() -> Router {
    let state = Arc::new(AppState::default());

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/health", get(health))
        .route("/api/v1/encode", post(encode))
        .route("/api/v1/decode", post(decode))
        .route("/api/v1/validate", post(validate))
        .route("/api/v1/stats", post(stats))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(cors)
        .with_state(state)
}

/// Run the HTTP server.
pub async fn run_http_server(addr: &str) -> anyhow::Result<()> {
    let app = build_router();

    eprintln!("toon-mcp HTTP server starting on http://{}", addr);
    eprintln!("  API docs: http://{}/swagger-ui/", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Health check endpoint.
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse)
    ),
    tag = "toon"
)]
async fn health(State(state): State<Arc<AppState>>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: state.version.clone(),
    })
}

/// Encode JSON to TOON format.
#[utoipa::path(
    post,
    path = "/api/v1/encode",
    request_body = EncodeRequest,
    responses(
        (status = 200, description = "Successfully encoded", body = EncodeResponse),
        (status = 400, description = "Invalid input", body = ApiError)
    ),
    tag = "toon"
)]
async fn encode(Json(request): Json<EncodeRequest>) -> Result<Json<EncodeResponse>, ApiError> {
    // Parse JSON input
    let json_value = core::parse_json_input(&request.json)?;

    // Build options
    let options = EncodeOptionsInput {
        delimiter: request.delimiter,
        indent: request.indent,
        fold_keys: request.fold_keys,
        flatten_depth: request.flatten_depth,
    };

    // Encode
    let toon = core::encode_json(&json_value, &options)?;

    Ok(Json(EncodeResponse { toon }))
}

/// Decode TOON to JSON format.
#[utoipa::path(
    post,
    path = "/api/v1/decode",
    request_body = DecodeRequest,
    responses(
        (status = 200, description = "Successfully decoded", body = DecodeResponse),
        (status = 400, description = "Invalid TOON syntax", body = ApiError)
    ),
    tag = "toon"
)]
async fn decode(Json(request): Json<DecodeRequest>) -> Result<Json<DecodeResponse>, ApiError> {
    let json = core::decode_toon(&request.toon, &request)?;
    Ok(Json(DecodeResponse { json }))
}

/// Validate TOON syntax.
#[utoipa::path(
    post,
    path = "/api/v1/validate",
    request_body = ValidateRequest,
    responses(
        (status = 200, description = "Validation result", body = ValidateResponse)
    ),
    tag = "toon"
)]
async fn validate(Json(request): Json<ValidateRequest>) -> Json<ValidateResponse> {
    let result = core::validate_toon(&request.toon, request.strict);
    Json(result)
}

/// Compare JSON and TOON statistics.
#[utoipa::path(
    post,
    path = "/api/v1/stats",
    request_body = StatsRequest,
    responses(
        (status = 200, description = "Statistics comparison", body = StatsResponse),
        (status = 400, description = "Invalid input", body = ApiError)
    ),
    tag = "toon"
)]
async fn stats(Json(request): Json<StatsRequest>) -> Result<Json<StatsResponse>, ApiError> {
    let json_value = core::parse_json_input(&request.json)?;
    let stats = core::compute_stats(&json_value, &request.encode_options)?;
    Ok(Json(stats))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_health_endpoint() {
        let app = build_router();

        let response = app
            .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_encode_endpoint() {
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
    }

    #[tokio::test]
    async fn test_decode_endpoint() {
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
    }

    #[tokio::test]
    async fn test_validate_endpoint() {
        let app = build_router();

        let body = serde_json::json!({
            "toon": "name: Alice"
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
            "json": {"name": "Alice", "age": 30},
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
    }
}
