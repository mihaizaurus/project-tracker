use axum::{routing::{get,post},Router};

use crate::handlers::project_handlers;

pub fn routes() -> Router {
    Router::new()
        .route("/projects",get(project_handlers::list_projects))
        .route("/projects",post(project_handlers::post_project))
}