use crate::id::Id;
use crate::EntityType;

#[derive(Clone)]
pub struct Task {
    name: String,
    description: String,
    id: Id<Task>,
}

impl EntityType for Task {
    fn prefix() -> &'static str {
        "task"
    }
}