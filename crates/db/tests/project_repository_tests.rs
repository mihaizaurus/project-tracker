use chrono::Utc;
use project_tracker_core::{
    HasId,
    builders::project_builder::ProjectBuilder,
    id::Id,
    models::{
        person::Person,
        project::Project,
        schedulable::{Schedulable, SchedulableItemStatus},
    },
};
use project_tracker_db::{
    database::Database,
    project_repository::{ProdProjectRepository, ProjectRepository},
};
use std::sync::Arc;

async fn setup_test_db() -> Arc<Database> {
    Arc::new(
        Database::connect()
            .await
            .expect("Failed to connect to test database"),
    )
}

#[tokio::test]
async fn test_create_and_get_project() {
    let db = setup_test_db().await;
    let repo = ProdProjectRepository::new(db.clone());

    // Create a test project
    let project = ProjectBuilder::new()
        .with_name("Test Project")
        .with_description("A test project description")
        .with_status(SchedulableItemStatus::InProgress)
        .build();

    let project_id = project.id();

    // Test create
    let create_result = repo.create(project.clone()).await;
    assert!(
        create_result.is_ok(),
        "Failed to create project: {:?}",
        create_result
    );

    // Test get by id
    let get_result = repo.get_by_id(project_id).await;
    assert!(
        get_result.is_ok(),
        "Failed to get project: {:?}",
        get_result
    );

    let retrieved_project = get_result.unwrap();
    assert!(
        retrieved_project.is_some(),
        "Project not found after creation"
    );

    let retrieved = retrieved_project.unwrap();
    assert_eq!(retrieved.name(), "Test Project");
}

#[tokio::test]
async fn test_get_nonexistent_project() {
    let db = setup_test_db().await;
    let repo = ProdProjectRepository::new(db.clone());

    let nonexistent_id = Id::<Project>::new();

    let result = repo.get_by_id(nonexistent_id).await;
    assert!(
        result.is_ok(),
        "Failed to query for nonexistent project: {:?}",
        result
    );
    assert!(
        result.unwrap().is_none(),
        "Should return None for nonexistent project"
    );
}

#[tokio::test]
async fn test_project_with_complex_data() {
    let db = setup_test_db().await;
    let repo = ProdProjectRepository::new(db.clone());

    // Create a project with all fields populated
    let owner_id = Id::<Person>::new();
    let dependency_id = Id::<Project>::new();

    let project = ProjectBuilder::new()
        .with_name("Complex Project")
        .with_description("A complex project with all fields")
        .with_owner_id(Some(owner_id))
        .with_start_date(Some(Utc::now()))
        .with_due_date(Some(Utc::now() + chrono::Duration::days(30)))
        .with_dependencies(vec![dependency_id])
        .with_status(SchedulableItemStatus::Planned)
        .build();

    let project_id = project.id();

    // Create and retrieve
    let create_result = repo.create(project.clone()).await;
    assert!(
        create_result.is_ok(),
        "Failed to create complex project: {:?}",
        create_result
    );

    let get_result = repo.get_by_id(project_id).await;
    assert!(get_result.is_ok());

    let retrieved = get_result.unwrap().unwrap();
    assert_eq!(retrieved.name(), "Complex Project");
    assert!(retrieved.has_owner());
    assert!(retrieved.start_date().is_some());
    assert!(retrieved.due_date().is_some());
}

#[tokio::test]
async fn test_multiple_projects() {
    let db = setup_test_db().await;
    let repo = ProdProjectRepository::new(db.clone());

    // Create multiple projects
    let project1 = ProjectBuilder::new().with_name("Project 1").build();
    let project2 = ProjectBuilder::new().with_name("Project 2").build();
    let project3 = ProjectBuilder::new().with_name("Project 3").build();

    // Store all projects
    assert!(repo.create(project1.clone()).await.is_ok());
    assert!(repo.create(project2.clone()).await.is_ok());
    assert!(repo.create(project3.clone()).await.is_ok());

    // Retrieve each project
    assert!(repo.get_by_id(project1.id()).await.unwrap().is_some());
    assert!(repo.get_by_id(project2.id()).await.unwrap().is_some());
    assert!(repo.get_by_id(project3.id()).await.unwrap().is_some());
}

#[tokio::test]
async fn test_project_serialization_deserialization() {
    let db = setup_test_db().await;
    let repo = ProdProjectRepository::new(db.clone());

    // Create a project with special characters in name and description
    let project = ProjectBuilder::new()
        .with_name("Project with 'quotes' and \"double quotes\"")
        .with_description("Description with\nnewlines\tand\ttabs")
        .build();

    let project_id = project.id();

    // Create and retrieve
    assert!(repo.create(project.clone()).await.is_ok());

    let retrieved = repo.get_by_id(project_id).await.unwrap().unwrap();
    assert_eq!(
        retrieved.name(),
        "Project with 'quotes' and \"double quotes\""
    );
    assert!(!retrieved.description().is_empty());
    assert!(retrieved.description().contains("newlines"));
    assert!(retrieved.description().contains("tabs"));
}

