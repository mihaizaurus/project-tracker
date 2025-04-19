use axum::{Router,routing::get};

use crate::handlers::project_handler;

pub fn routes() -> Router {
    Router::new().route("/projects",get(project_handler::list_projects))
}

async fn projects() -> &'static str {
    "OK"
}
