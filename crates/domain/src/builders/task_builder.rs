use crate::id::Id;
use crate::{EntityType,HasId};
use crate::models::person::Person;
use crate::models::project::ProjectStatus;
use crate::models::tag::Tag;
use crate::models::task::Task;

use chrono::{DateTime, Utc};

#[derive(Clone, PartialEq, Eq)]
pub struct TaskBuilder {
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

impl TaskBuilder {
    pub fn new() -> Self {
        TaskBuilder {
            id: Id::<Task>::new(),
            name: String::new(),
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

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.into();
        self
    }

    pub fn with_owner_id(mut self, owner_id: Id<Person>) -> Self {
        self.owner_id = Some(owner_id);
        self
    }

    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_tags(mut self, mut tags: Vec<Id<Tag>>) -> Self {
        self.tags.append(&mut tags);
        self
    }

    pub fn with_star_date(mut self, start_date: DateTime<Utc>) -> Self {
        self.start_date = Some(start_date);
        self
    }

    pub fn with_due_date(mut self, due_date: DateTime<Utc>) -> Self {
        self.due_date = Some(due_date);
        self
    }

    pub fn with_children(mut self, children: Vec<Id<Task>>) -> Self {
        self.children = children;
        self
    }

    pub fn with_dependencies(mut self, dependencies: Vec<Id<Task>>) -> Self {
        self.dependencies = dependencies;
        self
    }

    pub fn with_status(mut self, status: ProjectStatus) -> Self {
        self.status = status;
        self
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn owner_id(&self) -> Option<Id<Person>> {
        self.owner_id.clone()
    }

    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }

    pub fn tags(&self) -> Vec<Id<Tag>> {
        self.tags.clone()
    }

    pub fn start_date(&self) -> Option<DateTime<Utc>> {
        self.start_date.clone()
    }

    pub fn due_date(&self) -> Option<DateTime<Utc>> {
        self.due_date.clone()
    }

    pub fn children(&self) -> Vec<Id<Task>> {
        self.children.clone()
    }

    pub fn dependencies(&self) -> Vec<Id<Task>> {
        self.dependencies.clone()
    }

    pub fn status(&self) -> ProjectStatus {
        self.status.clone()
    }

    pub fn build(self) -> Task {
        Task::from_builder(self)
    }
}

impl EntityType for TaskBuilder {
    fn prefix() -> &'static str {
        "task"
    }
}

impl HasId for TaskBuilder {
    type Entity = Task;

    fn id(&self) -> Id<Task> {
        self.id.clone()
    }
}