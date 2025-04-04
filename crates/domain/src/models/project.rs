use crate::id::Id;
use crate::{EntityType,HasId};
use super::task::Task;
use super::person::Person;
use super::tag::Tag;

use core::fmt;
use chrono::{DateTime, Datelike, Utc};
// use serde::{Serialize, Deserialize};

/// Represents a Project in the Project Tracker App, consisting of some metadata, sub elements as well as displaying some dates and dependencies.
/// # Examples
/// ```
/// use project_tracker_core::models::project::Project;
/// 
/// let project_1 = Project::new("Building a cool new Project Tracking App in Rust");
/// ```
#[derive(Clone)]
pub struct Project {
    id: Id<Project>,
    name: String,
    owner_id: Option<Id<Person>>,
    description: Option<String>,
    tags: Vec<Id<Tag>>,
    start_date: Option<DateTime<Utc>>,
    due_date: Option<DateTime<Utc>>,
    sub_elements: Vec<ProjectSubElement>,
    dependencies: Vec<Id<Project>>,
    status: ProjectStatus,
}

impl Project {
    pub fn new(name: &str) -> Self {
        Project {
            id: Id::<Project>::new(),
            name: name.into(),
            owner_id: None,
            description: None,
            tags: Vec::new(),
            start_date: None,
            due_date: None,
            sub_elements: Vec::new(),
            dependencies: Vec::new(),
            status: ProjectStatus::NotStarted
        }
    }

    pub fn name(&self) -> &str {
        &self.name.as_str()
    }

    pub fn has_owner(&self) -> bool {
        self.owner_id.is_some()
    }

    pub fn owner_id(&self) -> Option<&Id<Person>> {
        self.owner_id.as_ref()
    }

    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }

    pub fn description(&self) -> &str {
        if let Some(description) = &self.description {
            description.as_str()
        } else {
            ""
        }
    }

    pub fn has_tags(&self) -> bool {
        self.tags().len() > 0
    }

    pub fn tags(&self) -> Vec<Id<Tag>> {
        self.tags.clone()
    }

    pub fn has_start_date(&self) -> bool {
        self.start_date.is_some()
    }

    pub fn start_date(&self) -> Option<DateTime<Utc>> {
        self.start_date
    }

    pub fn has_due_date(&self) -> bool {
        self.due_date.is_some()
    }

    pub fn due_date(&self) -> Option<DateTime<Utc>> {
        self.due_date
    }

    pub fn rename(&mut self, name: &str) -> &Self {
        self.name = name.into();
        self
    }

    pub fn set_owner(mut self, owner_id: Id<Person>) -> Self {
        self.owner_id = Some(owner_id);
        self
    }

    pub fn transfer_ownership(&mut self, owner_id: Id<Person>) -> &Self {
        self.owner_id = Some(owner_id);
        self
    }

    pub fn set_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn clear_description(&mut self) -> &Self {
        self.description = None;
        self
    }

    pub fn add_tag(mut self, tag: Id<Tag>) -> Self {
        self.tags.push(tag);
        self
    }

    pub fn add_tags(mut self, mut tags: Vec<Id<Tag>>) -> Self {
        self.tags.append(&mut tags);
        self
    }

    pub fn start_now(mut self) -> Self {
        self.start_date = Some(Utc::now());
        self
    }

    pub fn start_at_date(mut self, start_date: DateTime<Utc>) -> Self {
        self.start_date = Some(start_date);
        self
    }

    pub fn remove_start_date(&mut self) -> &Self {
        self.start_date = None;
        self
    }

    pub fn set_due_date(&mut self, due_date: DateTime<Utc>) -> &Self {
        self.due_date = Some(due_date);
        self
    }

    pub fn remove_due_date(&mut self) -> &Self {
        self.due_date = None;
        self
    }

    pub fn add_sub_element(mut self,sub_element: ProjectSubElement) -> Self {
        self.sub_elements.push(sub_element);
        self
    }

    pub fn add_sub_elements(mut self,mut sub_elements: Vec<ProjectSubElement>) -> Self {
        self.sub_elements.append(&mut sub_elements);
        self
    }

    pub fn add_dependency(mut self, project_id: Id<Project>) -> Self {
        self.dependencies.push(project_id);
        self
    }

    pub fn add_dependencies(mut self, mut project_ids: Vec<Id<Project>>) -> Self {
        self.dependencies.append(&mut project_ids);
        self
    }

    pub fn remove_tag(&mut self, tag: Id<Tag>) -> &Self {
        let index = self.tags.iter().position(|t| t == &tag).unwrap();
        self.tags.remove(index);
        self
    }

    pub fn remove_tags(&mut self, tags: Vec<Id<Tag>>) -> &Self {
        if !tags.is_empty() {
            for tag in tags {
                let index = self.tags.iter().position(|t| t == &tag).unwrap();
                self.tags.remove(index);
            }
        }
        self
    }

    pub fn remove_all_tags(&mut self) -> &Self {
        self.tags.clear();
        self
    }

    pub fn promote(&mut self) -> &Self {
        match self.status {
            ProjectStatus::NotStarted => self.status = ProjectStatus::Planned,
            ProjectStatus::Planned => self.status = ProjectStatus::InProgress,
            ProjectStatus::InProgress => self.status = ProjectStatus::InReview,
            ProjectStatus::InReview => self.status = ProjectStatus::Completed,
            _ => (),
        }
        self
    }

    pub fn demote(&mut self) -> &Self {
        match self.status {
            ProjectStatus::InReview => self.status = ProjectStatus::InProgress,
            ProjectStatus::InProgress => self.status = ProjectStatus::Planned,
            ProjectStatus::Planned => self.status = ProjectStatus::NotStarted,
            _ => (),
        }
        self
    }

    pub fn archive(&mut self) -> &Self {
        self.status = ProjectStatus::Archived;
        self
    }

    pub fn cancel(&mut self) -> &Self {
        self.status = ProjectStatus::Canceled;
        self
    }

}

impl fmt::Debug for Project {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Project [[{}]]",self.name)?;
        writeln!(f, "- Project Id:{:?}",self.id)?;
        if let Some(description) = &self.description {
            writeln!(f, "- Project Description: {}",description)?;
        } else {
            writeln!(f, "! No description provided")?;
        }
        if let Some(owner_id) = &self.owner_id {
            writeln!(f, "- Project Owner: {:?}",owner_id)?;
        } else {
            writeln!(f, "! No project owner")?;
        }
        if let Some(start_date) = &self.start_date {
            let year = start_date.year();
            let month = start_date.month();
            let day = start_date.day();
            let week = start_date.iso_week().week();
            writeln!(f, "- Project starts on: {}-{}-{} [Week {}]",day, month, year, week)?;
        } else {
            writeln!(f, "! No start date defined")?;
        }
        if let Some(due_date) = &self.due_date {
            let year = due_date.year();
            let month = due_date.month();
            let day = due_date.day();
            let week = due_date.iso_week().week();
            writeln!(f, "- Project is due on: {}-{}-{} [Week {}]",day, month, year, week)?;
        } else {
            writeln!(f, "! No due date defined")?;
        }
        writeln!(f, "- Project has {} children",self.sub_elements.len())?;
        {
            let mut child_projects: Vec<Id<Project>> = Vec::new();
            let mut child_tasks: Vec<Id<Task>> = Vec::new();
            for element in self.sub_elements.clone() {
                match element {
                    ProjectSubElement::Project(id) => {
                        child_projects.push(id);
                    },
                    ProjectSubElement::Task(id) => {
                        child_tasks.push(id);
                    }
                }
            }
            writeln!(f, "-- {}/{} children are projects",child_projects.len(), self.sub_elements.len())?;
            writeln!(f, "-- {}/{} children are tasks",child_tasks.len(), self.sub_elements.len())?;
        }
        Ok(())
    }
}

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[[{}]]",self.name)?;
        writeln!(f, "- Project Id: {}",self.id)?;
        if let Some(description) = &self.description {
            writeln!(f, "- Project Description: {}",description)?;
        }
        if let Some(owner_id) = &self.owner_id {
            writeln!(f, "- Project Owner: {}",owner_id)?;
        }
        if let Some(start_date) = &self.start_date {
            let year = start_date.year();
            let month = start_date.month();
            let day = start_date.day();
            let week = start_date.iso_week().week();
            writeln!(f, "- Project starts on: {}-{}-{} [Week {}]",day, month, year, week)?;
        }
        if let Some(due_date) = &self.due_date {
            let year = due_date.year();
            let month = due_date.month();
            let day = due_date.day();
            let week = due_date.iso_week().week();
            writeln!(f, "- Project is due on: {}-{}-{} [Week {}]",day, month, year, week)?;
        }
        writeln!(f, "- Project has {} children",self.sub_elements.len())?;
        {
            let mut child_projects: Vec<Id<Project>> = Vec::new();
            let mut child_tasks: Vec<Id<Task>> = Vec::new();
            for element in self.sub_elements.clone() {
                match element {
                    ProjectSubElement::Project(id) => {
                        child_projects.push(id);
                    },
                    ProjectSubElement::Task(id) => {
                        child_tasks.push(id);
                    }
                }
            }
            writeln!(f, "-- {}/{} children are projects",child_projects.len(), self.sub_elements.len())?;
            writeln!(f, "-- {}/{} children are tasks",child_tasks.len(), self.sub_elements.len())?;
        }
        Ok(())
    }
}

#[derive(Clone)]
pub enum ProjectStatus {
    NotStarted,
    Planned,
    InProgress,
    InReview,
    Completed,
    Archived,
    Canceled,
}

#[derive(Clone)]
pub enum ProjectSubElement {
    Project(Id<Project>),
    Task(Id<Task>)
}

impl EntityType for Project {
    fn prefix() -> &'static str {
        "project"
    }
}

impl HasId for Project {
    type Entity = Project;

    fn id(&self) -> Id<Project> {
        self.id.clone()
    }
}

/* ### TESTING ### */

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Timelike, Utc, Duration};
    use crate::HasId;
    use super::{Project, Person, Tag, Task};

    #[test]
    fn create_project() {
        let project_name = "This is a sample project title";
        let project = Project::new(project_name);
        assert_eq!(project.name(), project_name);
    }

    #[test]
    fn rename_project() {
        let project_name_old = "This is a sample project title";
        let project_name_new = "This is a different project title";
        let mut project = Project::new(&project_name_old);
        assert_eq!(project.name(),project_name_old);
        project.rename(project_name_new);
        assert_eq!(project.name(),project_name_new);
    }

    #[test]
    fn create_project_id() {
        let project_name = "This is a sample project title";
        let project = Project::new(project_name);
        let project_id  = project.id().to_string();
        assert!(project_id.starts_with("project-"));
    }

    #[test]
    fn assign_owner() {
        let owner = Person::new("Test","McTesty");
        let project_name = "This is a sample project title";
        let project = Project::new(project_name).set_owner(owner.id());
        assert!(project.has_owner());
        assert_eq!(project.owner_id().unwrap().clone(),owner.id());
    }

    #[test]
    fn transfer_ownership() {
        let owner_old = Person::new("Test","McTesty");
        let owner_new = Person::new("Newbie","McNewsy");
        let project_name = "This is a sample project title";
        let mut project = Project::new(project_name).set_owner(owner_old.id());
        assert!(project.has_owner());
        assert_eq!(project.owner_id().unwrap().clone(),owner_old.id());
        project.transfer_ownership(owner_new.id());
        assert_eq!(project.owner_id().unwrap().clone(),owner_new.id());
    }

    #[test]
    fn create_project_with_description() {
        let project_name = "This is a sample project title";
        let description = "This is a sample description";
        let project = Project::new(project_name).set_description(description);
        assert!(project.has_description());
        assert_eq!(project.description(),description);
    }

    #[test]
    fn clear_project_description() {
        let project_name = "This is a sample project title";
        let description = "This is a sample description";
        let mut project = Project::new(project_name).set_description(description);
        assert!(project.has_description());
        assert_eq!(project.description(),description);
        project.clear_description();
        assert!(!project.has_description());
        assert_eq!(project.description(),"");
    }

    #[test]
    fn add_tag() {
        let test_tag = Tag::new("TestTag");
        let project_name = "This is a sample project title";
        let project = Project::new(project_name).add_tag(test_tag.id());
        assert!(project.has_tags());
        assert!(project.tags().contains(&test_tag.id().clone()));
    }

    #[test]
    fn add_multiple_tags() {
        let test_tag_1 = Tag::new("TestTag1");
        let test_tag_2 = Tag::new("TestTag2");
        let test_tag_3 = Tag::new("TestTag3");
        let test_tags = vec![test_tag_1.id(), test_tag_2.id(), test_tag_3.id()];
        let project_name = "This is a sample project title";
        let project = Project::new(project_name).add_tags(test_tags.clone());
        assert!(project.has_tags());
        for tag in test_tags {
            assert!(project.tags().contains(&tag.clone())); 
        }
    }

    #[test]
    fn clear_tags() {
        let test_tag_1 = Tag::new("TestTag1");
        let test_tag_2 = Tag::new("TestTag2");
        let test_tag_3 = Tag::new("TestTag3");
        let test_tags = vec![test_tag_1.id(), test_tag_2.id(), test_tag_3.id()];
        let project_name = "This is a sample project title";
        let mut project = Project::new(project_name).add_tags(test_tags.clone());
        assert!(project.has_tags());
        project.remove_all_tags();
        assert!(!project.has_tags());
    }

    #[test]
    fn remove_tag() {
        let test_tag = Tag::new("TestTag");
        let project_name = "This is a sample project title";
        let mut project = Project::new(project_name).add_tag(test_tag.id());
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
        let project_name = "This is a sample project title";
        let mut project = Project::new(project_name).add_tags(test_tags.clone());
        assert!(project.has_tags());
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
        let project_name = "This is a sample project title";
        let now = Utc::now();
        let project = Project::new(project_name).start_now();
        assert!(project.has_start_date());
        assert_eq!(now.year(),project.start_date().unwrap().year());
        assert_eq!(now.month(),project.start_date().unwrap().month());
        assert_eq!(now.day(),project.start_date().unwrap().day());
        assert_eq!(now.hour(),project.start_date().unwrap().hour());
        assert_eq!(now.minute(),project.start_date().unwrap().minute());
    }

    #[test]
    fn create_project_with_start_tomorrow() {
        let project_name = "This is a sample project title";
        let now = Utc::now();
        let tomorrow = now + Duration::days(1);
        let project = Project::new(project_name).start_at_date(tomorrow);
        assert!(project.has_start_date());
        assert_eq!(tomorrow.year(),project.start_date().unwrap().year());
        assert_eq!(tomorrow.month(),project.start_date().unwrap().month());
        assert_eq!(tomorrow.day(),project.start_date().unwrap().day());
        assert_eq!(tomorrow.hour(),project.start_date().unwrap().hour());
        assert_eq!(tomorrow.minute(),project.start_date().unwrap().minute());
    }

    #[test]
    fn create_project_with_start_yesterday() {
        let project_name = "This is a sample project title";
        let now = Utc::now();
        let yesterday = now - Duration::days(1);
        let project = Project::new(project_name).start_at_date(yesterday);
        assert!(project.has_start_date());
        assert_eq!(yesterday.year(),project.start_date().unwrap().year());
        assert_eq!(yesterday.month(),project.start_date().unwrap().month());
        assert_eq!(yesterday.day(),project.start_date().unwrap().day());
        assert_eq!(yesterday.hour(),project.start_date().unwrap().hour());
        assert_eq!(yesterday.minute(),project.start_date().unwrap().minute());
    }

    #[test]
    fn remove_start_date() {
        let project_name = "This is a sample project title";
        let mut project = Project::new(project_name).start_now();
        assert!(project.has_start_date());
        project.remove_start_date();
        assert!(!project.has_start_date());
    }

    #[test]
    fn set_due_date_tomorrow() {
        let project_name = "This is a sample project title";
        let mut project = Project::new(project_name);
        let mut due_date = Utc::now();
        due_date += Duration::days(1);
        assert!(!project.has_due_date());
        project.set_due_date(due_date);
        assert!(project.has_due_date());
        assert_eq!(due_date.year(),project.due_date().unwrap().year());
        assert_eq!(due_date.month(),project.due_date().unwrap().month());
        assert_eq!(due_date.day(),project.due_date().unwrap().day());
        assert_eq!(due_date.hour(),project.due_date().unwrap().hour());
        assert_eq!(due_date.minute(),project.due_date().unwrap().minute());
    }

    #[test]
    fn remove_due_date() {
        let project_name = "This is a sample project title";
        let mut project = Project::new(project_name);
        let mut due_date = Utc::now();
        due_date += Duration::days(1);
        project.set_due_date(due_date);
        assert!(project.has_due_date());
        project.remove_due_date();
        assert!(!project.has_due_date());
    }
}