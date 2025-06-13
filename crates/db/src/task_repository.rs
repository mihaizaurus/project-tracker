use std::sync::Arc;
use project_tracker_core::{
    id::Id, 
    models::{task::Task, project::ProjectStatus, person::Person, tag::Tag}, 
    builders::task_builder::TaskBuilder,
    HasId
};
use crate::{database::Database, DatabaseError, Result};
use async_trait::async_trait;
use log::{debug, error};
use serde::{Serialize, Deserialize};
use surrealdb::sql::Datetime as SurrealDatetime;

// DTO for database operations (excludes id since SurrealDB manages it)
#[derive(Serialize, Deserialize, Debug)]
struct TaskRecord {
    name: String,
    owner_id: Option<String>,
    description: Option<String>,
    tags: Option<Vec<String>>,
    start_date: Option<SurrealDatetime>,
    due_date: Option<SurrealDatetime>,
    children: Option<Vec<String>>,
    dependencies: Option<Vec<String>>,
    status: String,
}

impl From<Task> for TaskRecord {
    fn from(task: Task) -> Self {
        let children_strings: Vec<String> = task.children().iter().map(|id| id.to_string()).collect();
        let dependencies_strings: Vec<String> = task.dependencies().iter().map(|id| id.to_string()).collect();
        let tags_strings: Vec<String> = task.tags().iter().map(|id| id.to_string()).collect();
        
        
        Self {
            name: task.name().to_string(),
            owner_id: task.owner_id().map(|id| id.clone().to_string()),
            description: if task.description().is_empty() { 
                None 
            } else { 
                Some(task.description().to_string()) 
            },
            tags: if tags_strings.is_empty() { None } else { Some(tags_strings) },
            start_date: task.start_date().map(SurrealDatetime::from),
            due_date: task.due_date().map(SurrealDatetime::from),
            children: if children_strings.is_empty() { None } else { Some(children_strings) },
            dependencies: if dependencies_strings.is_empty() { None } else { Some(dependencies_strings) },
            status: task.status().to_string(),
        }
    }
}

impl TaskRecord {
    fn into_task(self, id: Id<Task>) -> Result<Task> {
        let record = self;
        
        // Parse status
        let status = match record.status.as_str() {
            "NotStarted" => ProjectStatus::NotStarted,
            "Planned" => ProjectStatus::Planned,
            "InProgress" => ProjectStatus::InProgress,
            "InReview" => ProjectStatus::InReview,
            "Completed" => ProjectStatus::Completed,
            "Archived" => ProjectStatus::Archived,
            "Canceled" => ProjectStatus::Canceled,
            s => return Err(DatabaseError::QueryError(format!("Invalid status: {}", s))),
        };
        
        // Parse owner_id
        let owner_id = if let Some(owner_str) = record.owner_id {
            Some(owner_str.parse::<Id<Person>>()
                .map_err(|e| DatabaseError::QueryError(format!("Invalid owner_id: {:?}", e)))?)
        } else {
            None
        };
        
        // Parse tags
        let tags = record.tags.unwrap_or_default().into_iter()
            .map(|tag_str| tag_str.parse::<Id<Tag>>())
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| DatabaseError::QueryError(format!("Invalid tag ID: {:?}", e)))?;
        
        // Parse children
        let children = record.children.unwrap_or_default().into_iter()
            .map(|child_str| child_str.parse::<Id<Task>>())
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| DatabaseError::QueryError(format!("Invalid child task ID: {:?}", e)))?;
        
        
        // Parse dependencies
        let dependencies = record.dependencies.unwrap_or_default().into_iter()
            .map(|dep_str| dep_str.parse::<Id<Task>>())
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| DatabaseError::QueryError(format!("Invalid dependency ID: {:?}", e)))?;
        
        // Build the task
        let mut builder = TaskBuilder::new()
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
pub trait TaskRepository {
    async fn create(&self, task: Task) -> Result<()>;
    async fn get_by_id(&self, id: Id<Task>) -> Result<Option<Task>>;
}

pub struct ProdTaskRepository {
    db: Arc<Database>,
}

impl ProdTaskRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl TaskRepository for ProdTaskRepository {
    async fn create(&self, task: Task) -> Result<()> {
        let task_id = task.id();
        debug!("Creating task with ID: {}", task_id);
        
        let id_str = task_id.to_string();
        let task_record = TaskRecord::from(task);
        
        let result: Result<Option<TaskRecord>> = self.db.client()
            .create(("task", id_str.as_str()))
            .content(task_record)
            .await
            .map_err(|e| DatabaseError::QueryError(format!("Failed to create task: {}", e)));
        
        match result {
            Ok(_created_record) => {
                debug!("Successfully created task with ID: {}", task_id);
                Ok(())
            }
            Err(e) => {
                error!("Failed to create task: {:?}", e);
                Err(e)
            }
        }
    }

    async fn get_by_id(&self, id: Id<Task>) -> Result<Option<Task>> {
        debug!("Fetching task with ID: {}", id);
        
        // Convert the custom ID to the format SurrealDB expects
        let id_str = id.to_string();
        
        // Query using the string ID - get the record first
        let result: Result<Option<TaskRecord>> = self.db.client()
            .select(("task", id_str.as_str()))
            .await
            .map_err(|e| DatabaseError::QueryError(format!("Failed to get task by ID: {}", e)));
        
        match result {
            Ok(Some(task_record)) => {
                debug!("Found task record with ID: {}", id);
                // Convert from TaskRecord back to Task
                match task_record.into_task(id) {
                    Ok(task) => Ok(Some(task)),
                    Err(e) => {
                        error!("Failed to convert task record to domain object: {:?}", e);
                        Err(e)
                    }
                }
            }
            Ok(None) => {
                debug!("No task found with ID: {}", id);
                Ok(None)
            }
            Err(e) => {
                error!("Failed to get task by ID: {:?}", e);
                Err(e)
            }
        }
    }
}

// Additional repository methods that can be added in the future
impl ProdTaskRepository {
    #[allow(dead_code)]
    pub async fn update(&self, task: Task) -> Result<()> {
        let task_id = task.id();
        debug!("Updating task with ID: {}", task_id);
        
        let id_str = task_id.to_string();
        
        let result: Result<Option<Task>> = self.db.client()
            .update(("task", id_str.as_str()))
            .content(task)
            .await
            .map_err(|e| DatabaseError::QueryError(format!("Failed to update task: {}", e)));
        
        match result {
            Ok(Some(_)) => {
                debug!("Successfully updated task with ID: {}", task_id);
                Ok(())
            }
            Ok(None) => {
                error!("Task not found for update: {}", task_id);
                Err(DatabaseError::QueryError(format!("Task with ID {} not found", task_id)))
            }
            Err(e) => {
                error!("Failed to update task: {:?}", e);
                Err(e)
            }
        }
    }
    
    #[allow(dead_code)]
    pub async fn delete(&self, id: Id<Task>) -> Result<()> {
        debug!("Deleting task with ID: {}", id);
        
        let id_str = id.to_string();
        
        let result: Result<Option<Task>> = self.db.client()
            .delete(("task", id_str.as_str()))
            .await
            .map_err(|e| DatabaseError::QueryError(format!("Failed to delete task: {}", e)));
        
        match result {
            Ok(Some(_)) => {
                debug!("Successfully deleted task with ID: {}", id);
                Ok(())
            }
            Ok(None) => {
                error!("Task not found for deletion: {}", id);
                Err(DatabaseError::QueryError(format!("Task with ID {} not found", id)))
            }
            Err(e) => {
                error!("Failed to delete task: {:?}", e);
                Err(e)
            }
        }
    }
    
    #[allow(dead_code)]
    pub async fn get_all(&self) -> Result<Vec<Task>> {
        debug!("Fetching all tasks");
        
        let result: Result<Vec<Task>> = self.db.client()
            .select("task")
            .await
            .map_err(|e| DatabaseError::QueryError(format!("Failed to get all tasks: {}", e)));
        
        match result {
            Ok(tasks) => {
                debug!("Found {} tasks", tasks.len());
                Ok(tasks)
            }
            Err(e) => {
                error!("Failed to get all tasks: {:?}", e);
                Err(e)
            }
        }
    }
    
    #[allow(dead_code)]
    pub async fn get_by_owner(&self, owner_id: Id<project_tracker_core::models::person::Person>) -> Result<Vec<Task>> {
        debug!("Fetching tasks for owner: {}", owner_id);
        
        let query = format!(
            "SELECT * FROM task WHERE owner_id = '{}'",
            owner_id.to_string()
        );
        
        let mut response = self.db.client()
            .query(query)
            .await
            .map_err(|e| DatabaseError::QueryError(format!("Failed to get tasks by owner: {}", e)))?;
        
        let tasks: Vec<Task> = response
            .take(0)
            .map_err(|e| DatabaseError::QueryError(format!("Failed to parse tasks by owner: {}", e)))?;
        
        debug!("Found {} tasks for owner: {}", tasks.len(), owner_id);
        Ok(tasks)
    }
    
    #[allow(dead_code)]
    pub async fn get_by_parent_project(&self, project_id: Id<project_tracker_core::models::project::Project>) -> Result<Vec<Task>> {
        debug!("Fetching tasks for project: {}", project_id);
        
        // This would require tracking parent project in tasks or using a junction table
        // For now, we can search through all projects and find tasks that are children
        let query = format!(
            "SELECT * FROM project WHERE id = '{}' FETCH children",
            project_id.to_string()
        );
        
        let _response = self.db.client()
            .query(query)
            .await
            .map_err(|e| DatabaseError::QueryError(format!("Failed to get tasks by project: {}", e)))?;
        
        // Extract task IDs from project children and fetch them
        // This is a placeholder - actual implementation would depend on how parent-child relationships are stored
        Ok(vec![])
    }
}