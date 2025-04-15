// use project_tracker_core::{models::{milestone, person, project, tag, task}, HasId};
// use person::Person;
// use project::Project;
// use tag::Tag;
// use task::Task;

use project_tracker_core::factories::{project_factory::*, task_factory::*, tag_factory::*};

fn main() {
    let task1 = sample_task();
    println!("{task1}");

    let task2 = sample_task_with_due_date();
    println!("{task2}");

    let project1 = sample_project_with_tags();
    println!("{project1}");

    let tag1 = sample_tag();
    println!("{tag1}");
}