//use axum::{response::Html, routing::get, Router};
//use tower_http::services::ServeDir;
use auth_service::{app_state::AppState, Application};
use auth_service::services::hashmap_user_store::HashmapUserStore;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let address    = "0.0.0.0:3000";
    let e_build    = "Failed to build application";
    let e_run      = "Failed to run application";
    let user_store = HashmapUserStore::default();
    let user_store = Arc::new(RwLock::new(user_store));
    let app_state  = AppState::new(user_store);
    let app        = Application::build(app_state, address)
        .await
        .expect(e_build);
    app.run().await.expect(e_run);
}
