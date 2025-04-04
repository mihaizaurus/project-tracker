use core::fmt;
use crate::id::Id;
use crate::{EntityType,HasId};

#[derive(Clone, PartialEq, Eq)]
pub struct Person {
    id: Id<Person>,
    first_name: String,
    last_name: String,
}

impl Person {
    pub fn new(first_name: &str, last_name: &str) -> Person {
        Person {
            id: Id::<Person>::new(),
            first_name: first_name.into(),
            last_name: last_name.into(),
        }
    }
}

impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} {}]-[{}]", self.first_name, self.last_name, self.id)
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.first_name, self.last_name)
    }
}

impl EntityType for Person {
    fn prefix() -> &'static str {
        "person"
    }
}

impl HasId for Person {
    type Entity = Person;

    fn id(&self) -> Id<Person> {
        self.id.clone()
    }
}

