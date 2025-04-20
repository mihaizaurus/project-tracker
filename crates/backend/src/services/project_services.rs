use chrono::{DateTime,Utc};

use crate::{
    dto::project_dto::ProjectDTO,
    Result, Error
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
    1. [x] convert DTO to Project
    2. [ ] validate
    3. [ ] push to DB
    4. [ ] return success.failure
    */

    let project = validate(Project::try_from(payload)?)?;

    Ok(project)
}

fn validate(project: Project) -> Result<Project> {
    if has_incorrect_schedule(&project) {
        // use Error::InvalidPayload
    }
    Ok(project)
}

fn has_incorrect_schedule(project: &Project) -> bool {
    match (project.start_date(), project.due_date()) {
        (Some(start_date), Some(due_date)) => due_date < start_date,
        _ => false
    }
}