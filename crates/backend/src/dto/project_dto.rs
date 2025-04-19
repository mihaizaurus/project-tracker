// use std::str::FromStr;

// use chrono::{DateTime, Utc};
use project_tracker_core::{
    models::project::{Project,ProjectStatus,ProjectSubElement},
    HasId
};
use serde::{Deserialize, Serialize};
use core::fmt;

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ProjectDTO {
    id: String,
    name: String,
    owner_id: Option<String>,
    description: Option<String>,
    tags: Vec<String>,
    start_date: Option<String>,
    due_date: Option<String>,
    children: Vec<ProjectSubElementDTO>,
    dependencies: Vec<String>,
    status: ProjectStatus,
}

impl From<Project> for ProjectDTO {
    fn from(project: Project) -> Self {
        Self {
            id: project.id().to_string(),
            name: project.name().to_string(),
            owner_id: project.owner_id().map(|id| id.to_string()),
            description: project.description().to_string().clone().into(),
            tags: project.tags().into_iter().map(|id| id.to_string()).collect(),
            start_date: project.start_date().map(|date| date.to_rfc3339()),
            due_date: project.due_date().map(|date| date.to_rfc3339()),
            children: project.children().into_iter().map(|child| child.into()).collect(),
            dependencies: project.dependencies().into_iter().map(|id| id.to_string()).collect(),
            status: project.status(),
        }
    }
}

impl fmt::Debug for ProjectDTO {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Project [[{}]]",self.name)?;
        writeln!(f, "- Project Id:{:?}",self.id)?;
        if let Some(description) = &self.description {
            writeln!(f, "- Project Description: {}",description)?;
        } else {
            writeln!(f, "! No description provided")?;
        }
        if let Some(owner_id) = &self.owner_id {
            writeln!(f, "- Project Owner: {:?}",owner_id)?;
        } else {
            writeln!(f, "! No project owner")?;
        }
        if let Some(start_date) = &self.start_date {
            writeln!(f, "- Project starts on: {}",start_date)?;
        } else {
            writeln!(f, "! No start date defined")?;
        }
        if let Some(due_date) = &self.due_date {
            writeln!(f, "- Project is due on: {}",due_date)?;
        } else {
            writeln!(f, "! No due date defined")?;
        }
        writeln!(f, "- Project has {} children",self.children.len())?;
        writeln!(f, "- Project has {} dependencies",self.dependencies.len())?;
        writeln!(f, "- Project has {} tags",self.tags.len())?;
        Ok(())
    }
}

impl fmt::Display for ProjectDTO {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[[{}]]",self.name)?;
        writeln!(f, "- Project Id: {}",self.id)?;
        if let Some(description) = &self.description {
            writeln!(f, "- Project Description: {}",description)?;
        }
        if let Some(owner_id) = &self.owner_id {
            writeln!(f, "- Project Owner: {}",owner_id)?;
        }
        if let Some(start_date) = &self.start_date {
            writeln!(f, "- Project starts on: {}",start_date)?;
        }
        if let Some(due_date) = &self.due_date {
            writeln!(f, "- Project is due on: {}",due_date)?;
        }
        writeln!(f, "- Project has {} children",self.children.len())?;
        writeln!(f, "- Project has {} dependencies",self.dependencies.len())?;
        writeln!(f, "- Project has {} tags",self.tags.len())?;
        Ok(())
    }
}

// impl TryFrom<ProjectDTO> for Project {
//     type Error = String; //temporary, should be replaced with better errors.

//     fn try_from(dto: ProjectDTO) -> Result<Self, Self::Error> {
//         Ok(ProjectBuilder::new()
//             .with_name(&dto.name)
//             .with_owner_id(dto.owner_id.as_ref().map(|id| Id::from_str(id.as_str())).transpose().unwrap_or(None))
//             .with_description(dto.description.unwrap_or_default().as_str())
//             .with_tags(dto.tags.into_iter().map(|id| Id::from_str(id.as_str())).collect::<Result<Vec<_>,_>>().unwrap())
//             .with_star_date(dto.start_date.map(|d| d.parse::<DateTime<Utc>>()).transpose().unwrap().unwrap())
//             // .with_due_date(due_date)
//             // .with_children(children)
//             // .with_dependencies(dependencies)
//             .with_status(dto.status)
//             .build())
//     }
// }

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProjectSubElementDTO {
    Project(String),
    Task(String)
}

impl From<ProjectSubElement> for ProjectSubElementDTO {
    fn from(element: ProjectSubElement) -> Self {
        match element {
            ProjectSubElement::Project(id) => ProjectSubElementDTO::Project(id.to_string()),
            ProjectSubElement::Task(id) => ProjectSubElementDTO::Task(id.to_string())
        }
    }
}