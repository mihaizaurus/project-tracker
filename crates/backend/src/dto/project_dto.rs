use std::str::FromStr;

use crate::{Error, Result};
use chrono::{DateTime, Utc};
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
use std::fmt;

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ProjectDTO {
    id: String,
    name: String,
    owner_id: Option<String>,
    description: Option<String>,
    tags: Vec<String>,
    start_date: Option<String>,
    due_date: Option<String>,
    children: Vec<SchedulableItemDTO>,
    dependencies: Vec<String>,
    status: SchedulableItemStatus,
}

impl ProjectDTO {
    pub fn id(&self) -> String {
        self.id.clone()
    }
}

impl From<Project> for ProjectDTO {
    fn from(project: Project) -> Self {
        Self {
            id: project.id().to_string(),
            name: project.name().to_string(),
            owner_id: project.owner_id().map(|id| id.to_string()),
            description: project.description().to_string().clone().into(),
            tags: project
                .tags()
                .into_iter()
                .map(|id| id.to_string())
                .collect(),
            start_date: project.start_date().map(|date| date.to_rfc3339()),
            due_date: project.due_date().map(|date| date.to_rfc3339()),
            children: project
                .children()
                .into_iter()
                .map(|child| child.into())
                .collect(),
            dependencies: project
                .dependencies()
                .into_iter()
                .map(|id| id.to_string())
                .collect(),
            status: project.status(),
        }
    }
}

impl fmt::Debug for ProjectDTO {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Project [[{}]]", self.name)?;
        writeln!(f, "- Project Id:{:?}", self.id)?;
        if let Some(description) = &self.description {
            writeln!(f, "- Project Description: {}", description)?;
        } else {
            writeln!(f, "! No description provided")?;
        }
        if let Some(owner_id) = &self.owner_id {
            writeln!(f, "- Project Owner: {:?}", owner_id)?;
        } else {
            writeln!(f, "! No project owner")?;
        }
        if let Some(start_date) = &self.start_date {
            writeln!(f, "- Project starts on: {}", start_date)?;
        } else {
            writeln!(f, "! No start date defined")?;
        }
        if let Some(due_date) = &self.due_date {
            writeln!(f, "- Project is due on: {}", due_date)?;
        } else {
            writeln!(f, "! No due date defined")?;
        }
        writeln!(f, "- Project has {} children", self.children.len())?;
        writeln!(f, "- Project has {} dependencies", self.dependencies.len())?;
        writeln!(f, "- Project has {} tags", self.tags.len())?;
        Ok(())
    }
}

impl fmt::Display for ProjectDTO {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[[{}]]", self.name)?;
        writeln!(f, "- Project Id: {}", self.id)?;
        if let Some(description) = &self.description {
            writeln!(f, "- Project Description: {}", description)?;
        }
        if let Some(owner_id) = &self.owner_id {
            writeln!(f, "- Project Owner: {}", owner_id)?;
        }
        if let Some(start_date) = &self.start_date {
            writeln!(f, "- Project starts on: {}", start_date)?;
        }
        if let Some(due_date) = &self.due_date {
            writeln!(f, "- Project is due on: {}", due_date)?;
        }
        writeln!(f, "- Project has {} children", self.children.len())?;
        writeln!(f, "- Project has {} dependencies", self.dependencies.len())?;
        writeln!(f, "- Project has {} tags", self.tags.len())?;
        Ok(())
    }
}

impl TryFrom<ProjectDTO> for Project {
    type Error = Error; //temporary, should be replaced with better errors.

    fn try_from(dto: ProjectDTO) -> Result<Self> {
        let id = Id::<Project>::from_str(&dto.id)?;
        let name = &dto.name;
        let owner_id = match dto.owner_id {
            Some(ref owner_id_str) => Some(Id::<Person>::from_str(owner_id_str)?),
            None => None,
        };
        let description = match &dto.description {
            Some(desc) => desc,
            None => "",
        };
        let tags: Vec<Id<Tag>> = dto
            .tags
            .into_iter()
            .map(|id| {
                Id::from_str(&id)
                    .map_err(|_| Error::ProjectError(format!("Invalid project id: {id:?}")))
            })
            .collect::<Result<Vec<_>>>()?;
        let start_date = match dto.start_date {
            Some(date_string) => Some(date_string.parse::<DateTime<Utc>>().map_err(|_| {
                Error::ProjectError(format!("Invalid project start date: {date_string:?}"))
            })?),
            None => None,
        };
        let due_date = match dto.due_date {
            Some(date_string) => Some(date_string.parse::<DateTime<Utc>>().map_err(|_| {
                Error::ProjectError(format!("Invalid project due date: {date_string:?}"))
            })?),
            None => None,
        };
        let children: Vec<SchedulableItem> = dto
            .children
            .into_iter()
            .map(|child| child.try_into())
            .collect::<Result<Vec<_>>>()?;
        let dependencies: Vec<Id<Project>> = dto
            .dependencies
            .into_iter()
            .map(|id| {
                Id::from_str(&id)
                    .map_err(|_| Error::ProjectError(format!("Invalid project dependency: {id:?}")))
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(ProjectBuilder::new()
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

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SchedulableItemDTO {
    Project(String),
    Task(String),
}

impl From<SchedulableItem> for SchedulableItemDTO {
    fn from(element: SchedulableItem) -> Self {
        match element {
            SchedulableItem::Project(id) => SchedulableItemDTO::Project(id.to_string()),
            SchedulableItem::Task(id) => SchedulableItemDTO::Task(id.to_string()),
        }
    }
}

impl TryFrom<SchedulableItemDTO> for SchedulableItem {
    type Error = Error;

    fn try_from(element: SchedulableItemDTO) -> Result<Self> {
        match element {
            SchedulableItemDTO::Project(id) => {
                Ok(SchedulableItem::Project(Id::<Project>::from_str(&id)?))
            }
            SchedulableItemDTO::Task(id) => Ok(SchedulableItem::Task(Id::<Task>::from_str(&id)?)),
        }
    }
}

