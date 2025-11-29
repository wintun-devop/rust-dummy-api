use axum::Router;

mod health;
mod items;

pub fn create_router() -> Router {
    Router::new()
        .merge(health::router())
        .nest("/items", items::router())
}
