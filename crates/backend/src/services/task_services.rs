use crate::{
    dto::task_dto::TaskDTO,
    Result
};
use project_tracker_core::{
    factories::task_factory::*,
    models::task::Task
};

pub fn get_all_tasks() -> Vec<TaskDTO> {
    // TODO
    vec![
        TaskDTO::from(sample_task_for_dto()),
        TaskDTO::from(sample_task_for_dto()),
        TaskDTO::from(sample_task_for_dto()),
        TaskDTO::from(sample_task_for_dto()),
        TaskDTO::from(sample_task_for_dto()),
        TaskDTO::from(sample_task_for_dto()),
    ]
}

pub fn create_task(payload: TaskDTO) -> Result<Task> {
    /* TODO:
    1. [x] convert DTO to Task
    2. [ ] validate
    3. [ ] push to DB
    4. [ ] return success.failure
    */

    let task = Task::try_from(payload)?;

    Ok(task)
}