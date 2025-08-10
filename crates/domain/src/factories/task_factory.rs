use chrono::{Duration, Utc};

use crate::HasId;
use crate::builders::task_builder::TaskBuilder;
use crate::factories::person_factory::*;
use crate::id::Id;
use crate::models::{schedulable::SchedulableItemStatus, task::Task};

use super::tag_factory::*;

pub fn basic_task() -> Task {
    TaskBuilder::new().build()
}

// region: Factories for Tests

pub fn sample_task_for_dto() -> Task {
    TaskBuilder::new()
        .with_name("This is a sample task")
        .with_owner_id(Some(sample_person().id()))
        .with_description("This is a sample DTO task")
        .with_start_date(Some(Utc::now()))
        .with_due_date(Some(Utc::now() + Duration::days(30)))
        .with_tags(sample_tags_list())
        .with_children(sample_tasks_list())
        .with_dependencies(sample_tasks_list())
        .build()
}

pub fn sample_task() -> Task {
    TaskBuilder::new()
        .with_name("This is a sample task")
        .with_description("This is a sample task")
        .build()
}

pub fn sample_tasks_list() -> Vec<Id<Task>> {
    let task_1 = sample_task();
    let task_2 = sample_task();
    let task_3 = sample_task();
    vec![task_1.id(), task_2.id(), task_3.id()]
}

pub fn sample_task_with_owner() -> Task {
    let owner = sample_person();
    TaskBuilder::new().with_owner_id(Some(owner.id())).build()
}

pub fn sample_task_with_tags() -> Task {
    let tags = sample_tags_list();
    TaskBuilder::new().with_tags(tags).build()
}

pub fn sample_task_with_due_date() -> Task {
    let due_date = Utc::now() + Duration::days(1);
    TaskBuilder::new().with_due_date(Some(due_date)).build()
}

pub fn sample_task_with_children() -> Task {
    let children = sample_tasks_list();
    TaskBuilder::new().with_children(children).build()
}

pub fn sample_task_with_dependencies() -> Task {
    let dependency = sample_task();
    TaskBuilder::new()
        .with_dependencies(vec![dependency.id()])
        .build()
}

pub fn sample_planned_task() -> Task {
    TaskBuilder::new()
        .with_status(SchedulableItemStatus::Planned)
        .build()
}

pub fn sample_in_progress_task() -> Task {
    TaskBuilder::new()
        .with_status(SchedulableItemStatus::InProgress)
        .build()
}

pub fn sample_in_review_task() -> Task {
    TaskBuilder::new()
        .with_status(SchedulableItemStatus::InReview)
        .build()
}

pub fn sample_completed_task() -> Task {
    TaskBuilder::new()
        .with_status(SchedulableItemStatus::Completed)
        .build()
}

pub fn sample_archived_task() -> Task {
    TaskBuilder::new()
        .with_status(SchedulableItemStatus::Archived)
        .build()
}

pub fn sample_canceled_task() -> Task {
    TaskBuilder::new()
        .with_status(SchedulableItemStatus::Canceled)
        .build()
}

pub fn sample_scheduled_task() -> Task {
    let start_date = Utc::now() + Duration::days(1);
    let due_date = Utc::now() + Duration::days(1);
    TaskBuilder::new()
        .with_start_date(Some(start_date))
        .with_due_date(Some(due_date))
        .build()
}

// endregion: Factories for Tests

