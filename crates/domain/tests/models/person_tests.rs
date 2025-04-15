use project_tracker_core::HasId;
use project_tracker_core::factories::person_factory::*;

#[test]
fn create_person() {
    let person = sample_person();
    assert!(person.has_first_name());
    assert!(person.has_last_name());
}

#[test]
fn create_tag_id() {
    let person = sample_person();
    let person_id = person.id().to_string();
    assert!(person_id.starts_with("person-"));
}

#[test]
fn set_first_name() {
    let mut person = sample_person();
    person.set_first_name("Alison");
    assert_eq!(person.first_name(),"Alison");
}

#[test]
fn set_last_name() {
    let mut person = sample_person();
    person.set_last_name("O'Neil");
    assert_eq!(person.last_name(),"O'Neil");
}

#[test]
fn rename() {
    let mut person = sample_person();
    person.rename("Alison", "O'Neil");
    assert_eq!(person.first_name(),"Alison");
    assert_eq!(person.last_name(),"O'Neil");
}