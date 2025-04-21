use surrealdb::{
    Surreal,
    engine::local::{Mem,Db},
    opt::auth::Root
};

pub struct Database {
    client: Surreal<Db>
}

impl Database {
    pub async fn connect() -> surrealdb::Result<Self> {
        // memory:// for in-memory DB
        let client = Surreal::new::<Mem>(()).await?;

        // client.signin(Root {
        //     username: "root",
        //     password: "root",
        // }).await?;

        client.use_ns("namespace").use_db("database").await?;

        Ok(Self { client })
    }
}