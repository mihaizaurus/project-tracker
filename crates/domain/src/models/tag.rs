use crate::id::Id;
use crate::{EntityType, HasId};

#[derive(Clone, PartialEq, Eq)]
pub struct Tag {
    id: Id<Tag>,
    name: String,
    description: Option<String>,
    parents: Vec<Id<Tag>>,
}

impl Tag {
    pub fn new(name: &str) -> Self {
        Tag {
            id: Id::<Tag>::new(),
            name: name.into(),
            description: None,
            parents: Vec::new()
        }
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