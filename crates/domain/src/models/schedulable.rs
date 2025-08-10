use crate::{
    id::Id,
    models::{person::Person, project::Project, tag::Tag, task::Task},
};
use chrono::{DateTime, Utc};
use core::fmt;
use serde::{Deserialize, Serialize};

pub trait Schedulable {
    type ChildType;
    type DependencyType;

    // Core getters
    fn name(&self) -> &str;
    fn owner_id(&self) -> Option<&Id<Person>>;
    fn description(&self) -> &str;
    fn tags(&self) -> Vec<Id<Tag>>;
    fn start_date(&self) -> Option<DateTime<Utc>>;
    fn due_date(&self) -> Option<DateTime<Utc>>;
    fn status(&self) -> SchedulableItemStatus;
    fn children(&self) -> Vec<Self::ChildType>;
    fn dependencies(&self) -> Vec<Self::DependencyType>;

    // Core validator methods
    fn has_owner(&self) -> bool;
    fn has_description(&self) -> bool;
    fn has_tags(&self) -> bool;
    fn has_start_date(&self) -> bool;
    fn has_due_date(&self) -> bool;
    fn has_child(&self, child_to_validate: &SchedulableItem) -> bool;
    fn has_children(&self) -> bool;
    fn has_dependencies(&self) -> bool;

    // Core mutators
    fn rename(&mut self, name: &str) -> &Self;
    fn transfer_ownership(&mut self, owner_id: Id<Person>) -> &Self;

    fn set_description(&mut self, description: impl Into<String>) -> &Self;
    fn clear_description(&mut self) -> &Self;

    fn add_tag(&mut self, tag_id: Id<Tag>) -> &Self;
    fn add_tags(&mut self, tags: Vec<Id<Tag>>) -> &Self;
    fn remove_tag(&mut self, tag: Id<Tag>) -> &Self;
    fn remove_tags(&mut self, tags: Vec<Id<Tag>>) -> &Self;
    fn remove_all_tags(&mut self) -> &Self;

    fn start(&mut self) -> &Self;
    fn start_at_date(&mut self, start_date: DateTime<Utc>) -> &Self;
    fn remove_start_date(&mut self) -> &Self;
    fn set_due_date(&mut self, due_date: DateTime<Utc>) -> &Self;
    fn remove_due_date(&mut self) -> &Self;

    fn add_child(&mut self, child: SchedulableItem) -> &Self;
    fn add_children(&mut self, children: Vec<SchedulableItem>) -> &Self;
    fn remove_child(&mut self, child: SchedulableItem) -> &Self;
    fn remove_children(&mut self, children: Vec<SchedulableItem>) -> &Self;
    fn remove_all_children(&mut self) -> &Self;

    fn remove_all_dependencies(&mut self) -> &Self;

    fn promote(&mut self) -> &Self;
    fn demote(&mut self) -> &Self;
    fn archive(&mut self) -> &Self;
    fn cancel(&mut self) -> &Self;

    // Validation
    fn is_valid_tag(&self, tag_id: &Id<Tag>) -> bool;
    fn is_valid_start_date(&self, start_date: Option<DateTime<Utc>>) -> bool;
    fn is_valid_due_date(&self, due_date: Option<DateTime<Utc>>) -> bool;
    fn is_valid_child(&self, child_to_validate: &SchedulableItem) -> bool;
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SchedulableItem {
    Project(Id<Project>),
    Task(Id<Task>),
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum SchedulableItemStatus {
    NotStarted,
    Planned,
    InProgress,
    InReview,
    Completed,
    Archived,
    Canceled,
}

impl fmt::Display for SchedulableItemStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SchedulableItemStatus::NotStarted => write!(f, "NotStarted"),
            SchedulableItemStatus::Planned => write!(f, "Planned"),
            SchedulableItemStatus::InProgress => write!(f, "InProgress"),
            SchedulableItemStatus::InReview => write!(f, "InReview"),
            SchedulableItemStatus::Completed => write!(f, "Completed"),
            SchedulableItemStatus::Archived => write!(f, "Archived"),
            SchedulableItemStatus::Canceled => write!(f, "Canceled"),
        }
    }
}
