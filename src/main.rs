use axum::{Router};
mod config;
mod utils;
use config::config;
mod routes;


#[tokio::main]
async fn main() {
    let app : Router = routes::create_router();
    let build_url = config().build_address;
    println!("🚀 Running at {}.", build_url);
    let listener = tokio::net::TcpListener::bind(build_url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
