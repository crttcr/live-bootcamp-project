//use axum::{response::Html, routing::get, Router};
//use tower_http::services::ServeDir;
use auth_service::Application;

#[tokio::main]
async fn main() {
    let address = "0.0.0.0:3000";
    let e_build = "Failed to build application";
    let e_run   = "Failed to run application";
    let app     = Application::build(address)
        .await
        .expect(e_build);
    app.run().await.expect(e_run);
}
