use chrono::{DateTime, Utc};
use crate::{
    id::Id,
    models::{person::Person, tag::Tag, project::ProjectStatus},
    EntityType,
};

pub trait Schedulable {
    type IdType: EntityType;
    type ChildType;
    type DependencyType;

    // Core getters
    fn id(&self) -> Id<Self::IdType>;
    fn name(&self) -> &str;
    fn owner_id(&self) -> Option<&Id<Person>>;
    fn description(&self) -> &str;
    fn has_description(&self) -> bool;
    fn tags(&self) -> Vec<Id<Tag>>;
    fn has_tags(&self) -> bool;
    fn start_date(&self) -> Option<DateTime<Utc>>;
    fn has_start_date(&self) -> bool;
    fn due_date(&self) -> Option<DateTime<Utc>>;
    fn has_due_date(&self) -> bool;
    fn status(&self) -> ProjectStatus;
    fn children(&self) -> Vec<Self::ChildType>;
    fn has_children(&self) -> bool;
    fn dependencies(&self) -> Vec<Self::DependencyType>;
    fn has_dependencies(&self) -> bool;

    // Core mutators
    fn rename(&mut self, name: &str) -> &Self;
    fn transfer_ownership(&mut self, owner_id: Id<Person>) -> &Self;
    fn clear_description(&mut self) -> &Self;
    
    // Tag management
    fn add_tag(&mut self, tag_id: Id<Tag>) -> &Self;
    fn add_tags(&mut self, tags: Vec<Id<Tag>>) -> &Self;
    fn remove_tag(&mut self, tag: Id<Tag>) -> &Self;
    fn remove_all_tags(&mut self) -> &Self;
    
    // Date management
    fn start_now(&mut self) -> &Self;
    fn start_at_date(&mut self, start_date: DateTime<Utc>) -> &Self;
    fn remove_start_date(&mut self) -> &Self;
    fn set_due_date(&mut self, due_date: DateTime<Utc>) -> &Self;
    fn remove_due_date(&mut self) -> &Self;
    
    // Status management
    fn promote(&mut self) -> &Self;
    fn demote(&mut self) -> &Self;
    fn archive(&mut self) -> &Self;
    fn cancel(&mut self) -> &Self;
    
    // Validation
    fn is_valid_start_date(&self, start_date: Option<DateTime<Utc>>) -> bool;
    fn is_valid_due_date(&self, due_date: Option<DateTime<Utc>>) -> bool;
    fn is_valid_tag(&self, tag_id: &Id<Tag>) -> bool;
}