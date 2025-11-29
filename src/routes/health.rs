use axum::{Router, routing::get};

use crate::utils::response_helpers::health_check;

pub fn router() -> Router {
    Router::new().route("/health", get(health_check))
}
