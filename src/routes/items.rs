use axum::{Router, routing::get};

// use crate::handlers::items_handler::{
//     list_items, get_item, create_item, update_item, delete_item,
// };

use crate::utils::response_helpers::health_check;

pub fn router() -> Router {
    Router::new()
        .route("/", get(health_check).post(health_check))
        .route(
            "/id",
            get(health_check).put(health_check).delete(health_check)
        )
}
