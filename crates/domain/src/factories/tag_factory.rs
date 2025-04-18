use crate::models::tag::Tag;
use crate::builders::tag_builder::TagBuilder;
use crate::id::Id;
use crate::HasId;

pub fn basic_tag() -> Tag {
    TagBuilder::new().build()
}

// region: Factories for Tests

pub fn sample_tag() -> Tag {
    TagBuilder::new().with_name("SampleTag").with_description("This is a sample Tag").build()
}

pub fn sample_tags_list() -> Vec<Id<Tag>> {
    let (tag1,tag2,tag3) = (TagBuilder::new().build(),TagBuilder::new().build(),TagBuilder::new().build());
    vec![tag1.id(),tag2.id(),tag3.id()]
}

pub fn sample_child_tag() -> Tag {
    let (mut tag1,tag2,tag3) = (TagBuilder::new().build(),TagBuilder::new().build(),TagBuilder::new().build());
    tag1.add_parents(vec![tag2.id(),tag3.id()]);
    tag1
}

// endregion: Factories for Tests