#[cfg(test)]
mod tests {
    use chrono::{Datelike, Timelike, Utc, Duration};
    use project_tracker_core::HasId;
    use project_tracker_core::models::{person, task, tag, project};
    use project_tracker_core::builders::project_builder;
    use project_tracker_core::factories::project_factory::*;
    use person::Person;
    use task::Task;
    use tag::Tag;
    use project::{ProjectStatus,ProjectSubElement};
    use project_builder::ProjectBuilder;

    #[test]
    fn create_project() {
        let project_name = "This is a sample project title";
        let project = ProjectBuilder::new().with_name(project_name).build();
        assert_eq!(project.name(), project_name);
    }

    #[test]
    fn rename_project() {
        let project_name_new = "This is a different project title";
        let mut project = sample_project();
        assert_ne!(project.name(),project_name_new);
        project.rename(project_name_new);
        assert_eq!(project.name(),project_name_new);
    }

    #[test]
    fn create_project_id() {
        let project = sample_project();
        let project_id  = project.id().to_string();
        assert!(project_id.starts_with("project-"));
    }

    #[test]
    fn assign_owner() {
        let owner = Person::new("Test","McTesty");
        let project = ProjectBuilder::new().with_owner_id(owner.id()).build();
        assert!(project.has_owner());
        assert_eq!(project.owner_id().unwrap().clone(),owner.id());
    }

    #[test]
    fn transfer_ownership() {
        let owner_new = Person::new("Newbie","McNewsy");
        let mut project = sample_project();
        project.transfer_ownership(owner_new.id());
        assert!(project.has_owner());
        assert_eq!(project.owner_id().unwrap().clone(),owner_new.id());
    }

    #[test]
    fn create_project_with_description() {
        let description = "This is a sample description";
        let project = ProjectBuilder::new().with_description(description).build();
        assert!(project.has_description());
        assert_eq!(project.description(),description);
    }

    #[test]
    fn clear_project_description() {
        let description = "This is a sample description";
        let mut project = ProjectBuilder::new().with_description(description).build();
        project.clear_description();
        assert!(!project.has_description());
        assert_eq!(project.description(),"");
    }

    #[test]
    fn add_tag() {
        let test_tag = Tag::new("TestTag");
        let mut project = sample_project();
        project.add_tag(test_tag.id());
        assert!(project.has_tags());
        assert!(project.tags().contains(&test_tag.id().clone()));
    }

    #[test]
    fn add_multiple_tags() {
        let test_tag_1 = Tag::new("TestTag1");
        let test_tag_2 = Tag::new("TestTag2");
        let test_tag_3 = Tag::new("TestTag3");
        let test_tags = vec![test_tag_1.id(), test_tag_2.id(), test_tag_3.id()];
        let mut project = sample_project();
        project.add_tags(test_tags.clone());
        assert!(project.has_tags());
        for tag in test_tags {
            assert!(project.tags().contains(&tag.clone())); 
        }
    }

    #[test]
    fn clear_tags() {
        let mut project = sample_project_with_tags();
        assert!(project.has_tags());
        project.remove_all_tags();
        assert!(!project.has_tags());
    }

    #[test]
    fn remove_tag() {
        let test_tag = Tag::new("TestTag");
        let mut project = ProjectBuilder::new().with_tags(vec![test_tag.id()]).build();
        assert!(project.has_tags());
        project.remove_tag(test_tag.id());
        assert!(!project.has_tags());
    }

    #[test]
    fn remove_multiple_tags() {
        let test_tag_1 = Tag::new("TestTag1");
        let test_tag_2 = Tag::new("TestTag2");
        let test_tag_3 = Tag::new("TestTag3");
        let test_tags = vec![test_tag_1.id(), test_tag_2.id(), test_tag_3.id()];
        let test_tags_to_remove = vec![test_tag_1.id(), test_tag_2.id()];
        let mut project = ProjectBuilder::new().with_tags(test_tags.clone()).build();
        for tag in test_tags {
            assert!(project.tags().contains(&tag.clone())); 
        }
        project.remove_tags(test_tags_to_remove.clone());
        assert!(project.has_tags());
        for tag in test_tags_to_remove {
            assert!(!project.tags().contains(&tag.clone())); 
        }
        assert!(project.tags().contains(&test_tag_3.id().clone()));
    }

    #[test]
    fn create_project_with_start_now() {
        let now = Utc::now();
        let mut project = ProjectBuilder::new().build();
        project.start_now();
        assert!(project.has_start_date());
        assert_eq!(now.year(),project.start_date().unwrap().year());
        assert_eq!(now.month(),project.start_date().unwrap().month());
        assert_eq!(now.day(),project.start_date().unwrap().day());
        assert_eq!(now.hour(),project.start_date().unwrap().hour());
        assert_eq!(now.minute(),project.start_date().unwrap().minute());
    }

    #[test]
    fn create_project_with_start_tomorrow() {
        let now = Utc::now();
        let tomorrow = now + Duration::days(1);
        let mut project = ProjectBuilder::new().build();
        project.start_at_date(tomorrow);
        assert!(project.has_start_date());
        assert_eq!(tomorrow.year(),project.start_date().unwrap().year());
        assert_eq!(tomorrow.month(),project.start_date().unwrap().month());
        assert_eq!(tomorrow.day(),project.start_date().unwrap().day());
    }

    #[test]
    fn create_project_with_start_yesterday() {
        let now = Utc::now();
        let yesterday = now - Duration::days(1);
        let mut project = ProjectBuilder::new().build();
        project.start_at_date(yesterday);
        assert!(project.has_start_date());
        assert_eq!(yesterday.year(),project.start_date().unwrap().year());
        assert_eq!(yesterday.month(),project.start_date().unwrap().month());
        assert_eq!(yesterday.day(),project.start_date().unwrap().day());
    }

    #[test]
    fn remove_start_date() {
        let mut project = ProjectBuilder::new().with_star_date(Utc::now()).build();
        assert!(project.has_start_date());
        project.remove_start_date();
        assert!(!project.has_start_date());
    }

    #[test]
    fn set_due_date_tomorrow() {
        let mut project = sample_project();
        let mut due_date = Utc::now();
        due_date += Duration::days(1);
        assert!(!project.has_due_date());
        project.set_due_date(due_date);
        assert!(project.has_due_date());
        assert_eq!(due_date.year(),project.due_date().unwrap().year());
        assert_eq!(due_date.month(),project.due_date().unwrap().month());
        assert_eq!(due_date.day(),project.due_date().unwrap().day());
    }

    #[test]
    fn set_due_date_yesterday() {
        let mut project = sample_project();
        let mut due_date = Utc::now();
        due_date -= Duration::days(1);
        assert!(!project.has_due_date());
        assert!(!project.is_valid_due_date(Some(due_date))); // yesterday is not avalid due date
        project.set_due_date(due_date);
        assert!(!project.has_due_date());
    }

    #[test]
    fn remove_due_date() {
        let mut project = sample_project_with_due_date();
        assert!(project.has_due_date());
        project.remove_due_date();
        assert!(!project.has_due_date());
    }

    #[test]
    fn promote_from_not_started() {
        let mut project = sample_project();
        project.promote();
        assert!(project.status() == ProjectStatus::Planned);
    }
    #[test]
    fn demote_from_not_started() {
        let mut project = sample_project();
        project.demote();
        assert!(project.status() == ProjectStatus::NotStarted);
    }

    #[test]
    fn promote_from_planned() {
        let mut project = sample_planned_project();
        project.promote();
        assert!(project.status() == ProjectStatus::InProgress);
    }

    #[test]
    fn demote_from_planned() {
        let mut project = sample_planned_project();
        project.demote();
        assert!(project.status() == ProjectStatus::NotStarted);
    }

    #[test]
    fn promote_from_in_progress() {
        let mut project = sample_in_progress_project();
        project.promote();
        assert!(project.status() == ProjectStatus::InReview);
    }
    #[test]
    fn demote_from_in_progress() {
        let mut project = sample_in_progress_project();
        project.demote();
        assert!(project.status() == ProjectStatus::Planned);
    }

    #[test]
    fn promote_from_in_review() {
        let mut project = sample_in_review_project();
        project.promote();
        assert!(project.status() == ProjectStatus::Completed);
    }

    #[test]
    fn demote_from_in_review() {
        let mut project = sample_in_review_project();
        project.demote();
        assert!(project.status() == ProjectStatus::InProgress);
    }

    #[test]
    fn promote_from_completed() {
        let mut project = sample_completed_project();
        project.promote();
        assert!(project.status() == ProjectStatus::Completed);
    }

    #[test]
    fn demote_from_completed() {
        let mut project = sample_completed_project();
        project.demote();
        assert!(project.status() == ProjectStatus::Completed);
    }

    #[test]
    fn archive() {
        let mut project = sample_project();
        project.archive();
        assert!(project.status() == ProjectStatus::Archived);
        let mut project = sample_planned_project();
        project.archive();
        assert!(project.status() == ProjectStatus::Archived);
        let mut project = sample_in_progress_project();
        project.archive();
        assert!(project.status() == ProjectStatus::Archived);
        let mut project = sample_in_review_project();
        project.archive();
        assert!(project.status() == ProjectStatus::Archived);
        let mut project = sample_completed_project();
        project.archive();
        assert!(project.status() == ProjectStatus::Archived);
        let mut project = sample_canceled_project();
        project.archive();
        assert!(project.status() == ProjectStatus::Archived);
    }

    #[test]
    fn promote_from_archived() {
        let mut project = sample_archived_project();
        project.promote();
        assert!(project.status() == ProjectStatus::Archived); // Archive can't be promoted beyond
    }

    #[test]
    fn demote_from_archived() {
        let mut project = sample_archived_project();
        project.demote();
        assert!(project.status() == ProjectStatus::Archived);
    }

    #[test]
    fn cancel() {
        let mut project = sample_project();
        project.cancel();
        assert!(project.status() == ProjectStatus::Canceled);
    }

    #[test]
    fn cancel_archived_project() {
        let mut project = sample_archived_project();
        project.cancel();
        assert!(project.status() == ProjectStatus::Archived);
    }

    #[test]
    fn promote_from_canceled() {
        let mut project = sample_canceled_project();
        project.promote();
        assert!(project.status() == ProjectStatus::Canceled);
    }

    #[test]
    fn demote_from_canceled() {
        let mut project = sample_canceled_project();
        project.demote();
        assert!(project.status() == ProjectStatus::Canceled);
    }

    #[test]
    fn add_child_project_to_project() {
        let mut project = sample_project();
        let child_project = sample_project();
        assert_ne!(project, child_project);
        project.add_child(ProjectSubElement::Project(child_project.id()));
        assert!(project.has_children());
        assert!(project.children().len() == 1);
        assert!(project.project_children().len() == 1);
        assert!(project.task_children().len() == 0);
    }

    #[test]
    fn add_multiple_child_projects_to_project() {
        let mut project = sample_project();
        let children = vec![
            ProjectSubElement::Project(sample_project().id()),
            ProjectSubElement::Project(sample_project().id()),
            ProjectSubElement::Project(sample_project().id())
        ];
        project.add_children(children);
        assert!(project.has_children());
        assert!(project.children().len() == 3);
        assert!(project.project_children().len() == 3);
        assert!(project.task_children().len() == 0);
    }

    #[test]
    fn remove_child_project_from_project() {
        let mut project = sample_project();
        let child_project_2 = sample_project();
        let children = vec![
            ProjectSubElement::Project(sample_project().id()),
            ProjectSubElement::Project(child_project_2.id()),
            ProjectSubElement::Project(sample_project().id())
        ];
        project.add_children(children);
        project.remove_child(ProjectSubElement::Project(child_project_2.id()));
        assert!(!project.has_child(&ProjectSubElement::Project(child_project_2.id())));
    }

    #[test]
    fn remove_multiple_child_projects_from_project() {
        let mut project = sample_project();
        let child_project_1 = sample_project();
        let child_project_2 = sample_project();
        let child_project_3 = sample_project();
        let children = vec![
            ProjectSubElement::Project(child_project_1.id()),
            ProjectSubElement::Project(child_project_2.id()),
            ProjectSubElement::Project(child_project_3.id())
        ];
        let children_to_remove = vec![
            ProjectSubElement::Project(child_project_1.id()),
            ProjectSubElement::Project(child_project_3.id())
        ];
        project.add_children(children);
        project.remove_children(children_to_remove);
        assert!(project.has_children());
        assert!(project.children().len() == 1);
        assert!(project.project_children().len() == 1);
        assert!(project.task_children().len() == 0);
        assert!(project.has_child(&ProjectSubElement::Project(child_project_2.id())));
    }

    #[test]
    fn add_child_task_to_project() {
        let mut project = sample_project();
        let task_name = "This is a child task title";
        let child_task = Task::new(task_name);
        project.add_child(ProjectSubElement::Task(child_task.id()));
        assert!(project.has_children());
        assert!(project.children().len() == 1);
        assert!(project.project_children().len() == 0);
        assert!(project.task_children().len() == 1);
    }

    #[test]
    fn add_multiple_child_tasks_to_project() {
        let mut project = sample_project();
        let child_task_name_1 = "This is a child project title 1";
        let child_task_1 = Task::new(child_task_name_1);
        let child_task_name_2 = "This is a child project title 2";
        let child_task_2 = Task::new(child_task_name_2);
        let child_task_name_3 = "This is a child project title 3";
        let child_task_3 = Task::new(child_task_name_3);
        let children = vec![
            ProjectSubElement::Task(child_task_1.id()),
            ProjectSubElement::Task(child_task_2.id()),
            ProjectSubElement::Task(child_task_3.id())
        ];
        project.add_children(children);
        assert!(project.has_children());
        assert!(project.children().len() == 3);
        assert!(project.project_children().len() == 0);
        assert!(project.task_children().len() == 3);
    }

    #[test]
    fn remove_child_task_from_project() {
        let mut project = sample_project();
        let child_task_name_1 = "This is a child project title 1";
        let child_task_1 = Task::new(child_task_name_1);
        let child_task_name_2 = "This is a child project title 2";
        let child_task_2 = Task::new(child_task_name_2);
        let child_task_name_3 = "This is a child project title 3";
        let child_task_3 = Task::new(child_task_name_3);
        let children = vec![
            ProjectSubElement::Task(child_task_1.id()),
            ProjectSubElement::Task(child_task_2.id()),
            ProjectSubElement::Task(child_task_3.id())
        ];
        project.add_children(children);
        assert!(project.has_children());
        assert!(project.children().len() == 3);
        assert!(project.project_children().len() == 0);
        assert!(project.task_children().len() == 3);
        project.remove_child(ProjectSubElement::Task(child_task_2.id()));
        assert!(project.has_children());
        assert!(project.children().len() == 2);
        assert!(project.project_children().len() == 0);
        assert!(project.task_children().len() == 2);
    }

    #[test]
    fn remove_multiple_child_tasks_from_project() {
        let mut project = sample_project();
        let child_task_name_1 = "This is a child project title 1";
        let child_task_1 = Task::new(child_task_name_1);
        let child_task_name_2 = "This is a child project title 2";
        let child_task_2 = Task::new(child_task_name_2);
        let child_task_name_3 = "This is a child project title 3";
        let child_task_3 = Task::new(child_task_name_3);
        let children = vec![
            ProjectSubElement::Task(child_task_1.id()),
            ProjectSubElement::Task(child_task_2.id()),
            ProjectSubElement::Task(child_task_3.id())
        ];
        let children_to_remove = vec![
            ProjectSubElement::Task(child_task_1.id()),
            ProjectSubElement::Task(child_task_3.id())
        ];
        project.add_children(children);
        assert!(project.has_children());
        assert!(project.children().len() == 3);
        assert!(project.project_children().len() == 0);
        assert!(project.task_children().len() == 3);
        project.remove_children(children_to_remove);
        assert!(project.has_children());
        assert!(project.children().len() == 1);
        assert!(project.project_children().len() == 0);
        assert!(project.task_children().len() == 1);
        assert!(project.task_children().contains(&child_task_2.id().clone()));
    }

    #[test]
    fn add_mixed_children_to_project() {
        let mut project = sample_project();
        let child_project_1 = sample_project();
        let child_project_2 = sample_project();
        let child_project_3 = sample_project();
        let child_task_name_1 = "This is a child project title 1";
        let child_task_1 = Task::new(child_task_name_1);
        let child_task_name_2 = "This is a child project title 2";
        let child_task_2 = Task::new(child_task_name_2);
        let child_task_name_3 = "This is a child project title 3";
        let child_task_3 = Task::new(child_task_name_3);
        let children = vec![
            ProjectSubElement::Project(child_project_1.id()),
            ProjectSubElement::Project(child_project_2.id()),
            ProjectSubElement::Project(child_project_3.id()),
            ProjectSubElement::Task(child_task_1.id()),
            ProjectSubElement::Task(child_task_2.id()),
            ProjectSubElement::Task(child_task_3.id())
        ];
        project.add_children(children);
        assert!(project.has_children());
        assert!(project.children().len() == 6);
        assert!(project.project_children().len() == 3);
        assert!(project.task_children().len() == 3);
    }

    #[test]
    fn remove_all_children() {
        let mut project = sample_project_with_child_projects_and_tasks();
        assert!(project.has_children());
        project.remove_all_children();
        assert!(!project.has_children());
    }

    #[test]
    fn add_dependency() {
        let mut project = sample_project();
        let dependency = sample_project();
        project.add_dependency(dependency.id());
        assert!(project.has_dependencies());
    }

    #[test]
    fn remove_dependency() {
        let mut project = sample_project();
        let dependency = sample_project();
        project.add_dependency(dependency.id());
        assert!(project.has_dependencies());
        project.remove_dependency(dependency.id());
        assert!(!project.has_dependencies());
    }

    #[test]
    fn add_dependencies() {
        let mut project = sample_project();
        let dependency_1 = sample_project();
        let dependency_2 = sample_project();
        project.add_dependencies(vec![
            dependency_1.id(),
            dependency_2.id()
        ]);
        assert!(project.has_dependencies());
        assert_eq!(project.dependencies().len(),2);
    }

    #[test]
    fn remove_dependencies() {
        let mut project = sample_project();
        let dependency_1 = sample_project();
        let dependency_2 = sample_project();
        let dependency_3 = sample_project();
        project.add_dependencies(vec![
            dependency_1.id(),
            dependency_2.id(),
            dependency_3.id()
        ]);
        project.remove_dependencies(vec![
            dependency_1.id(),
            dependency_2.id(),
        ]);
        assert!(project.has_dependencies());
        assert!(project.has_dependency(&dependency_3.id()));
        assert_eq!(project.dependencies().len(),1);
    }
    
    #[test]
    fn remove_all_dependencies() {
        let mut project = sample_project_with_dependencies();
        project.remove_all_dependencies();
        assert!(!project.has_dependencies());
    }

}