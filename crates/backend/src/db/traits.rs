use project_tracker_core::{id::Id,models::project::Project};
use crate::{Result};
use async_trait::async_trait;

#[async_trait]
pub trait ProjectRepository {
    async fn create(&self, project: Project) -> Result<()>;
    async fn get_by_id(&self, id: Id<Project>) -> Result<Option<Project>>;
}