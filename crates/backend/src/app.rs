use std::{net::SocketAddr, sync::Arc};
use axum::{Router,serve};
use tokio::net::TcpListener;
use project_tracker_db::database::Database as ProdDatabase;
use project_tracker_db_mock::database::Database as MockDatabase;
use crate::{
    routes::create_router,
    db::project_repository,
    services::project_services
};

pub async fn run() {
    /* TODO: finalize back-end setup
    1. launch config
    2. prepare db connection
    3. prepare services ??
    4. create router
    5. launch back-end server
    */
    
    load_config(); // WIP

    let db = MockDatabase::connect().await.expect("Failed to connect to db");
    let db = Arc::new(db);

    let router = create_router();
    prepare_services();
    launch_server(router).await;
}

fn load_config() {
    // TODO: config definition for back-end
}

fn prepare_services() {

}

async fn launch_server(router: Router) {
    let addr = SocketAddr::from(([127,0,0,1],7878));
    
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");
    
    println!("Server listening on http://{}",listener.local_addr().unwrap());
    serve(listener,router).await.unwrap()
}