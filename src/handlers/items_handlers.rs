use axum::{Json, extract::Path};
use serde_json::json;
use axum::http::StatusCode;

use crate::utils::response_helpers::build_response;


// GET /items
pub async fn list_items() -> Json<serde_json::Value> {
    // you can add actual business logics here
    Json(json!({ "items": ["item1", "item2"] }))
}

// GET /items/:id
pub async fn get_item(Path(id): Path<u64>) -> (StatusCode, Json<serde_json::Value>) {
    // Json(json!({ "id": id, "name": "Sample Item" }))
    build_response(
        StatusCode::OK,
        serde_json::json!({ "id": id, "name": "Sample Item" })
    )
}

// POST /items
pub async fn create_item(Json(payload): Json<serde_json::Value>) -> (StatusCode, Json<serde_json::Value>) {
    build_response(
        StatusCode::CREATED,
        serde_json::json!(payload)
    )
}

// PUT /items/:id
pub async fn update_item(Path(id): Path<u64>, Json(payload): Json<serde_json::Value>) -> (StatusCode, Json<serde_json::Value>) {
    // Json(json!({ "updated": { "id": id, "payload": payload } }))
    build_response(
        StatusCode::OK,
        serde_json::json!({ "updated": { "id": id, "payload": payload } })
    )
}

// DELETE /items/:id
pub async fn delete_item(Path(id): Path<u64>) -> (StatusCode, Json<serde_json::Value>) {
    // Json(json!({ "deleted": id }))
    build_response(
        StatusCode::OK,
        serde_json::json!({ "deleted": id })
    )
}