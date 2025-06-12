use project_tracker_core::{
    models::{person::Person, project::Project, tag::Tag},
    builders::{project_builder::ProjectBuilder, tag_builder::TagBuilder},
    id::Id,
    HasId,
};
use anyhow::Result;

use crate::models::ProjectFormState;

pub struct ProjectHandler;

impl ProjectHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn create_project(
        &self,
        form_state: &ProjectFormState,
        default_user: &Person,
    ) -> Result<Project> {
        let mut builder = ProjectBuilder::new()
            .with_name(&form_state.name)
            .with_owner_id(Some(default_user.id()));

        if !form_state.description.trim().is_empty() {
            builder = builder.with_description(&form_state.description);
        }

        // Convert string tags to Tag IDs
        let tag_ids = self.create_tags_from_strings(&form_state.tags).await?;
        if !tag_ids.is_empty() {
            builder = builder.with_tags(tag_ids);
        }

        Ok(Project::from_builder(builder))
    }

    async fn create_tags_from_strings(&self, tag_names: &[String]) -> Result<Vec<Id<Tag>>> {
        let mut tag_ids = Vec::new();
        
        for tag_name in tag_names {
            let tag = TagBuilder::new()
                .with_name(tag_name)
                .build();
            tag_ids.push(tag.id());
        }

        Ok(tag_ids)
    }
}