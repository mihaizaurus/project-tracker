use crate::id::Id;
use crate::{EntityType,HasId};

#[derive(Clone, PartialEq, Eq)]
pub struct Task {
    id: Id<Task>,
    name: String,
    description: Option<String>,
}

impl Task {
    pub fn new(name: &str) -> Self {
        Task {
            id: Id::<Task>::new(),
            name: name.into(),
            description: None,
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