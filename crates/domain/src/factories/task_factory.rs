use chrono::{Utc,Duration};

use crate::models::project::ProjectStatus;
use crate::models::person::Person;
use crate::models::task::Task;
use crate::models::tag::Tag;
use crate::builders::task_builder::TaskBuilder;
use crate::HasId;

pub fn sample_task() -> Task {
    TaskBuilder::new().with_name("This is a sample task").build()
}

pub fn sample_task_with_owner() -> Task {
    let owner = Person::new("Sample", "Owner");
    TaskBuilder::new().with_owner_id(owner.id()).build()
}

pub fn sample_task_with_tags() -> Task {
    let tag_1 = Tag::new("TestTag1");
    let tag_2 = Tag::new("TestTag2");
    let tag_3 = Tag::new("TestTag3");
    let tags = vec![tag_1.id(), tag_2.id(), tag_3.id()];
    TaskBuilder::new().with_tags(tags).build()
}

pub fn sample_task_with_due_date() -> Task {
    let due_date = Utc::now() + Duration::days(1);
    TaskBuilder::new().with_due_date(due_date).build()
}

pub fn sample_task_with_children() -> Task {

    let child_task_1 = sample_task();
    let child_task_2 = sample_task();
    let child_task_3 = sample_task();
    let children = vec![
        child_task_1.id(),
        child_task_2.id(),
        child_task_3.id()
    ];
    TaskBuilder::new().with_children(children).build()
}

pub fn sample_task_with_dependencies() -> Task {
    let dependency = sample_task();
    TaskBuilder::new().with_dependencies(vec![dependency.id()]).build()
}

pub fn sample_planned_task() -> Task {
    TaskBuilder::new().with_status(ProjectStatus::Planned).build()
}

pub fn sample_in_progress_task() -> Task {
    TaskBuilder::new().with_status(ProjectStatus::InProgress).build()
}

pub fn sample_in_review_task() -> Task {
    TaskBuilder::new().with_status(ProjectStatus::InReview).build()
}

pub fn sample_completed_task() -> Task {
    TaskBuilder::new().with_status(ProjectStatus::Completed).build()
}

pub fn sample_archived_task() -> Task {
    TaskBuilder::new().with_status(ProjectStatus::Archived).build()
}

pub fn sample_canceled_task() -> Task {
    TaskBuilder::new().with_status(ProjectStatus::Canceled).build()
}

pub fn sample_scheduled_task() -> Task {
    let start_date = Utc::now() + Duration::days(1);
    let due_date = Utc::now() + Duration::days(1);
    TaskBuilder::new().with_star_date(start_date).with_due_date(due_date).build()
}