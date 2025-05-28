use chrono::Utc;
use crate::{
    dto::project_dto::ProjectDTO,
    Result, Error,
    db::project_repository::ProjectRepository,
};
use project_tracker_core::{
    factories::project_factory::*,
    models::project::{Project, ProjectStatus, ProjectSubElement}, HasId
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

pub fn get_project_from_id(id: String) -> Result<ProjectDTO> {
    /* TODO:
    1. [ ] make call to DB
    2. [ ] return success/failure
    */

    let project = ProjectDTO::from(basic_project()); //placeholder
    Ok(project)
}

pub fn create_project(payload: ProjectDTO) -> Result<Project> {
    /* TODO:
    1. [x] convert DTO to Project
    2. [ ] validate
    3. [ ] push to DB
    4. [ ] return success/failure
    */

    let project = validate(Project::try_from(payload)?)?;

    Ok(project)
}

fn validate(project: Project) -> Result<Project> {
    let mut errors: Vec<Error> = Vec::new();

    if has_incorrect_schedule(&project) {
        errors.push(Error::InvalidPayload("Provided project has incorrect schedule".into()));
    }
    if has_inconsistent_status(&project) {
        errors.push(Error::InvalidPayload("Provided project has status inconsistent with provided data".into()));
    }
    if is_own_parent(&project) {
        errors.push(Error::InvalidPayload("Provided project cannot be its own parent".into()));
    }
    if depends_on_self(&project) {
        errors.push(Error::InvalidPayload("Provided project cannot be its own dependency".into()));
    }
    // validate provided tags
    // validate provided tasks
    // validate dependencies 

    println!("errors: {errors:?}");

    if errors.is_empty() {
        Ok(project)
    } else {
        Err(Error::Multiple(errors))
    }
}

fn has_incorrect_schedule(project: &Project) -> bool {
    match (project.start_date(), project.due_date()) {
        (Some(start_date), Some(due_date)) => due_date < start_date,
        _ => false
    }
}

fn has_inconsistent_status(project: &Project) -> bool {
    match project.status() {
        ProjectStatus::NotStarted => is_invalid_not_started_project(project),
        ProjectStatus::Planned => is_invalid_planned_project(project),
        ProjectStatus::InProgress => is_invalid_in_progress_project(project),
        ProjectStatus::InReview => is_invalid_in_review_project(project),
        ProjectStatus::Completed => is_invalid_completed_project(project),
        _ => false //No restrictions on canceled and archived projects
    }
}

fn is_invalid_not_started_project(project: &Project) -> bool {
    // not started project should not have a start date in the past
    match project.start_date() {
        Some(start_date) => start_date <= Utc::now(),
        _ => false
    }
}

fn is_invalid_planned_project(project: &Project) -> bool {
    // planned project should have a start date for the future
    match project.start_date() {
        Some(start_date) => {
            start_date <= Utc::now()
        },
        _ => true
    }
}

fn is_invalid_in_progress_project(project: &Project) -> bool {
    // in progress project should have a start date in the past (and an optional due date in the future)
    match (project.start_date(), project.due_date()) {
        (Some(start_date), Some(due_date)) => {
            !(start_date <= Utc::now() && due_date >= Utc::now())
        },
        (Some(start_date), None) => {
            !(start_date <= Utc::now())
        }
        _ => true
    }
}

fn is_invalid_in_review_project(project: &Project) -> bool {
    // in review project should have a start date in the past and a fixed due date in the future
    match (project.start_date(), project.due_date()) {
        (Some(start_date), Some(due_date)) => {
            !(start_date <= Utc::now() && due_date >= Utc::now())
        },
        _ => true
    }
}

fn is_invalid_completed_project(project: &Project) -> bool {
    // completed project should have both a start date and a due date in the past
    match (project.start_date(), project.due_date()) {
        (Some(start_date), Some(due_date)) => {
            !(start_date <= Utc::now() && due_date <= Utc::now())
        },
        _ => true
    }
}

fn is_own_parent(project: &Project) -> bool {
    project.children().contains(&ProjectSubElement::Project(project.id()))
}

fn depends_on_self(project: &Project) -> bool {
    project.dependencies().contains(&project.id())
}