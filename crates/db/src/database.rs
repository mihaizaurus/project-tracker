use surrealdb::{
    Surreal,
    engine::local::{Mem,Db},
    // opt::auth::Root
};
use crate::{Result, DatabaseError};

pub struct Database {
    client: Surreal<Db>
}

impl Database {
    pub async fn connect() -> Result<Self> {
        // memory:// for in-memory DB
        let client = Surreal::new::<Mem>(()).await.map_err(|_| DatabaseError::ConnectionError("Could not start SurrealDB client".into()))?;

        // client.signin(Root { // Not Relevant for in Mem DB
        //     username: "root",
        //     password: "root",
        // }).await?;

        client.use_ns("namespace").use_db("database").await.map_err(|_| DatabaseError::ConnectionError("Could not connect to namespace".into()))?;

        let db = Self {client};
        db.initialise_schema().await?;

        Ok(db)
    }

    pub async fn initialise_schema(&self) -> Result<()> {
        // Fix the hardcoded path - use relative path from the crate root
        let project_schema = include_str!("../schemas/project.surql");
        let task_schema = include_str!("../schemas/task.surql");
        
        self.client.query(project_schema).await
            .map_err(|e| DatabaseError::SchemaError(format!("Failed to initialize project schema: {}", e)))?;
        self.client.query(task_schema).await
            .map_err(|e| DatabaseError::SchemaError(format!("Failed to initialize task schema: {}", e)))?;
        
        Ok(())
    }
    
    pub fn client(&self) -> &Surreal<Db> {
        &self.client
    }
}