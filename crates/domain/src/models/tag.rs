use crate::id::Id;
use crate::{EntityType, HasId};
use crate::builders::tag_builder::TagBuilder;

#[derive(Clone, PartialEq, Eq)]
pub struct Tag {
    id: Id<Tag>,
    name: String,
    description: Option<String>,
    parents: Vec<Id<Tag>>,
}

impl Tag {
    pub fn from_builder(builder: TagBuilder) -> Self {
        Tag { 
            id: builder.id(),
            name: builder.name(), 
            description: builder.description(), 
            parents: builder.parents() 
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn rename(&mut self, name: &str) -> &Self {
        self.name = name.into();
        self
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

    pub fn add_parent(&mut self, tag_id: Id<Tag>) -> &Self {
        if self.is_valid_parent(&tag_id) {
            self.parents.push(tag_id);
        }
        self
    }

    pub fn add_parents(&mut self, tags: Vec<Id<Tag>>) -> &Self {
        for tag_id in tags {
            self.add_parent(tag_id);
        }
        self
    }

    pub fn remove_parent(&mut self, tag: Id<Tag>) -> &Self {
        let index = self.parents.iter().position(|t| t == &tag).unwrap();
        self.parents.remove(index);
        self
    }

    pub fn remove_parents(&mut self, tags: Vec<Id<Tag>>) -> &Self {
        if !tags.is_empty() {
            for tag in tags {
                let index = self.parents.iter().position(|t| t == &tag).unwrap();
                self.parents.remove(index);
            }
        }
        self
    }

    pub fn remove_all_parents(&mut self) -> &Self {
        self.parents.clear();
        self
    }

    pub fn is_valid_parent(&self, parent_to_validate: &Id<Tag>) -> bool {
        &self.id() != parent_to_validate
    }
}

impl EntityType for Tag {
    fn prefix() -> &'static str {
        "tag"
    }
}

impl HasId for Tag {
    type Entity = Tag;

    fn id(&self) -> Id<Tag> {
        self.id.clone()
    }
}