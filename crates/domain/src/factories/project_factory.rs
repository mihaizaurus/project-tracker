use crate::models::project::Project;
use crate::models::project_builder::ProjectBuilder;

pub fn sample_project() -> Project {
    ProjectBuilder::new().with_name("This is a sample project title").build()
}