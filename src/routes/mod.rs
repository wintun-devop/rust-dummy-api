use axum::Router;

mod health;

pub fn create_router() -> Router {
    Router::new().merge(health::router())
}
