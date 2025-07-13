use crate::{DatabaseError, Result, database::Database};
use async_trait::async_trait;
use log::{debug, error};
use project_tracker_core::{
    HasId,
    builders::project_builder::ProjectBuilder,
    id::Id,
    models::{
        person::Person,
        project::Project,
        schedulable::{Schedulable, SchedulableItem, SchedulableItemStatus},
        tag::Tag,
        task::Task,
    },
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::sql::Datetime as SurrealDatetime;

// DTO for database operations (excludes id since SurrealDB manages it)
#[derive(Serialize, Deserialize)]
struct ProjectRecord {
    name: String,
    owner_id: Option<String>,
    description: Option<String>,
    tags: Vec<String>,
    start_date: Option<SurrealDatetime>,
    due_date: Option<SurrealDatetime>,
    children: Vec<String>, // Store as string array instead of enum
    dependencies: Vec<String>,
    status: String,
}

impl From<Project> for ProjectRecord {
    fn from(project: Project) -> Self {
        Self {
            name: project.name().to_string(),
            owner_id: project.owner_id().map(|id| id.clone().to_string()),
            description: if project.description().is_empty() {
                None
            } else {
                Some(project.description().to_string())
            },
            tags: project.tags().iter().map(|id| id.to_string()).collect(),
            start_date: project.start_date().map(SurrealDatetime::from),
            due_date: project.due_date().map(SurrealDatetime::from),
            children: project
                .children()
                .iter()
                .map(|child| match child {
                    SchedulableItem::Project(id) => id.to_string(),
                    SchedulableItem::Task(id) => id.to_string(),
                })
                .collect(),
            dependencies: project
                .dependencies()
                .iter()
                .map(|id| id.to_string())
                .collect(),
            status: project.status().to_string(),
        }
    }
}

impl ProjectRecord {
    fn into_project(self, id: Id<Project>) -> Result<Project> {
        let record = self;
        // Parse status
        let status = match record.status.as_str() {
            "NotStarted" => SchedulableItemStatus::NotStarted,
            "Planned" => SchedulableItemStatus::Planned,
            "InProgress" => SchedulableItemStatus::InProgress,
            "InReview" => SchedulableItemStatus::InReview,
            "Completed" => SchedulableItemStatus::Completed,
            "Archived" => SchedulableItemStatus::Archived,
            "Canceled" => SchedulableItemStatus::Canceled,
            s => return Err(DatabaseError::QueryError(format!("Invalid status: {}", s))),
        };

        // Parse owner_id
        let owner_id = if let Some(owner_str) = record.owner_id {
            Some(
                owner_str
                    .parse::<Id<Person>>()
                    .map_err(|e| DatabaseError::QueryError(format!("Invalid owner_id: {:?}", e)))?,
            )
        } else {
            None
        };

        // Parse tags
        let tags = record
            .tags
            .into_iter()
            .map(|tag_str| tag_str.parse::<Id<Tag>>())
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| DatabaseError::QueryError(format!("Invalid tag ID: {:?}", e)))?;

        // Parse children
        let children = record
            .children
            .into_iter()
            .map(|child_str| {
                if child_str.starts_with("project-") {
                    child_str
                        .parse::<Id<Project>>()
                        .map(SchedulableItem::Project)
                        .map_err(|e| {
                            DatabaseError::QueryError(format!("Invalid project child ID: {:?}", e))
                        })
                } else if child_str.starts_with("task-") {
                    child_str
                        .parse::<Id<Task>>()
                        .map(SchedulableItem::Task)
                        .map_err(|e| {
                            DatabaseError::QueryError(format!("Invalid task child ID: {:?}", e))
                        })
                } else {
                    Err(DatabaseError::QueryError(format!(
                        "Unknown child type: {}",
                        child_str
                    )))
                }
            })
            .collect::<Result<Vec<_>>>()?;

        // Parse dependencies
        let dependencies = record
            .dependencies
            .into_iter()
            .map(|dep_str| dep_str.parse::<Id<Project>>())
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| DatabaseError::QueryError(format!("Invalid dependency ID: {:?}", e)))?;

        // Build the project
        let mut builder = ProjectBuilder::new()
            .with_id(id)
            .with_name(&record.name)
            .with_status(status)
            .with_tags(tags)
            .with_children(children)
            .with_dependencies(dependencies);

        if let Some(owner_id) = owner_id {
            builder = builder.with_owner_id(Some(owner_id));
        }

        if let Some(description) = record.description {
            if !description.is_empty() {
                builder = builder.with_description(&description);
            }
        }

        if let Some(start_date) = record.start_date {
            builder = builder.with_start_date(Some(start_date.into()));
        }

        if let Some(due_date) = record.due_date {
            builder = builder.with_due_date(Some(due_date.into()));
        }

        Ok(builder.build())
    }
}

// Define the repository trait locally to avoid circular dependencies
#[async_trait]
pub trait ProjectRepository {
    async fn create(&self, project: Project) -> Result<()>;
    async fn get_by_id(&self, id: Id<Project>) -> Result<Option<Project>>;
}

pub struct ProdProjectRepository {
    db: Arc<Database>,
}

impl ProdProjectRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl ProjectRepository for ProdProjectRepository {
    async fn create(&self, project: Project) -> Result<()> {
        let project_id = project.id();
        debug!("Creating project with ID: {}", project_id);

        let id_str = project_id.to_string();
        let project_record = ProjectRecord::from(project);

        let result: Result<Option<ProjectRecord>> = self
            .db
            .client()
            .create(("project", id_str.as_str()))
            .content(project_record)
            .await
            .map_err(|e| DatabaseError::QueryError(format!("Failed to create project: {}", e)));

        match result {
            Ok(_) => {
                debug!("Successfully created project with ID: {}", project_id);
                Ok(())
            }
            Err(e) => {
                error!("Failed to create project: {:?}", e);
                Err(e)
            }
        }
    }

    async fn get_by_id(&self, id: Id<Project>) -> Result<Option<Project>> {
        debug!("Fetching project with ID: {}", id);

        // Convert the custom ID to the format SurrealDB expects
        let id_str = id.to_string();

        // Query using the string ID - get the record first
        let result: Result<Option<ProjectRecord>> = self
            .db
            .client()
            .select(("project", id_str.as_str()))
            .await
            .map_err(|e| DatabaseError::QueryError(format!("Failed to get project by ID: {}", e)));

        match result {
            Ok(Some(project_record)) => {
                debug!("Found project record with ID: {}", id);
                // Convert from ProjectRecord back to Project
                match project_record.into_project(id) {
                    Ok(project) => Ok(Some(project)),
                    Err(e) => {
                        error!("Failed to convert project record to domain object: {:?}", e);
                        Err(e)
                    }
                }
            }
            Ok(None) => {
                debug!("No project found with ID: {}", id);
                Ok(None)
            }
            Err(e) => {
                error!("Failed to get project by ID: {:?}", e);
                Err(e)
            }
        }
    }
}

// Additional repository methods that can be added in the future
impl ProdProjectRepository {
    #[allow(dead_code)]
    pub async fn update(&self, project: Project) -> Result<()> {
        let project_id = project.id();
        debug!("Updating project with ID: {}", project_id);

        let id_str = project_id.to_string();

        let result: Result<Option<Project>> = self
            .db
            .client()
            .update(("project", id_str.as_str()))
            .content(project)
            .await
            .map_err(|e| DatabaseError::QueryError(format!("Failed to update project: {}", e)));

        match result {
            Ok(Some(_)) => {
                debug!("Successfully updated project with ID: {}", project_id);
                Ok(())
            }
            Ok(None) => {
                error!("Project not found for update: {}", project_id);
                Err(DatabaseError::QueryError(format!(
                    "Project with ID {} not found",
                    project_id
                )))
            }
            Err(e) => {
                error!("Failed to update project: {:?}", e);
                Err(e)
            }
        }
    }

    #[allow(dead_code)]
    pub async fn delete(&self, id: Id<Project>) -> Result<()> {
        debug!("Deleting project with ID: {}", id);

        let id_str = id.to_string();

        let result: Result<Option<Project>> = self
            .db
            .client()
            .delete(("project", id_str.as_str()))
            .await
            .map_err(|e| DatabaseError::QueryError(format!("Failed to delete project: {}", e)));

        match result {
            Ok(Some(_)) => {
                debug!("Successfully deleted project with ID: {}", id);
                Ok(())
            }
            Ok(None) => {
                error!("Project not found for deletion: {}", id);
                Err(DatabaseError::QueryError(format!(
                    "Project with ID {} not found",
                    id
                )))
            }
            Err(e) => {
                error!("Failed to delete project: {:?}", e);
                Err(e)
            }
        }
    }

    #[allow(dead_code)]
    pub async fn get_all(&self) -> Result<Vec<Project>> {
        debug!("Fetching all projects");

        let result: Result<Vec<Project>> =
            self.db.client().select("project").await.map_err(|e| {
                DatabaseError::QueryError(format!("Failed to get all projects: {}", e))
            });

        match result {
            Ok(projects) => {
                debug!("Found {} projects", projects.len());
                Ok(projects)
            }
            Err(e) => {
                error!("Failed to get all projects: {:?}", e);
                Err(e)
            }
        }
    }

    #[allow(dead_code)]
    pub async fn get_by_owner(
        &self,
        owner_id: Id<project_tracker_core::models::person::Person>,
    ) -> Result<Vec<Project>> {
        debug!("Fetching projects for owner: {}", owner_id);

        let query = format!(
            "SELECT * FROM project WHERE owner_id = '{}'",
            owner_id.to_string()
        );

        let mut response = self.db.client().query(query).await.map_err(|e| {
            DatabaseError::QueryError(format!("Failed to get projects by owner: {}", e))
        })?;

        let projects: Vec<Project> = response.take(0).map_err(|e| {
            DatabaseError::QueryError(format!("Failed to parse projects by owner: {}", e))
        })?;

        debug!("Found {} projects for owner: {}", projects.len(), owner_id);
        Ok(projects)
    }
}

