use chrono::{Datelike, Timelike, Utc, Duration};
use project_tracker_core::HasId;
use project_tracker_core::models::{person, project};
use project_tracker_core::builders::tag_builder;
use project_tracker_core::factories::tag_factory::*;
use tag_builder::TagBuilder;

#[test]
fn create_tag() {
    let tag = sample_tag();
    assert_eq!(tag.name(), "SampleTag");
}

#[test]
fn create_tag_id() {
    let tag = sample_tag();
    let tag_id = tag.id().to_string();
    assert!(tag_id.starts_with("tag-"));
}

#[test]
fn rename_tag() {
    let mut tag = sample_tag();
    let new_name = "NewTagName";
    tag.rename(new_name);
    assert_eq!(tag.name(),new_name);
}

#[test]
fn set_description() {
    let mut tag = sample_tag();
    tag.set_description("Sample Description");
    assert!(tag.has_description());
    assert_eq!(tag.description(),"Sample Description");
}

#[test]
fn clear_description() {
    let mut tag = sample_tag();
    tag.clear_description();
    assert!(!tag.has_description());
    assert_eq!(tag.description(),"");
}

#[test]
fn add_parent() {
    let mut tag = sample_tag();
    let parent = sample_tag();
    assert!(!tag.has_parents());
    tag.add_parent(parent.id());
    assert!(tag.has_parents());
    assert!(tag.parents().contains(&parent.id()));
}

#[test]
fn add_parents() {
    let mut tag = sample_tag();
    let parents = sample_tags_list();
    assert!(!tag.has_parents());
    tag.add_parents(parents);
    assert!(tag.has_parents());
    assert_eq!(tag.parents().len(),3);
}

#[test]
fn remove_parent() {
    let mut tag = sample_tag();
    let parent = sample_tag();
    assert!(!tag.has_parents());
    tag.add_parent(parent.id());
    assert!(tag.has_parents());
    assert!(tag.parents().contains(&parent.id()));
    tag.remove_parent(parent.id());
    assert!(!tag.has_parents());
}

#[test]
fn remove_parents() {
    let mut tag = sample_tag();
    let parents = sample_tags_list();
    assert!(!tag.has_parents());
    tag.add_parents(parents.clone());
    assert!(tag.has_parents());
    assert_eq!(tag.parents().len(),3);
    tag.remove_parents(parents);
    assert!(!tag.has_parents());
}

#[test]
fn remove_all_parents() {
    let mut tag = sample_tag();
    let parents = sample_tags_list();
    assert!(!tag.has_parents());
    tag.add_parents(parents.clone());
    assert!(tag.has_parents());
    assert_eq!(tag.parents().len(),3);
    tag.remove_all_parents();
    assert!(!tag.has_parents());
}