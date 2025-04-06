use crate::id::Id;
use crate::{EntityType,HasId};
use crate::models::person::Person;
use crate::models::tag::Tag;
use crate::models::project::ProjectStatus;
use crate::builders::task_builder::TaskBuilder;

use log::{error, info};
use core::fmt;
use chrono::{DateTime, Datelike, Utc};

#[derive(Clone, PartialEq, Eq)]
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
            status: builder.status()
        }
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

    pub fn set_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn clear_description(&mut self) -> &Self {
        self.description = None;
        self
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