use axum::http::StatusCode;
use axum::response::Json;
use serde::Serialize;
use serde_json::json;

pub async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "success",
        "message":"api is working well!"
    }))
}

pub fn build_response<T: Serialize>(status_code: StatusCode, body: T) -> (StatusCode, Json<T>) {
    (status_code, Json(body))
}
