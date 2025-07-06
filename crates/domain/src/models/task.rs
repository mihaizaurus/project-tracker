use crate::builders::task_builder::TaskBuilder;
use crate::id::Id;
use crate::models::person::Person;
use crate::models::project::{ProjectStatus, ProjectSubElement};
use crate::models::schedulable::Schedulable;
use crate::models::tag::Tag;
use crate::{EntityType, HasId};

use chrono::{DateTime, Datelike, Utc};
use core::fmt;
use log::{error, info};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    id: Id<Task>,
    name: String,
    owner_id: Option<Id<Person>>,
    description: Option<String>,
    tags: Vec<Id<Tag>>,
    start_date: Option<DateTime<Utc>>,
    due_date: Option<DateTime<Utc>>,
    children: Vec<Id<Task>>,
    dependencies: Vec<Id<Task>>,
    status: ProjectStatus,
}

impl Task {
    pub fn from_builder(builder: TaskBuilder) -> Self {
        Task {
            id: builder.id(),
            name: builder.name(),
            owner_id: builder.owner_id(),
            description: builder.description(),
            tags: builder.tags(),
            start_date: builder.start_date(),
            due_date: builder.due_date(),
            children: builder.children(),
            dependencies: builder.dependencies(),
            status: builder.status(),
        }
    }

    pub fn has_dependency(&self, dependency_to_validate: &Id<Task>) -> bool {
        self.dependencies.contains(dependency_to_validate)
    }

    pub fn add_dependency(&mut self, project_id: Id<Task>) -> &Self {
        if self.is_valid_dependency(&project_id) {
            self.dependencies.push(project_id);
        }
        self
    }

    pub fn add_dependencies(&mut self, project_ids: Vec<Id<Task>>) -> &Self {
        for project_id in project_ids {
            self.add_dependency(project_id);
        }
        self
    }

    pub fn remove_dependency(&mut self, dependency_project_id: Id<Task>) -> &Self {
        let index = self
            .dependencies
            .iter()
            .position(|t| t == &dependency_project_id)
            .unwrap();
        self.dependencies.remove(index);
        self
    }

    pub fn remove_dependencies(&mut self, dependency_project_ids: Vec<Id<Task>>) -> &Self {
        if !dependency_project_ids.is_empty() {
            for dependency_project_id in dependency_project_ids {
                self.remove_dependency(dependency_project_id);
            }
        }
        self
    }

    /* ### Validation Methods */
    pub fn is_valid_dependency(&self, dependency_project_id: &Id<Task>) -> bool {
        dependency_project_id != &HasId::id(self)
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[[{}]]", self.name)?;
        writeln!(f, "- Task Id: {}", self.id)?;
        if let Some(description) = &self.description {
            writeln!(f, "- Task Description: {description}")?;
        }
        if let Some(owner_id) = &self.owner_id {
            writeln!(f, "- Task Owner: {owner_id}")?;
        }
        if let Some(start_date) = &self.start_date {
            let year = start_date.year();
            let month = start_date.month();
            let day = start_date.day();
            let week = start_date.iso_week().week();
            writeln!(f, "- Task starts on: {day}-{month}-{year} [Week {week}]")?;
        }
        if let Some(due_date) = &self.due_date {
            let year = due_date.year();
            let month = due_date.month();
            let day = due_date.day();
            let week = due_date.iso_week().week();
            writeln!(f, "- Task is due on: {day}-{month}-{year} [Week {week}]")?;
        }
        writeln!(f, "- Task has {} children", self.children.len())?;
        writeln!(f, "- Task has {} dependencies", self.dependencies.len())?;
        Ok(())
    }
}

impl EntityType for Task {
    fn prefix() -> &'static str {
        "task"
    }
}

impl HasId for Task {
    type Entity = Task;

    fn id(&self) -> Id<Task> {
        self.id.clone()
    }
}

impl Schedulable for Task {
    type IdType = Task;
    type ChildType = Id<Task>;
    type DependencyType = Id<Task>;

    // region: Core Getters

    fn name(&self) -> &str {
        &self.name
    }

    fn owner_id(&self) -> Option<&Id<Person>> {
        self.owner_id.as_ref()
    }

    fn description(&self) -> &str {
        if let Some(description) = &self.description {
            description.as_str()
        } else {
            ""
        }
    }

    fn tags(&self) -> Vec<Id<Tag>> {
        self.tags.clone()
    }

    fn start_date(&self) -> Option<DateTime<Utc>> {
        self.start_date
    }

    fn due_date(&self) -> Option<DateTime<Utc>> {
        self.due_date
    }

    fn status(&self) -> ProjectStatus {
        self.status.clone()
    }

    fn children(&self) -> Vec<Self::ChildType> {
        self.children.clone()
    }

    fn dependencies(&self) -> Vec<Self::DependencyType> {
        self.dependencies.clone()
    }
    // endregion: Core Getters

    // region: Core Validator Methods

    fn has_owner(&self) -> bool {
        self.owner_id.is_some()
    }

    fn has_description(&self) -> bool {
        self.description.is_some()
    }

    fn has_tags(&self) -> bool {
        !self.tags.is_empty()
    }

    fn has_start_date(&self) -> bool {
        self.start_date.is_some()
    }

    fn has_due_date(&self) -> bool {
        self.due_date.is_some()
    }

    fn has_child(&self, child_to_validate: &ProjectSubElement) -> bool {
        if let ProjectSubElement::Task(task_id) = child_to_validate {
            self.children.contains(task_id)
        } else {
            false
        }
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn has_dependencies(&self) -> bool {
        !self.dependencies.is_empty()
    }
    // endregion: Core Validator Methods

    // region: Core Mutators

    fn rename(&mut self, name: &str) -> &Self {
        self.name = name.into();
        self
    }

    fn transfer_ownership(&mut self, owner_id: Id<Person>) -> &Self {
        self.owner_id = Some(owner_id);
        self
    }

    fn set_description(&mut self, description: impl Into<String>) -> &Self {
        self.description = Some(description.into());
        self
    }

    fn clear_description(&mut self) -> &Self {
        self.description = None;
        self
    }

    fn add_tag(&mut self, tag_id: Id<Tag>) -> &Self {
        if self.is_valid_tag(&tag_id) {
            self.tags.push(tag_id);
        }
        self
    }

    fn add_tags(&mut self, tags: Vec<Id<Tag>>) -> &Self {
        for tag_id in tags {
            self.add_tag(tag_id);
        }
        self
    }

    fn remove_tag(&mut self, tag: Id<Tag>) -> &Self {
        if let Some(index) = self.tags.iter().position(|t| t == &tag) {
            self.tags.remove(index);
        }
        self
    }

    fn remove_tags(&mut self, tags: Vec<Id<Tag>>) -> &Self {
        if !tags.is_empty() {
            for tag in tags {
                let index = self.tags.iter().position(|t| t == &tag).unwrap();
                self.tags.remove(index);
            }
        }
        self
    }

    fn remove_all_tags(&mut self) -> &Self {
        self.tags.clear();
        self
    }

    fn start(&mut self) -> &Self {
        self.start_at_date(Utc::now());
        self
    }

    fn start_at_date(&mut self, start_date: DateTime<Utc>) -> &Self {
        if self.is_valid_start_date(Some(start_date)) {
            self.start_date = Some(start_date);
        } else {
            error!("Provided start date ({start_date}) is invalid.");
        }
        self
    }

    fn remove_start_date(&mut self) -> &Self {
        self.start_date = None;
        self
    }

    fn set_due_date(&mut self, due_date: DateTime<Utc>) -> &Self {
        if self.is_valid_due_date(Some(due_date)) {
            self.due_date = Some(due_date);
        }
        self
    }

    fn remove_due_date(&mut self) -> &Self {
        self.due_date = None;
        self
    }

    fn add_child(&mut self, child: ProjectSubElement) -> &Self {
        if self.is_valid_child(&child) {
            if let ProjectSubElement::Task(task_id) = child {
                self.children.push(task_id);
            }
        }
        self
    }

    fn add_children(&mut self, children: Vec<ProjectSubElement>) -> &Self {
        for child in children {
            self.add_child(child);
        }
        self
    }

    fn remove_child(&mut self, child: ProjectSubElement) -> &Self {
        if let ProjectSubElement::Task(task_id) = child {
            let index = self.children.iter().position(|t| t == &task_id).unwrap();
            self.children.remove(index);
        }
        self
    }

    fn remove_children(&mut self, children: Vec<ProjectSubElement>) -> &Self {
        if !children.is_empty() {
            for child in children {
                self.remove_child(child);
            }
        }
        self
    }

    fn remove_all_children(&mut self) -> &Self {
        self.children.clear();
        self
    }

    fn remove_all_dependencies(&mut self) -> &Self {
        self.dependencies.clear();
        self
    }

    fn promote(&mut self) -> &Self {
        match self.status {
            ProjectStatus::NotStarted => self.status = ProjectStatus::Planned,
            ProjectStatus::Planned => self.status = ProjectStatus::InProgress,
            ProjectStatus::InProgress => self.status = ProjectStatus::InReview,
            ProjectStatus::InReview => self.status = ProjectStatus::Completed,
            _ => (),
        }
        self
    }

    fn demote(&mut self) -> &Self {
        match self.status {
            ProjectStatus::InReview => self.status = ProjectStatus::InProgress,
            ProjectStatus::InProgress => self.status = ProjectStatus::Planned,
            ProjectStatus::Planned => self.status = ProjectStatus::NotStarted,
            _ => (),
        }
        self
    }

    fn archive(&mut self) -> &Self {
        if self.status != ProjectStatus::Archived {
            self.status = ProjectStatus::Archived;
        }
        self
    }

    fn cancel(&mut self) -> &Self {
        match self.status {
            ProjectStatus::Archived => {
                info!("Task is already archived and cannot be canceled");
            }
            ProjectStatus::Completed => {
                info!("Task is already completed and cannot be canceled");
            }
            _ => {
                self.status = ProjectStatus::Canceled;
            }
        }
        self
    }

    // endregion: Core Mutators

    fn is_valid_tag(&self, tag_id: &Id<Tag>) -> bool {
        !self.tags().contains(tag_id)
    }

    fn is_valid_start_date(&self, start_date: Option<DateTime<Utc>>) -> bool {
        match (start_date, self.due_date()) {
            (Some(start), Some(due)) => start <= due,
            _ => true,
        }
    }

    fn is_valid_due_date(&self, due_date: Option<DateTime<Utc>>) -> bool {
        match (self.start_date(), due_date) {
            (Some(start), Some(due)) => start <= due,
            (_, Some(due)) => due >= Utc::now(),
            _ => true,
        }
    }

    fn is_valid_child(&self, child_to_validate: &ProjectSubElement) -> bool {
        if let ProjectSubElement::Task(task_id) = child_to_validate {
            &HasId::id(self) != task_id
        } else {
            false
        }
    }
}
