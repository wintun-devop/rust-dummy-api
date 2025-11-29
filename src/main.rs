
use axum::{Router, routing::get};
mod config;
mod utils;
use config::config;
use utils::response_helpers::health_check;

#[tokio::main]
async fn main() {
    // Create router with a single health check endpoint
    let app = Router::new().route("/health", get(health_check));

    
    let build_url = config().build_address;
    println!("🚀 Running at {}.",build_url);
    let listener = tokio::net::TcpListener::bind(build_url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

