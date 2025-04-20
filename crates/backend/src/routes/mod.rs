pub mod health_routes;
pub mod project_routes;
pub mod task_routes;
pub mod tag_routes;
pub mod people_routes;

use axum::Router;

pub fn create_router() -> Router {
    Router::new()
        .merge(health_routes::routes())
        .nest("/api",
            Router::new() 
                .merge(project_routes::routes())
        )
}