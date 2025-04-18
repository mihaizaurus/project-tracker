use axum::{Router,routing::get};

pub fn routes() -> Router {
    Router::new().route("/projects",get(projects))
}

async fn projects() -> &'static str {
    "OK"
}
