// use project_tracker_core::{models::{milestone, person, project, tag, task}, HasId};
// use person::Person;
// use project::Project;
// use tag::Tag;
// use task::Task;

use project_tracker_core::{builders::task_builder::TaskBuilder, factories::{project_factory::*, task_factory::*}};
use chrono::{DateTime, Utc};

fn main() {
    let task1 = sample_task();
    println!("{task1}");

    let task2 = TaskBuilder::new().with_name("This is a sample Task").with_description("Sample Description").with_due_date(Utc::now()).build();
    println!("{task2}");
}