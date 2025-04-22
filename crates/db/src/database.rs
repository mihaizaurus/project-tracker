use std::fs;
use surrealdb::{
    Surreal,
    engine::local::{Mem,Db},
    // opt::auth::Root
};
use crate::{Result, Error};

pub struct Database {
    client: Surreal<Db>
}

impl Database {
    pub async fn connect() -> surrealdb::Result<Self> {
        // memory:// for in-memory DB
        let client = Surreal::new::<Mem>(()).await?;

        // client.signin(Root { // Not Relevant for in Mem DB
        //     username: "root",
        //     password: "root",
        // }).await?;

        client.use_ns("namespace").use_db("database").await?;

        Ok(Self { client })
    }
    // pub async fn initialise_schema(&self) -> Result<&Self> {
    //     let schema = fs::read_to_string("/crates/db/schemas/project.surql").map_err(|_| Error::DatabaseError)?;
    //     self.client.query(schema).await?;
    //     Ok(self)
    // }
}