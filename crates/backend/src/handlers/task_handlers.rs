use axum::{Json, response::IntoResponse};
use project_tracker_core::HasId;
use serde_json::{Value, json};

use crate::{Result, dto::task_dto::TaskDTO, services::task_services};

pub async fn list_tasks() -> impl IntoResponse {
    let projects = task_services::get_all_tasks();
    Json(projects)
}

pub async fn post_task(payload: Json<TaskDTO>) -> Result<Json<Value>> {
    let task = task_services::create_task(payload.0)?;

    println!("Created Project from payload:{task:?}");

    Ok(Json(json!({
        "status": "success",
        "message": "Task received",
        "project_id": task.id().to_string(),
    })))
}

