use auth_service::services::hashmap_user_store::HashmapUserStore;
use auth_service::utils::constants::prod;
use auth_service::{app_state::AppState, Application};
use std::sync::Arc;
use tokio::sync::RwLock;
use auth_service::services::hashmap_2fa_code_store::HashmapTwoFACodeStore;
use auth_service::services::hashset_token_store::HashSetTokenStore;
use auth_service::services::mock_email_client::MockEmailClient;

#[tokio::main]
async fn main() {
    let e_build        = "Failed to build application";
    let e_run          = "Failed to run application";
    let user_store     = HashmapUserStore::default();
    let user_store     = Arc::new(RwLock::new(user_store));
    let token_store    = HashSetTokenStore::new();
    let banned_tokens  = Arc::new(RwLock::new(token_store));
    let code_store     = HashmapTwoFACodeStore::new();
    let code_store     = Arc::new(RwLock::new(code_store));
    let email_client   = Arc::new(RwLock::new(MockEmailClient::new()));
    let app_state      = AppState::new(user_store, banned_tokens, code_store, email_client);
    let app            = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect(e_build);
    app.run().await.expect(e_run);
}
