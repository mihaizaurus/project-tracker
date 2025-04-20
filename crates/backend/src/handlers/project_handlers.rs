use axum::{response::IntoResponse,Json};
use project_tracker_core::HasId;
use serde_json::{json, Value};

use crate::{
    services::project_services,
    dto::project_dto::ProjectDTO,
    Result
};

pub async fn list_projects() -> impl IntoResponse {
    let projects = project_services::get_all_projects();
    Json(projects)
}

pub async fn post_project(payload: Json<ProjectDTO>) -> Result<Json<Value>> {
    let project = project_services::create_project(payload.0)?;

    println!("Created Project from payload:{project:?}");

    Ok(Json(json!({
        "status": "success",
        "message": "Project received",
        "project_id": project.id().to_string(),
    })))
}