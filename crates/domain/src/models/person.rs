use core::fmt;
use crate::id::Id;
use crate::{EntityType,HasId};
use crate::builders::person_builder::PersonBuilder;

#[derive(Clone, PartialEq, Eq)]
pub struct Person {
    id: Id<Person>,
    first_name: String,
    last_name: String,
}

impl Person {
    pub fn from_builder(builder: PersonBuilder) -> Self {
        Person { 
            id: builder.id(), 
            first_name: builder.first_name(), 
            last_name: builder.last_name() 
        }
    }

    pub fn has_first_name(&self) -> bool {
        self.first_name != ""
    }

    pub fn first_name(&self) -> &str {
        self.first_name.as_str()
    }

    pub fn set_first_name(&mut self, first_name: &str) -> &Self {
        self.first_name = first_name.into();
        self
    }

    pub fn has_last_name(&self) -> bool {
        self.last_name != ""
    }

    pub fn last_name(&self) -> &str {
        self.last_name.as_str()
    }

    pub fn set_last_name(&mut self, last_name: &str) -> &Self {
        self.last_name = last_name.into();
        self
    }

    pub fn rename(&mut self, first_name: &str, last_name: &str) -> &Self {
        self.first_name = first_name.into();
        self.last_name = last_name.into();
        self
    }
}

impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} {}]({})", self.first_name, self.last_name, self.id)
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

