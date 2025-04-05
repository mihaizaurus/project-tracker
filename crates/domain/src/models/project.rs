use crate::id::Id;
use crate::{EntityType,HasId};
use super::task::Task;
use super::person::Person;
use super::tag::Tag;

use log::{info,warn,debug};
use core::fmt;
use chrono::{DateTime, Datelike, Utc};
// use serde::{Serialize, Deserialize};

/// Represents a Project in the Project Tracker App, consisting of some metadata, sub elements as well as displaying some dates and dependencies.
/// # Examples
/// ```
/// use project_tracker_core::models::project::Project;
/// 
/// let project_1 = Project::new("Building a cool new Project Tracking App in Rust");
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct Project {
    id: Id<Project>,
    name: String,
    owner_id: Option<Id<Person>>,
    description: Option<String>,
    tags: Vec<Id<Tag>>,
    start_date: Option<DateTime<Utc>>,
    due_date: Option<DateTime<Utc>>,
    children: Vec<ProjectSubElement>,
    dependencies: Vec<Id<Project>>,
    status: ProjectStatus,
}

impl Project {
    pub fn new(name: &str) -> Self {
        Project {
            id: Id::<Project>::new(),
            name: name.into(),
            owner_id: None,
            description: None,
            tags: Vec::new(),
            start_date: None,
            due_date: None,
            children: Vec::new(),
            dependencies: Vec::new(),
            status: ProjectStatus::NotStarted
        }
    }

    pub fn name(&self) -> &str {
        &self.name.as_str()
    }

    pub fn has_owner(&self) -> bool {
        self.owner_id.is_some()
    }

    pub fn owner_id(&self) -> Option<&Id<Person>> {
        self.owner_id.as_ref()
    }

    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }

    pub fn description(&self) -> &str {
        if let Some(description) = &self.description {
            description.as_str()
        } else {
            ""
        }
    }

    pub fn has_tags(&self) -> bool {
        self.tags().len() > 0
    }

    pub fn tags(&self) -> Vec<Id<Tag>> {
        self.tags.clone()
    }

    pub fn has_start_date(&self) -> bool {
        self.start_date.is_some()
    }

    pub fn start_date(&self) -> Option<DateTime<Utc>> {
        self.start_date
    }

    pub fn has_due_date(&self) -> bool {
        self.due_date.is_some()
    }

    pub fn due_date(&self) -> Option<DateTime<Utc>> {
        self.due_date
    }

    pub fn rename(&mut self, name: &str) -> &Self {
        self.name = name.into();
        self
    }

    pub fn set_owner(mut self, owner_id: Id<Person>) -> Self {
        self.owner_id = Some(owner_id);
        self
    }

    pub fn transfer_ownership(&mut self, owner_id: Id<Person>) -> &Self {
        self.owner_id = Some(owner_id);
        self
    }

    pub fn set_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn clear_description(&mut self) -> &Self {
        self.description = None;
        self
    }

    pub fn add_tag(mut self, tag: Id<Tag>) -> Self {
        self.tags.push(tag);
        self
    }

    pub fn add_tags(mut self, mut tags: Vec<Id<Tag>>) -> Self {
        self.tags.append(&mut tags);
        self
    }

    pub fn remove_tag(&mut self, tag: Id<Tag>) -> &Self {
        let index = self.tags.iter().position(|t| t == &tag).unwrap();
        self.tags.remove(index);
        self
    }

    pub fn remove_tags(&mut self, tags: Vec<Id<Tag>>) -> &Self {
        if !tags.is_empty() {
            for tag in tags {
                let index = self.tags.iter().position(|t| t == &tag).unwrap();
                self.tags.remove(index);
            }
        }
        self
    }

    pub fn remove_all_tags(&mut self) -> &Self {
        self.tags.clear();
        self
    }

    pub fn start_now(mut self) -> Self {
        self.start_date = Some(Utc::now());
        self
    }

    pub fn start_at_date(mut self, start_date: DateTime<Utc>) -> Self {
        self.start_date = Some(start_date);
        self
    }

    pub fn remove_start_date(&mut self) -> &Self {
        self.start_date = None;
        self
    }

    pub fn set_due_date(&mut self, due_date: DateTime<Utc>) -> &Self {
        self.due_date = Some(due_date);
        self
    }

    pub fn remove_due_date(&mut self) -> &Self {
        self.due_date = None;
        self
    }

    pub fn has_children(&self) -> bool {
        self.children.len() > 0
    }

    pub fn children(&self) -> Vec<ProjectSubElement> {
        self.children.clone()
    }

    pub fn project_children(&self) -> Vec<Id<Project>> {
        let mut child_projects: Vec<Id<Project>> = Vec::new();
        for element in self.children.clone() {
            match element {
                ProjectSubElement::Project(id) => {
                    child_projects.push(id);
                },
                _ => {}
            }
        }
        child_projects
    }

    pub fn task_children(&self) -> Vec<Id<Task>> {
        let mut child_tasks: Vec<Id<Task>> = Vec::new();
            for element in self.children.clone() {
                match element {
                    ProjectSubElement::Task(id) => {
                        child_tasks.push(id);
                    },
                    _ => {}
                }
            }
            child_tasks
    }

    pub fn add_child(&mut self,child: ProjectSubElement) -> &Self {
        self.children.push(child);
        self
    }

    pub fn add_children(&mut self,mut children: Vec<ProjectSubElement>) -> &Self {
        self.children.append(&mut children);
        self
    }

    pub fn remove_child(&mut self,child: ProjectSubElement) -> &Self {
        let index = self.children.iter().position(|t| t == &child).unwrap();
        self.children.remove(index);
        self
    }

    pub fn remove_children(&mut self,children: Vec<ProjectSubElement>) -> &Self {
        if !children.is_empty() {
            for child in children {
                let index = self.children.iter().position(|t| t == &child).unwrap();
                self.children.remove(index);
            }
        }
        self
    }

    pub fn remove_all_children(&mut self) -> &Self {
        self.children.clear();
        self
    }

    pub fn add_dependency(mut self, project_id: Id<Project>) -> Self {
        self.dependencies.push(project_id);
        self
    }

    pub fn add_dependencies(mut self, mut project_ids: Vec<Id<Project>>) -> Self {
        self.dependencies.append(&mut project_ids);
        self
    }

    pub fn status(&self) -> ProjectStatus {
        self.status.clone()
    }

    pub fn promote(&mut self) -> &Self {
        match self.status {
            ProjectStatus::NotStarted => self.status = ProjectStatus::Planned,
            ProjectStatus::Planned => self.status = ProjectStatus::InProgress,
            ProjectStatus::InProgress => self.status = ProjectStatus::InReview,
            ProjectStatus::InReview => self.status = ProjectStatus::Completed,
            _ => (),
        }
        self
    }

    pub fn demote(&mut self) -> &Self {
        match self.status {
            ProjectStatus::InReview => self.status = ProjectStatus::InProgress,
            ProjectStatus::InProgress => self.status = ProjectStatus::Planned,
            ProjectStatus::Planned => self.status = ProjectStatus::NotStarted,
            _ => (),
        }
        self
    }

    pub fn archive(&mut self) -> &Self {
        self.status = ProjectStatus::Archived;
        self
    }

    pub fn cancel(&mut self) -> &Self {
        match self.status {
            ProjectStatus::Archived => {
                warn!("Project is already archived and cannot be canceled");
            },
            ProjectStatus::Completed => {
                warn!("Project is already completed and cannot be canceled");
            },
            _ => {self.status = ProjectStatus::Canceled;}
        }
        self
    }

}

impl fmt::Debug for Project {
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
            let year = start_date.year();
            let month = start_date.month();
            let day = start_date.day();
            let week = start_date.iso_week().week();
            writeln!(f, "- Project starts on: {}-{}-{} [Week {}]",day, month, year, week)?;
        } else {
            writeln!(f, "! No start date defined")?;
        }
        if let Some(due_date) = &self.due_date {
            let year = due_date.year();
            let month = due_date.month();
            let day = due_date.day();
            let week = due_date.iso_week().week();
            writeln!(f, "- Project is due on: {}-{}-{} [Week {}]",day, month, year, week)?;
        } else {
            writeln!(f, "! No due date defined")?;
        }
        writeln!(f, "- Project has {} children",self.children.len())?;
        {
            let child_projects: Vec<Id<Project>> = self.project_children();
            writeln!(f, "-- {}/{} children are projects",child_projects.len(), self.children.len())?;
            let child_tasks: Vec<Id<Task>> = self.task_children();
            writeln!(f, "-- {}/{} children are tasks",child_tasks.len(), self.children.len())?;
        }
        Ok(())
    }
}

impl fmt::Display for Project {
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
            let year = start_date.year();
            let month = start_date.month();
            let day = start_date.day();
            let week = start_date.iso_week().week();
            writeln!(f, "- Project starts on: {}-{}-{} [Week {}]",day, month, year, week)?;
        }
        if let Some(due_date) = &self.due_date {
            let year = due_date.year();
            let month = due_date.month();
            let day = due_date.day();
            let week = due_date.iso_week().week();
            writeln!(f, "- Project is due on: {}-{}-{} [Week {}]",day, month, year, week)?;
        }
        writeln!(f, "- Project has {} children",self.children.len())?;
        {
            let child_projects: Vec<Id<Project>> = self.project_children();
            if child_projects.len() > 0 {
                writeln!(f, "-- {}/{} children are projects",child_projects.len(), self.children.len())?;
            }
            let child_tasks: Vec<Id<Task>> = self.task_children();
            if child_tasks.len() > 0 {
                writeln!(f, "-- {}/{} children are tasks",child_tasks.len(), self.children.len())?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum ProjectStatus {
    NotStarted,
    Planned,
    InProgress,
    InReview,
    Completed,
    Archived,
    Canceled,
}

#[derive(Clone, PartialEq, Eq)]
pub enum ProjectSubElement {
    Project(Id<Project>),
    Task(Id<Task>)
}

impl EntityType for Project {
    fn prefix() -> &'static str {
        "project"
    }
}

impl HasId for Project {
    type Entity = Project;

    fn id(&self) -> Id<Project> {
        self.id.clone()
    }
}