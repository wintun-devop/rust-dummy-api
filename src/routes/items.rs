use axum::{Router, routing::get};


use crate::handlers::items_handlers::{
    list_items,get_item,create_item,update_item,delete_item
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(list_items).post(create_item))
        .route(
            "/id",
            get(get_item).put(update_item).delete(delete_item)
        )
}
