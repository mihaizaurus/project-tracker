use crate::dto::project_dto::ProjectDTO;
use project_tracker_core::factories::project_factory::{self, sample_project};

pub fn get_all_projects() -> Vec<ProjectDTO> {
    // TODO
    vec![
        sample_project_for_dto(),
        sample_project_for_dto(),
        sample_project_for_dto(),
        sample_project_for_dto(),
        sample_project_for_dto(),
        sample_project_for_dto(),
    ]
}