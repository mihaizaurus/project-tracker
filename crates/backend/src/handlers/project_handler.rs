use axum::{response::IntoResponse,Json};
use crate::services::project_service;

pub async fn list_projects() -> impl IntoResponse {
    let projects = project_service::get_all_projects();
    Json(projects)
}