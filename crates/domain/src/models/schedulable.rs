use crate::{
    EntityType,
    id::Id,
    models::{person::Person, project::ProjectStatus, project::ProjectSubElement, tag::Tag},
};
use chrono::{DateTime, Utc};

pub trait Schedulable {
    type IdType: EntityType;
    type ChildType;
    type DependencyType;

    // Core getters
    fn name(&self) -> &str;
    fn owner_id(&self) -> Option<&Id<Person>>;
    fn description(&self) -> &str;
    fn tags(&self) -> Vec<Id<Tag>>;
    fn start_date(&self) -> Option<DateTime<Utc>>;
    fn due_date(&self) -> Option<DateTime<Utc>>;
    fn status(&self) -> ProjectStatus;
    fn children(&self) -> Vec<Self::ChildType>;
    fn dependencies(&self) -> Vec<Self::DependencyType>;

    // Core validator methods
    fn has_owner(&self) -> bool;
    fn has_description(&self) -> bool;
    fn has_tags(&self) -> bool;
    fn has_start_date(&self) -> bool;
    fn has_due_date(&self) -> bool;
    fn has_child(&self, child_to_validate: &ProjectSubElement) -> bool;
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

    fn add_child(&mut self, child: ProjectSubElement) -> &Self;
    fn add_children(&mut self, children: Vec<ProjectSubElement>) -> &Self;
    fn remove_child(&mut self, child: ProjectSubElement) -> &Self;
    fn remove_children(&mut self, children: Vec<ProjectSubElement>) -> &Self;
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
    fn is_valid_child(&self, child_to_validate: &ProjectSubElement) -> bool;
}
