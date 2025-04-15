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