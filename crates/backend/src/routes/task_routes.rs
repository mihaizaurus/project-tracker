use axum::{routing::{get,post},Router};

use crate::handlers::task_handlers;

pub fn routes() -> Router {
    Router::new()
        .route("/tasks",get(task_handlers::list_tasks))
        .route("/tasks",post(task_handlers::post_task))
}