//use axum::{response::Html, routing::get, Router};
//use tower_http::services::ServeDir;
use auth_service::Application;

#[tokio::main]
async fn main() {
    let address = "0.0.0.0:3000";
    let app       = Application::build(address)
        .await
        .expect("failed to build application");

    app.run().await.expect("failed to run application");
}