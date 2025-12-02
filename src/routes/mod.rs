use axum::Router;

mod health;
mod items;

pub fn create_router() -> Router {
   /*
    //old structure
    Router::new()
        .merge(health::router())
        .nest("/items", items::router()) 
    */

    /* 1). build each module's router  */
    let health_router = health::router();
    let items_router = items::router();

    /* 2).combine them under an aps router */
    let api_routers = Router::new()
    .nest("/health", health_router)
    .nest("/items", items_router);
    /* 3). mount api_router at /api/v1 */
    
    Router::new().nest("/api/v1", api_routers)
}
