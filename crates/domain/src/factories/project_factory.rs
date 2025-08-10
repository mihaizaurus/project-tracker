use chrono::{Duration, Utc};

use crate::HasId;
use crate::builders::project_builder::ProjectBuilder;
use crate::factories::{tag_factory::*, task_factory::*};
use crate::models::project::Project;
use crate::models::schedulable::{SchedulableItem, SchedulableItemStatus};

use super::person_factory::sample_person;

/// Basic **Project** with default values
pub fn basic_project() -> Project {
    ProjectBuilder::new().build()
}

// region: Factories for Tests
pub fn sample_project_for_dto() -> Project {
    ProjectBuilder::new()
        .with_name("DTO Project")
        .with_owner_id(Some(sample_person().id()))
        .with_description("This is a sample DTO project")
        .with_start_date(Some(Utc::now()))
        .with_due_date(Some(Utc::now() + Duration::days(30)))
        .with_tags(sample_tags_list())
        .with_children(sample_projects_list())
        .build()
}

pub fn sample_project() -> Project {
    ProjectBuilder::new()
        .with_name("This is a sample project title")
        .build()
}

pub fn sample_projects_list() -> Vec<SchedulableItem> {
    let project1 = sample_project();
    let project2 = sample_project();
    let project3 = sample_project();
    vec![
        SchedulableItem::Project(project1.id()),
        SchedulableItem::Project(project2.id()),
        SchedulableItem::Project(project3.id()),
    ]
}

pub fn sample_project_with_tags() -> Project {
    let tags = sample_tags_list();
    ProjectBuilder::new()
        .with_name("Sample Project with Tags")
        .with_tags(tags)
        .build()
}

pub fn sample_project_with_due_date() -> Project {
    let due_date = Some(Utc::now() + Duration::days(1));
    ProjectBuilder::new().with_due_date(due_date).build()
}

pub fn sample_project_with_child_projects() -> Project {
    let children = sample_projects_list();
    ProjectBuilder::new().with_children(children).build()
}

pub fn sample_project_with_child_projects_and_tasks() -> Project {
    let sub_project_1 = sample_project();
    let sub_project_2 = sample_project();
    let sub_project_3 = sample_project();
    let child_task_1 = sample_task();
    let child_task_2 = sample_task();
    let child_task_3 = sample_task();
    let children = vec![
        SchedulableItem::Project(sub_project_1.id()),
        SchedulableItem::Project(sub_project_2.id()),
        SchedulableItem::Project(sub_project_3.id()),
        SchedulableItem::Task(child_task_1.id()),
        SchedulableItem::Task(child_task_2.id()),
        SchedulableItem::Task(child_task_3.id()),
    ];
    ProjectBuilder::new().with_children(children).build()
}

pub fn sample_planned_project() -> Project {
    ProjectBuilder::new()
        .with_status(SchedulableItemStatus::Planned)
        .build()
}

pub fn sample_in_progress_project() -> Project {
    ProjectBuilder::new()
        .with_status(SchedulableItemStatus::InProgress)
        .build()
}

pub fn sample_in_review_project() -> Project {
    ProjectBuilder::new()
        .with_status(SchedulableItemStatus::InReview)
        .build()
}

pub fn sample_completed_project() -> Project {
    ProjectBuilder::new()
        .with_status(SchedulableItemStatus::Completed)
        .build()
}

pub fn sample_archived_project() -> Project {
    ProjectBuilder::new()
        .with_status(SchedulableItemStatus::Archived)
        .build()
}

pub fn sample_canceled_project() -> Project {
    ProjectBuilder::new()
        .with_status(SchedulableItemStatus::Canceled)
        .build()
}

pub fn sample_project_with_dependencies() -> Project {
    let dependency = sample_project();
    ProjectBuilder::new()
        .with_dependencies(vec![dependency.id()])
        .build()
}

// endregion: Factories for Tests

