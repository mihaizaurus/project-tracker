use project_tracker_backend::app;

#[tokio::main]
async fn main() {
    app::run().await;
}