use axum::{Json, extract::Path};
use serde_json::json;

pub async fn list_items() -> Json<serde_json::Value> {
    // you can add actual business logics here
    Json(json!({ "items": ["item1", "item2"] }))
}

