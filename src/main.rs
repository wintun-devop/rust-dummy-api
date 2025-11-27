use axum::{routing::get, Router};
use axum::response::Json;
use serde_json::json;

#[tokio::main]
async fn main() {
    // Create router with a single health check endpoint
    let app = Router::new()
        .route("/health", get(health_check));

    println!("🚀 Running at http://127.0.0.1:8000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "success"
    }))
}
