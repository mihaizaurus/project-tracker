use crate::id::Id;
use crate::{EntityType,HasId};
use crate::models::tag::Tag;

#[derive(Clone, PartialEq, Eq)]
pub struct TagBuilder {
    id: Id<Tag>,
    name: String,
    description: Option<String>,
    parents: Vec<Id<Tag>>,
}

impl TagBuilder {
    pub fn new() -> Self {
        TagBuilder {
            id: Id::<Tag>::new(),
            name: String::new(),
            description: Some(String::new()),
            parents: Vec::new(),
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.into();
        self
    }

    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.into());
        self
    }
    
    pub fn with_parents(mut self, parents: Vec<Id<Tag>>) -> Self {
        self.parents = parents;
        self
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }

    pub fn parents(&self) -> Vec<Id<Tag>> {
        self.parents.clone()
    }

    pub fn build(self) -> Tag {
        Tag::from_builder(self)
    }
}

impl EntityType for TagBuilder {
    fn prefix() -> &'static str {
        "tag"
    }
}

impl HasId for TagBuilder {
    type Entity = Tag;

    fn id(&self) -> Id<Tag> {
        self.id.clone()
    }
}