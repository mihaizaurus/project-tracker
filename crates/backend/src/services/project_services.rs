use crate::{
    dto::project_dto::ProjectDTO,
    Result
};
use project_tracker_core::{
    factories::project_factory::*,
    models::project::Project
};

pub fn get_all_projects() -> Vec<ProjectDTO> {
    // TODO
    vec![
        ProjectDTO::from(sample_project_for_dto()),
        ProjectDTO::from(sample_project_for_dto()),
        ProjectDTO::from(sample_project_for_dto()),
        ProjectDTO::from(sample_project_for_dto()),
        ProjectDTO::from(sample_project_for_dto()),
        ProjectDTO::from(sample_project_for_dto()),
    ]
}

pub fn create_project(payload: ProjectDTO) -> Result<Project> {
    /* TODO:
    1. convert DTO to Project
    2. validate
    3. push to DB
    4. return success.failure
    */

    let project = Project::try_from(payload)?;

    Ok(project)
}