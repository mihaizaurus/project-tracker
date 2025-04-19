use crate::dto::project_dto::ProjectDTO;
use project_tracker_core::factories::project_factory::*;

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