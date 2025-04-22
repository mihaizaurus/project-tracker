use std::fs;
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
        let schema = fs::read_to_string("/crates/db/schemas/project.surql").map_err(|_| DatabaseError::SchemaError("schema file could not be read".into()))?;
        self.client.query(schema).await.map_err(|_| DatabaseError::QueryError("Database query could not be operated".into()))?;
        Ok(())
    }
}