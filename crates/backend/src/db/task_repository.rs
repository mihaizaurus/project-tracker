use std::sync::Arc;
use project_tracker_core::{id::Id, models::task::Task, HasId};
use crate::{Result, Error};
use project_tracker_db::database::Database as ProdDatabase;
use project_tracker_db_mock::database::Database as MockDatabase;
use async_trait::async_trait;

// Traits
#[async_trait]
pub trait TaskRepository {
    async fn create(&self, task: Task) -> Result<()>;
    async fn get_by_id(&self, id: Id<Task>) -> Result<Option<Task>>;
}

// region: Actual DB
pub struct ProdTaskRepository {
    db: Arc<ProdDatabase>
}

impl ProdTaskRepository {
    pub fn new(db: Arc<ProdDatabase>) -> Self {
        Self { db } 
    }
}

#[async_trait]
impl TaskRepository for ProdTaskRepository {
    async fn create(&self, task: Task) -> Result<()> {
        use project_tracker_db::task_repository::ProdTaskRepository as DbTaskRepository;
        use project_tracker_db::task_repository::TaskRepository as DbTaskRepositoryTrait;
        
        let db_repo = DbTaskRepository::new(self.db.clone());
        db_repo.create(task).await.map_err(Error::DatabaseError)
    }

    async fn get_by_id(&self, id: Id<Task>) -> Result<Option<Task>> {
        use project_tracker_db::task_repository::ProdTaskRepository as DbTaskRepository;
        use project_tracker_db::task_repository::TaskRepository as DbTaskRepositoryTrait;
        
        let db_repo = DbTaskRepository::new(self.db.clone());
        db_repo.get_by_id(id).await.map_err(Error::DatabaseError)
    }
}

// endregion: Actual DB
// region: Mock db for testing
pub struct MockTaskRepository {
    db: Arc<MockDatabase>
}

impl MockTaskRepository {
    pub fn new(db: Arc<MockDatabase>) -> Self {
        Self { db } 
    }
}

#[async_trait]
impl TaskRepository for MockTaskRepository {
    async fn create(&self, task: Task) -> Result<()> {
        let id_str = task.id().to_string();
        println!("Successfully created task in database");
        Ok(())
    }

    async fn get_by_id(&self, id: Id<Task>) -> Result<Option<Task>> {
        // query db for the task by its id
        Ok(None)
    }
}
// endregion: Mock db for testing