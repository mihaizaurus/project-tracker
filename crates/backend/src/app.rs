use std::net::SocketAddr;
use axum::{Router,serve};
use tokio::net::TcpListener;
use crate::routes::create_router;

pub async fn run() {
    load_config();
    let router = create_router();
    prepare_services();
    launch_server(router).await;
}

fn load_config() {

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