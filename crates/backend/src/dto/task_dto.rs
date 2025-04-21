use std::str::FromStr;
use std::fmt;
use serde::{Deserialize, Serialize};
use chrono::{DateTime,Utc};

use crate::{Error, Result};
use project_tracker_core::{
    models::{
        project::ProjectStatus,
        task::Task,
        tag::Tag,
        person::Person
    },
    builders::task_builder::*,
    HasId,
    id::Id
};

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TaskDTO {
    id: String,
    name: String,
    owner_id: Option<String>,
    description: Option<String>,
    tags: Vec<String>,
    start_date: Option<String>,
    due_date: Option<String>,
    children: Vec<String>,
    dependencies: Vec<String>,
    status: ProjectStatus,
}

impl fmt::Debug for TaskDTO {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Task [[{}]]",self.name)?;
        writeln!(f, "- Task Id:{:?}",self.id)?;
        if let Some(description) = &self.description {
            writeln!(f, "- Task Description: {}",description)?;
        } else {
            writeln!(f, "! No description provided")?;
        }
        if let Some(owner_id) = &self.owner_id {
            writeln!(f, "- Task Owner: {:?}",owner_id)?;
        } else {
            writeln!(f, "! No project owner")?;
        }
        if let Some(start_date) = &self.start_date {
            writeln!(f, "- Task starts on: {}",start_date)?;
        } else {
            writeln!(f, "! No start date defined")?;
        }
        if let Some(due_date) = &self.due_date {
            writeln!(f, "- Task is due on: {}",due_date)?;
        } else {
            writeln!(f, "! No due date defined")?;
        }
        writeln!(f, "- Task has {} children",self.children.len())?;
        writeln!(f, "- Task has {} dependencies",self.dependencies.len())?;
        writeln!(f, "- Task has {} tags",self.tags.len())?;
        Ok(())
    }
}

impl fmt::Display for TaskDTO {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[[{}]]",self.name)?;
        writeln!(f, "- Task Id: {}",self.id)?;
        if let Some(description) = &self.description {
            writeln!(f, "- Task Description: {}",description)?;
        }
        if let Some(owner_id) = &self.owner_id {
            writeln!(f, "- Task Owner: {}",owner_id)?;
        }
        if let Some(start_date) = &self.start_date {
            writeln!(f, "- Task starts on: {}",start_date)?;
        }
        if let Some(due_date) = &self.due_date {
            writeln!(f, "- Task is due on: {}",due_date)?;
        }
        writeln!(f, "- Task has {} children",self.children.len())?;
        writeln!(f, "- Task has {} dependencies",self.dependencies.len())?;
        writeln!(f, "- Task has {} tags",self.tags.len())?;
        Ok(())
    }
}

impl From<Task> for TaskDTO {
    fn from(task: Task) -> Self {
        Self {
            id: task.id().to_string(),
            name: task.name().to_string(),
            owner_id: task.owner_id().map(|id| id.to_string()),
            description: task.description().to_string().clone().into(),
            tags: task.tags().into_iter().map(|id| id.to_string()).collect(),
            start_date: task.start_date().map(|date| date.to_rfc3339()),
            due_date: task.due_date().map(|date| date.to_rfc3339()),
            children: task.children().into_iter().map(|child| child.to_string()).collect(),
            dependencies: task.dependencies().into_iter().map(|id| id.to_string()).collect(),
            status: task.status(),
        }
    }
}

impl TaskDTO {
    pub fn id(&self) -> String {
        self.id.clone()
    }
}

impl TryFrom<TaskDTO> for Task {
    type Error = Error; //temporary, should be replaced with better errors.

    fn try_from(dto: TaskDTO) -> Result<Self> {
        let id = Id::<Task>::from_str(&dto.id)?;
        let name = &dto.name;
        let owner_id = match dto.owner_id {
            Some(ref owner_id_str) => Some(Id::<Person>::from_str(owner_id_str)?),
            None => None
        };
        let description = match &dto.description {
            Some(desc) => desc,
            None => ""
        };
        let tags: Vec<Id<Tag>> = dto.tags
            .into_iter()
            .map(|id| Id::from_str(&id).map_err(|_| Error::ProjectError(format!("Invalid task id: {id:?}"))))
            .collect::<Result<Vec<_>>>()?;
        let start_date = match dto.start_date {
            Some(date_string) => Some(date_string.parse::<DateTime<Utc>>().map_err(|_| Error::ProjectError(format!("Invalid task start date: {date_string:?}")))?),
            None => None,
        };
        let due_date = match dto.due_date {
            Some(date_string) => Some(date_string.parse::<DateTime<Utc>>().map_err(|_| Error::ProjectError(format!("Invalid task due date: {date_string:?}")))?),
            None => None,
        };
        let children: Vec<Id<Task>> = dto.children
            .into_iter()
            .map(|id| Id::from_str(&id).map_err(|_| Error::ProjectError(format!("Invalid child task: {id:?}"))))
            .collect::<Result<Vec<_>>>()?;
        let dependencies: Vec<Id<Task>> = dto.dependencies
            .into_iter()
            .map(|id| Id::from_str(&id).map_err(|_| Error::ProjectError(format!("Invalid child dependency: {id:?}"))))
            .collect::<Result<Vec<_>>>()?;

        Ok(TaskBuilder::new()
            .with_id(id)
            .with_name(name)
            .with_owner_id(owner_id)
            .with_description(description)
            .with_tags(tags)
            .with_start_date(start_date)
            .with_due_date(due_date)
            .with_children(children)
            .with_dependencies(dependencies)
            .with_status(dto.status)
            .build())
    }
}
