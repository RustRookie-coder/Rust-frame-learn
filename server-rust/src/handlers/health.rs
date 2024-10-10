use axum::{Json, response::IntoResponse, Router};
use axum::routing::get;

pub fn heath_handler() -> Router {
    Router::new().route("/health", get(health_checker_handler))
}
async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Simple CRUD API with Rust, SQLX, Postgres,and Axum";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}
