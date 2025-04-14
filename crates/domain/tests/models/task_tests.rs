#[cfg(test)]
mod tests {
    use chrono::{Datelike, Timelike, Utc, Duration};
    use project_tracker_core::HasId;
    use project_tracker_core::builders::task_builder::*;
    use project_tracker_core::factories::task_factory::*;
    use project_tracker_core::models::{person,tag, project};
    use project::ProjectStatus;
    use person::Person;
    use tag::Tag;

    #[test]
    fn create_task() {
        let task = TaskBuilder::new().with_name("This is a sample project title").build();
        assert_eq!(task.name(), "This is a sample project title");
    }

    #[test]
    fn rename_task() {
        let name = "Sample";
        let mut task = sample_task();
        task.rename(name);
        assert_eq!(task.name(), name);
    }

    #[test]
    fn create_task_id() {
        let task = sample_task();
        let task_id = task.id().to_string();
        assert!(task_id.starts_with("task-"));
    }

    #[test]
    fn assign_task_owner() {
        let owner = Person::new("Test","McTesty");
        let task = TaskBuilder::new().with_owner_id(owner.id()).build();
        assert!(task.has_owner());
        assert_eq!(task.owner_id().unwrap().clone(),owner.id());
    }

    #[test]
    fn transfer_task_ownership() {
        let owner_new = Person::new("Newbie","McNewsy");
        let mut task = sample_task();
        task.transfer_ownership(owner_new.id());
        assert!(task.has_owner());
        assert_eq!(task.owner_id().unwrap().clone(),owner_new.id());
    }

    #[test]
    fn create_task_with_description() {
        let description = "This is a sample description";
        let task = TaskBuilder::new().with_description(description).build();
        assert!(task.has_description());
        assert_eq!(task.description(),description);
    }

    #[test]
    fn clear_task_description() {
        let description = "This is a sample description";
        let mut task = TaskBuilder::new().with_description(description).build();
        task.clear_description();
        assert!(!task.has_description());
        assert_eq!(task.description(),"");
    }

    #[test]
    fn add_tag() {
        let test_tag = Tag::new("TestTag");
        let mut task = sample_task();
        task.add_tag(test_tag.id());
        assert!(task.has_tags());
        assert!(task.tags().contains(&test_tag.id().clone()));
    }

    #[test]
    fn add_multiple_tags() {
        let test_tag_1 = Tag::new("TestTag1");
        let test_tag_2 = Tag::new("TestTag2");
        let test_tag_3 = Tag::new("TestTag3");
        let test_tags = vec![test_tag_1.id(), test_tag_2.id(), test_tag_3.id()];
        let mut task = sample_task();
        task.add_tags(test_tags.clone());
        assert!(task.has_tags());
        for tag in test_tags {
            assert!(task.tags().contains(&tag.clone())); 
        }
    }

    #[test]
    fn clear_tags() {
        let mut task = sample_task_with_tags();
        assert!(task.has_tags());
        task.remove_all_tags();
        assert!(!task.has_tags());
    }

    #[test]
    fn remove_tag() {
        let test_tag = Tag::new("TestTag");
        let mut task = TaskBuilder::new().with_tags(vec![test_tag.id()]).build();
        assert!(task.has_tags());
        task.remove_tag(test_tag.id());
        assert!(!task.has_tags());
    }

    #[test]
    fn remove_multiple_tags() {
        let test_tag_1 = Tag::new("TestTag1");
        let test_tag_2 = Tag::new("TestTag2");
        let test_tag_3 = Tag::new("TestTag3");
        let test_tags = vec![test_tag_1.id(), test_tag_2.id(), test_tag_3.id()];
        let test_tags_to_remove = vec![test_tag_1.id(), test_tag_2.id()];
        let mut task = TaskBuilder::new().with_tags(test_tags.clone()).build();
        for tag in test_tags {
            assert!(task.tags().contains(&tag.clone())); 
        }
        task.remove_tags(test_tags_to_remove.clone());
        assert!(task.has_tags());
        for tag in test_tags_to_remove {
            assert!(!task.tags().contains(&tag.clone())); 
        }
        assert!(task.tags().contains(&test_tag_3.id().clone()));
    }

    #[test]
    fn create_task_with_start_now() {
        let now = Utc::now();
        let mut task = TaskBuilder::new().build();
        task.start_now();
        assert!(task.has_start_date());
        assert_eq!(now.year(),task.start_date().unwrap().year());
        assert_eq!(now.month(),task.start_date().unwrap().month());
        assert_eq!(now.day(),task.start_date().unwrap().day());
        assert_eq!(now.hour(),task.start_date().unwrap().hour());
        assert_eq!(now.minute(),task.start_date().unwrap().minute());
    }

    #[test]
    fn create_task_with_start_tomorrow() {
        let now = Utc::now();
        let tomorrow = now + Duration::days(1);
        let mut task = TaskBuilder::new().build();
        task.start_at_date(tomorrow);
        assert!(task.has_start_date());
        assert_eq!(tomorrow.year(),task.start_date().unwrap().year());
        assert_eq!(tomorrow.month(),task.start_date().unwrap().month());
        assert_eq!(tomorrow.day(),task.start_date().unwrap().day());
    }

    #[test]
    fn create_task_with_start_yesterday() {
        let now = Utc::now();
        let yesterday = now - Duration::days(1);
        let mut task = TaskBuilder::new().build();
        task.start_at_date(yesterday);
        assert!(task.has_start_date());
        assert_eq!(yesterday.year(),task.start_date().unwrap().year());
        assert_eq!(yesterday.month(),task.start_date().unwrap().month());
        assert_eq!(yesterday.day(),task.start_date().unwrap().day());
    }

    #[test]
    fn remove_start_date() {
        let mut task = TaskBuilder::new().with_star_date(Utc::now()).build();
        assert!(task.has_start_date());
        task.remove_start_date();
        assert!(!task.has_start_date());
    }

    #[test]
    fn set_due_date_tomorrow() {
        let mut task = sample_task();
        let mut due_date = Utc::now();
        due_date += Duration::days(1);
        assert!(!task.has_due_date());
        task.set_due_date(due_date);
        assert!(task.has_due_date());
        assert_eq!(due_date.year(),task.due_date().unwrap().year());
        assert_eq!(due_date.month(),task.due_date().unwrap().month());
        assert_eq!(due_date.day(),task.due_date().unwrap().day());
    }

    #[test]
    fn set_due_date_yesterday() {
        let mut task = sample_task();
        let mut due_date = Utc::now();
        due_date -= Duration::days(1);
        assert!(!task.has_due_date());
        assert!(!task.is_valid_due_date(Some(due_date))); // yesterday is not avalid due date
        task.set_due_date(due_date);
        assert!(!task.has_due_date());
    }

    #[test]
    fn remove_due_date() {
        let mut task = sample_task_with_due_date();
        assert!(task.has_due_date());
        task.remove_due_date();
        assert!(!task.has_due_date());
    }

    #[test]
    fn promote_from_not_started() {
        let mut task = sample_task();
        task.promote();
        assert!(task.status() == ProjectStatus::Planned);
    }
    #[test]
    fn demote_from_not_started() {
        let mut task = sample_task();
        task.demote();
        assert!(task.status() == ProjectStatus::NotStarted);
    }

    #[test]
    fn promote_from_planned() {
        let mut task = sample_planned_task();
        task.promote();
        assert!(task.status() == ProjectStatus::InProgress);
    }

    #[test]
    fn demote_from_planned() {
        let mut task = sample_planned_task();
        task.demote();
        assert!(task.status() == ProjectStatus::NotStarted);
    }

    #[test]
    fn promote_from_in_progress() {
        let mut task = sample_in_progress_task();
        task.promote();
        assert!(task.status() == ProjectStatus::InReview);
    }
    #[test]
    fn demote_from_in_progress() {
        let mut task = sample_in_progress_task();
        task.demote();
        assert!(task.status() == ProjectStatus::Planned);
    }

    #[test]
    fn promote_from_in_review() {
        let mut task = sample_in_review_task();
        task.promote();
        assert!(task.status() == ProjectStatus::Completed);
    }

    #[test]
    fn demote_from_in_review() {
        let mut task = sample_in_review_task();
        task.demote();
        assert!(task.status() == ProjectStatus::InProgress);
    }

    #[test]
    fn promote_from_completed() {
        let mut task = sample_completed_task();
        task.promote();
        assert!(task.status() == ProjectStatus::Completed);
    }

    #[test]
    fn demote_from_completed() {
        let mut task = sample_completed_task();
        task.demote();
        assert!(task.status() == ProjectStatus::Completed);
    }

    #[test]
    fn archive() {
        let mut task = sample_task();
        task.archive();
        assert!(task.status() == ProjectStatus::Archived);
        let mut task = sample_planned_task();
        task.archive();
        assert!(task.status() == ProjectStatus::Archived);
        let mut task = sample_in_progress_task();
        task.archive();
        assert!(task.status() == ProjectStatus::Archived);
        let mut task = sample_in_review_task();
        task.archive();
        assert!(task.status() == ProjectStatus::Archived);
        let mut task = sample_completed_task();
        task.archive();
        assert!(task.status() == ProjectStatus::Archived);
        let mut task = sample_canceled_task();
        task.archive();
        assert!(task.status() == ProjectStatus::Archived);
    }

    #[test]
    fn promote_from_archived() {
        let mut task = sample_archived_task();
        task.promote();
        assert!(task.status() == ProjectStatus::Archived); // Archive can't be promoted beyond
    }

    #[test]
    fn demote_from_archived() {
        let mut task = sample_archived_task();
        task.demote();
        assert!(task.status() == ProjectStatus::Archived);
    }

    #[test]
    fn cancel() {
        let mut task = sample_task();
        task.cancel();
        assert!(task.status() == ProjectStatus::Canceled);
    }

    #[test]
    fn cancel_archived_project() {
        let mut task = sample_archived_task();
        task.cancel();
        assert!(task.status() == ProjectStatus::Archived);
    }

    #[test]
    fn promote_from_canceled() {
        let mut task = sample_canceled_task();
        task.promote();
        assert!(task.status() == ProjectStatus::Canceled);
    }

    #[test]
    fn demote_from_canceled() {
        let mut task = sample_canceled_task();
        task.demote();
        assert!(task.status() == ProjectStatus::Canceled);
    }

    #[test]
    fn add_child_task() {
        let mut task = sample_task();
        let child_task = sample_task();
        task.add_child(child_task.id());
        assert!(task.has_children());
        assert!(task.children().len() == 1);
    }

    #[test]
    fn add_multiple_child_tasks() {
        let mut task = sample_task();
        let child_task_1 = sample_task();
        let child_task_2 = sample_task();
        let child_task_3 = sample_task();
        let children = vec![
            child_task_1.id(),
            child_task_2.id(),
            child_task_3.id()
        ];
        task.add_children(children);
        assert!(task.has_children());
        assert!(task.children().len() == 3);
    }

    #[test]
    fn remove_child_task() {
        let mut task = sample_task();
        let child_task_1 = sample_task();
        let child_task_2 = sample_task();
        let child_task_3 = sample_task();
        let children = vec![
            child_task_1.id(),
            child_task_2.id(),
            child_task_3.id()
        ];
        task.add_children(children);
        assert!(task.has_children());
        assert!(task.children().len() == 3);
        task.remove_child(child_task_2.id());
        assert!(task.has_children());
        assert!(task.children().len() == 2);
    }

    #[test]
    fn remove_multiple_child_tasks() {
        let mut task = sample_task();
        let child_task_1 = sample_task();
        let child_task_2 = sample_task();
        let child_task_3 = sample_task();
        let children = vec![
            child_task_1.id(),
            child_task_2.id(),
            child_task_3.id()
        ];
        let children_to_remove = vec![
            child_task_1.id(),
            child_task_3.id()
        ];
        task.add_children(children);
        assert!(task.has_children());
        assert!(task.children().len() == 3);
        task.remove_children(children_to_remove);
        assert!(task.has_children());
        assert!(task.children().len() == 1);
        assert!(task.children().contains(&child_task_2.id().clone()));
    }

    #[test]
    fn remove_all_children() {
        let mut task = sample_task_with_children();
        assert!(task.has_children());
        task.remove_all_children();
        assert!(!task.has_children());
    }

    #[test]
    fn add_dependency() {
        let mut task = sample_task();
        let dependency = sample_task();
        task.add_dependency(dependency.id());
        assert!(task.has_dependencies());
    }

    #[test]
    fn remove_dependency() {
        let mut task = sample_task();
        let dependency = sample_task();
        task.add_dependency(dependency.id());
        assert!(task.has_dependencies());
        task.remove_dependency(dependency.id());
        assert!(!task.has_dependencies());
    }

    #[test]
    fn add_dependencies() {
        let mut task = sample_task();
        let dependency_1 = sample_task();
        let dependency_2 = sample_task();
        task.add_dependencies(vec![
            dependency_1.id(),
            dependency_2.id()
        ]);
        assert!(task.has_dependencies());
        assert_eq!(task.dependencies().len(),2);
    }

    #[test]
    fn remove_dependencies() {
        let mut task = sample_task();
        let dependency_1 = sample_task();
        let dependency_2 = sample_task();
        let dependency_3 = sample_task();
        task.add_dependencies(vec![
            dependency_1.id(),
            dependency_2.id(),
            dependency_3.id()
        ]);
        task.remove_dependencies(vec![
            dependency_1.id(),
            dependency_2.id(),
        ]);
        assert!(task.has_dependencies());
        assert!(task.has_dependency(&dependency_3.id()));
        assert_eq!(task.dependencies().len(),1);
    }
    
    #[test]
    fn remove_all_dependencies() {
        let mut task = sample_task_with_dependencies();
        task.remove_all_dependencies();
        assert!(!task.has_dependencies());
    }
}