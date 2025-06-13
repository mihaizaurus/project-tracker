use std::sync::Arc;
use project_tracker_core::{id::Id, models::project::Project, HasId};
use crate::{Result, Error};
use project_tracker_db::database::Database as ProdDatabase;
use project_tracker_db_mock::database::Database as MockDatabase;
use async_trait::async_trait;

// Traits
#[async_trait]
pub trait ProjectRepository { // can serve db and db-mock
    async fn create(&self, project: Project) -> Result<()>;
    async fn get_by_id(&self, id: Id<Project>) -> Result<Option<Project>>;
}

// region: Actual DB
pub struct ProdProjectRepository {
    db: Arc<ProdDatabase>
}

impl ProdProjectRepository {
    pub fn new(db: Arc<ProdDatabase>) -> Self {
        Self { db } 
    }
}

#[async_trait]
impl ProjectRepository for ProdProjectRepository {
    async fn create(&self, project: Project) -> Result<()> {
        use project_tracker_db::project_repository::ProdProjectRepository as DbProjectRepository;
        use project_tracker_db::project_repository::ProjectRepository as DbProjectRepositoryTrait;
        
        let db_repo = DbProjectRepository::new(self.db.clone());
        db_repo.create(project).await.map_err(Error::DatabaseError)
    }

    async fn get_by_id(&self, id: Id<Project>) -> Result<Option<Project>> {
        use project_tracker_db::project_repository::ProdProjectRepository as DbProjectRepository;
        use project_tracker_db::project_repository::ProjectRepository as DbProjectRepositoryTrait;
        
        let db_repo = DbProjectRepository::new(self.db.clone());
        db_repo.get_by_id(id).await.map_err(Error::DatabaseError)
    }
}

// endregion: Actual DB
// region: Mock db for testing
pub struct MockProjectRepository {
    db: Arc<MockDatabase>
}

impl MockProjectRepository {
    pub fn new(db: Arc<MockDatabase>) -> Self {
        Self { db } 
    }
}

#[async_trait]
impl ProjectRepository for MockProjectRepository {
    async fn create(&self, project: Project) -> Result<()> {
        let id_str = project.id().to_string();
        println!("Succesfully created project in database");
        Ok(())
    }

    async fn get_by_id(&self, id: Id<Project>) -> Result<Option<Project>> {
        // query db for the project by its id
        Ok(None)
    }
}
// endregion: Mock db for testing

// TODO: Hold a reference to the database client

