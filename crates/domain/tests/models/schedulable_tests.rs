use project_tracker_core::{
    models::{schedulable::Schedulable, project::Project, task::Task},
    builders::{project_builder::ProjectBuilder, task_builder::TaskBuilder},
    factories::{person_factory::basic_person, tag_factory::basic_tag},
    HasId,
};
use chrono::{Utc, Duration};

// Generic function that works with any Schedulable type
fn schedule_item<T: Schedulable>(item: &mut T, name: &str) {
    item.rename(name);
    item.start_now();
}

fn add_common_properties<T: Schedulable>(item: &mut T, _description: &str) {
    let user = basic_person();
    let tag = basic_tag();
    
    item.transfer_ownership(user.id());
    item.clear_description();
    item.add_tag(tag.id());
    item.promote();
}

#[test]
fn test_project_schedulable() {
    let mut project = Project::from_builder(ProjectBuilder::new().with_name("Test Project"));
    
    // Test generic function usage
    schedule_item(&mut project, "Renamed Project");
    assert_eq!(project.name(), "Renamed Project");
    assert!(project.has_start_date());
    
    // Test common properties function
    add_common_properties(&mut project, "A test project");
    assert!(project.has_tags());
    assert_eq!(project.status().to_string(), "Planned");
}

#[test] 
fn test_task_schedulable() {
    let mut task = Task::from_builder(TaskBuilder::new().with_name("Test Task"));
    
    // Test generic function usage
    schedule_item(&mut task, "Renamed Task");
    assert_eq!(task.name(), "Renamed Task");
    assert!(task.has_start_date());
    
    // Test common properties function
    add_common_properties(&mut task, "A test task");
    assert!(task.has_tags());
    assert_eq!(task.status().to_string(), "Planned");
}

#[test]
fn test_polymorphic_usage() {
    let mut project = Project::from_builder(ProjectBuilder::new().with_name("Project"));
    let mut task = Task::from_builder(TaskBuilder::new().with_name("Task"));
    
    // Test that we can use the same logic for both
    fn process_schedulable<T: Schedulable>(item: &mut T) {
        item.rename("Processed");
        item.start_now();
        item.promote();
    }
    
    process_schedulable(&mut project);
    process_schedulable(&mut task);
    
    assert_eq!(project.name(), "Processed");
    assert_eq!(task.name(), "Processed");
    assert!(project.has_start_date());
    assert!(task.has_start_date());
}

#[test]
fn test_validation_behavior() {
    let mut project = Project::from_builder(ProjectBuilder::new().with_name("Test"));
    let mut task = Task::from_builder(TaskBuilder::new().with_name("Test"));
    
    // Test date validation
    let future_date = Utc::now() + Duration::days(30);
    let _past_date = Utc::now() - Duration::days(30);
    
    // Both should handle validation the same way
    project.set_due_date(future_date);
    task.set_due_date(future_date);
    
    assert!(project.is_valid_start_date(Some(Utc::now())));
    assert!(task.is_valid_start_date(Some(Utc::now())));
    
    assert!(!project.is_valid_start_date(Some(future_date + Duration::days(1))));
    assert!(!task.is_valid_start_date(Some(future_date + Duration::days(1))));
}