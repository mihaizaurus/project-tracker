use axum::{
    extract::{Path, Query}, 
    response::IntoResponse, 
    Json
};
use project_tracker_core::HasId;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{
    services::project_services,
    dto::project_dto::ProjectDTO,
    Result
};

#[derive(Debug, Deserialize)]
pub struct ProjectParameters {
    id: String
}

pub async fn list_projects() -> impl IntoResponse {
    let projects = project_services::get_all_projects();
    Json(projects)
}

pub async fn get_project_from_parameters(Query(params): Query<ProjectParameters>) -> impl IntoResponse {
    let id = params.id;
    match project_services::get_project_from_id(id) {
        Ok(project) => Json(project).into_response(),
        Err(error) => error.into_response(),
    }
}

pub async fn get_project_from_path(Path(id): Path<String>) -> impl IntoResponse {
    match project_services::get_project_from_id(id) {
        Ok(project) => Json(project).into_response(),
        Err(error) => error.into_response(),
    }
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