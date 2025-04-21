use axum::{routing::{get,post},Router};

use crate::handlers::project_handlers;

pub fn routes() -> Router {
    Router::new()
        .route("/projects",get(project_handlers::list_projects))
        .route("/project",get(project_handlers::get_project_from_parameters))
        .route("/project/{id}",get(project_handlers::get_project_from_path))
        .route("/projects",post(project_handlers::post_project))
}