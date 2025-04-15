use crate::models::person::Person;
use crate::builders::person_builder::PersonBuilder;

pub fn sample_person() -> Person {
    PersonBuilder::new().with_first_name("Testy").with_last_name("McTest").build()
}