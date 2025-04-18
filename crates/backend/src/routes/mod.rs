pub mod health;
pub mod projects;
pub mod tasks;
pub mod tags;
pub mod people;

use axum::Router;

pub fn create_router() -> Router {
    Router::new()
        .merge(health::routes())
        .merge(projects::routes())
}