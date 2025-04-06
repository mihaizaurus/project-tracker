use chrono::{Utc,Duration};

use crate::models::project::{Project, ProjectStatus, ProjectSubElement};
use crate::models::task::Task;
use crate::models::tag::Tag;
use crate::models::project_builder::ProjectBuilder;
use crate::HasId;

pub fn sample_project() -> Project {
    ProjectBuilder::new().with_name("This is a sample project title").build()
}

pub fn sample_project_with_tags() -> Project {
    let tag_1 = Tag::new("TestTag1");
    let tag_2 = Tag::new("TestTag2");
    let tag_3 = Tag::new("TestTag3");
    let tags = vec![tag_1.id(), tag_2.id(), tag_3.id()];
    ProjectBuilder::new().with_tags(tags).build()
}

pub fn sample_project_with_due_date() -> Project {
    let due_date = Utc::now() + Duration::days(1);
    ProjectBuilder::new().with_due_date(due_date).build()
}

pub fn sample_project_with_child_projects() -> Project {
    let sub_project_1 = sample_project();
    let sub_project_2 = sample_project();
    let sub_project_3 = sample_project();
    let children = vec![
        ProjectSubElement::Project(sub_project_1.id()),
        ProjectSubElement::Project(sub_project_2.id()),
        ProjectSubElement::Project(sub_project_3.id())
    ];
    ProjectBuilder::new().with_children(children).build()
}

pub fn sample_project_with_child_projects_and_tasks() -> Project {
    let sub_project_1 = sample_project();
    let sub_project_2 = sample_project();
    let sub_project_3 = sample_project();
        let child_task_1 = Task::new("");
        let child_task_2 = Task::new("");
        let child_task_3 = Task::new("");
    let children = vec![
        ProjectSubElement::Project(sub_project_1.id()),
        ProjectSubElement::Project(sub_project_2.id()),
        ProjectSubElement::Project(sub_project_3.id()),
        ProjectSubElement::Task(child_task_1.id()),
        ProjectSubElement::Task(child_task_2.id()),
        ProjectSubElement::Task(child_task_3.id())
    ];
    ProjectBuilder::new().with_children(children).build()
}

pub fn sample_planned_project() -> Project {
    ProjectBuilder::new().with_status(ProjectStatus::Planned).build()
}

pub fn sample_in_progress_project() -> Project {
    ProjectBuilder::new().with_status(ProjectStatus::InProgress).build()
}

pub fn sample_in_review_project() -> Project {
    ProjectBuilder::new().with_status(ProjectStatus::InReview).build()
}

pub fn sample_completed_project() -> Project {
    ProjectBuilder::new().with_status(ProjectStatus::Completed).build()
}

pub fn sample_archived_project() -> Project {
    ProjectBuilder::new().with_status(ProjectStatus::Archived).build()
}

pub fn sample_canceled_project() -> Project {
    ProjectBuilder::new().with_status(ProjectStatus::Canceled).build()
}

pub fn sample_project_with_dependencies() -> Project {
    let dependency = sample_project();
    ProjectBuilder::new().with_dependencies(vec![dependency.id()]).build()
}