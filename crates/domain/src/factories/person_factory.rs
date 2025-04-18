use crate::models::person::Person;
use crate::builders::person_builder::PersonBuilder;

pub fn basic_person() -> Person {
    PersonBuilder::new().build()
}

// region: Factories for Tests

pub fn sample_person() -> Person {
    PersonBuilder::new().with_first_name("Testy").with_last_name("McTest").build()
}

// endregion: Factories for Tests