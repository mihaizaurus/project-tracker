use std::sync::Arc;
use project_tracker_core::{
    models::task::Task,
    models::project::ProjectStatus,
    models::person::Person,
    builders::task_builder::TaskBuilder,
    id::Id,
    HasId,
};
use project_tracker_db::{
    database::Database,
    task_repository::{ProdTaskRepository, TaskRepository},
};
use chrono::Utc;

async fn setup_test_db() -> Arc<Database> {
    Arc::new(Database::connect().await.expect("Failed to connect to test database"))
}

#[tokio::test]
async fn test_create_and_get_task() {
    let db = setup_test_db().await;
    let repo = ProdTaskRepository::new(db.clone());
    
    // Create a test task
    let task = TaskBuilder::new()
        .with_name("Test Task")
        .with_description("A test task description")
        .with_status(ProjectStatus::InProgress)
        .build();
    
    let task_id = task.id();
    
    // Test create
    let create_result = repo.create(task.clone()).await;
    assert!(create_result.is_ok(), "Failed to create task: {:?}", create_result);
    
    // Test get by id
    let get_result = repo.get_by_id(task_id).await;
    assert!(get_result.is_ok(), "Failed to get task: {:?}", get_result);
    
    let retrieved_task = get_result.unwrap();
    assert!(retrieved_task.is_some(), "Task not found after creation");
    
    let retrieved = retrieved_task.unwrap();
    assert_eq!(retrieved.name(), "Test Task");
}

#[tokio::test]
async fn test_get_nonexistent_task() {
    let db = setup_test_db().await;
    let repo = ProdTaskRepository::new(db.clone());
    
    let nonexistent_id = Id::<Task>::new();
    
    let result = repo.get_by_id(nonexistent_id).await;
    assert!(result.is_ok(), "Failed to query for nonexistent task: {:?}", result);
    assert!(result.unwrap().is_none(), "Should return None for nonexistent task");
}

#[tokio::test]
async fn test_task_with_complex_data() {
    let db = setup_test_db().await;
    let repo = ProdTaskRepository::new(db.clone());
    
    // Create a task with all fields populated
    let owner_id = Id::<Person>::new();
    let dependency_id = Id::<Task>::new();
    let child_id = Id::<Task>::new();
    
    let task = TaskBuilder::new()
        .with_name("Complex Task")
        .with_description("A complex task with all fields")
        .with_owner_id(Some(owner_id))
        .with_start_date(Some(Utc::now()))
        .with_due_date(Some(Utc::now() + chrono::Duration::days(7)))
        .with_dependencies(vec![dependency_id])
        .with_children(vec![child_id])
        .with_status(ProjectStatus::Planned)
        .build();
    
    let task_id = task.id();
    
    // Create and retrieve
    let create_result = repo.create(task.clone()).await;
    assert!(create_result.is_ok(), "Failed to create complex task: {:?}", create_result);
    
    let get_result = repo.get_by_id(task_id).await;
    assert!(get_result.is_ok());
    
    let retrieved = get_result.unwrap().unwrap();
    assert_eq!(retrieved.name(), "Complex Task");
    assert!(retrieved.has_owner());
    assert!(retrieved.start_date().is_some());
    assert!(retrieved.due_date().is_some());
}

#[tokio::test]
async fn test_multiple_tasks() {
    let db = setup_test_db().await;
    let repo = ProdTaskRepository::new(db.clone());
    
    // Create multiple tasks
    let task1 = TaskBuilder::new().with_name("Task 1").build();
    let task2 = TaskBuilder::new().with_name("Task 2").build();
    let task3 = TaskBuilder::new().with_name("Task 3").build();
    
    // Store all tasks
    assert!(repo.create(task1.clone()).await.is_ok());
    assert!(repo.create(task2.clone()).await.is_ok());
    assert!(repo.create(task3.clone()).await.is_ok());
    
    // Retrieve each task
    assert!(repo.get_by_id(task1.id()).await.unwrap().is_some());
    assert!(repo.get_by_id(task2.id()).await.unwrap().is_some());
    assert!(repo.get_by_id(task3.id()).await.unwrap().is_some());
}

#[tokio::test]
async fn test_task_hierarchy() {
    let db = setup_test_db().await;
    let repo = ProdTaskRepository::new(db.clone());
    
    // Create parent and child tasks
    let child_task1 = TaskBuilder::new().with_name("Child Task 1").build();
    let child_task2 = TaskBuilder::new().with_name("Child Task 2").build();
    
    // Store child tasks first
    assert!(repo.create(child_task1.clone()).await.is_ok());
    assert!(repo.create(child_task2.clone()).await.is_ok());
    
    // Create parent task with children
    let parent_task = TaskBuilder::new()
        .with_name("Parent Task")
        .with_children(vec![child_task1.id(), child_task2.id()])
        .build();
    
    assert!(repo.create(parent_task.clone()).await.is_ok());
    
    // Retrieve parent task and verify children
    let retrieved = repo.get_by_id(parent_task.id()).await.unwrap().unwrap();
    assert_eq!(retrieved.children().len(), 2);
}

#[tokio::test]
async fn test_task_status_transitions() {
    let db = setup_test_db().await;
    let repo = ProdTaskRepository::new(db.clone());
    
    // Test different status values
    let statuses = vec![
        ProjectStatus::NotStarted,
        ProjectStatus::Planned,
        ProjectStatus::InProgress,
        ProjectStatus::InReview,
        ProjectStatus::Completed,
        ProjectStatus::Archived,
        ProjectStatus::Canceled,
    ];
    
    for (i, status) in statuses.iter().enumerate() {
        let task = TaskBuilder::new()
            .with_name(&format!("Task with status {}", i))
            .with_status(status.clone())
            .build();
        
        assert!(repo.create(task.clone()).await.is_ok());
        
        let retrieved = repo.get_by_id(task.id()).await.unwrap().unwrap();
        assert_eq!(&retrieved.status(), status);
    }
}