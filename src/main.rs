use axum::response::Json;
use axum::{Router, routing::get};
use serde_json::json;
mod config;
use config::config;

#[tokio::main]
async fn main() {
    // Create router with a single health check endpoint
    let app = Router::new().route("/health", get(health_check));

    
    let build_url = config().build_address;
    println!("🚀 Running at {}",build_url);
    let listener = tokio::net::TcpListener::bind(build_url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "success"
    }))
}
