#[cfg(test)]
mod tests {
    use project_tracker_core::builders::task_builder::TaskBuilder;

    #[test]
    fn create_task() {
        let task = TaskBuilder::new().with_name("This is a sample project title").build();
        assert_eq!(task.name(), "This is a sample project title");
    }
}