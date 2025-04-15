use crate::id::Id;
use crate::{EntityType,HasId};
use crate::models::person::Person;

#[derive(Clone, PartialEq, Eq)]
pub struct PersonBuilder {
    id: Id<Person>,
    first_name: String,
    last_name: String,
}

impl PersonBuilder {
    pub fn new() -> Self {
        PersonBuilder { 
            id: Id::<Person>::new(), 
            first_name: String::new(), 
            last_name: String::new()
        }
    }

    pub fn with_first_name(mut self, first_name: &str) -> Self {
        self.first_name = first_name.into();
        self
    }

    pub fn with_last_name(mut self, last_name: &str) -> Self {
        self.last_name = last_name.into();
        self
    }

    pub fn first_name(&self) -> String {
        self.first_name.clone()
    }

    pub fn last_name(&self) -> String {
        self.last_name.clone()
    }

    pub fn build(self) -> Person {
        Person::from_builder(self)
    }
}

impl EntityType for PersonBuilder {
    fn prefix() -> &'static str {
        "person"
    }
}

impl HasId for PersonBuilder {
    type Entity = Person;

    fn id(&self) -> Id<Person> {
        self.id.clone()
    }
}