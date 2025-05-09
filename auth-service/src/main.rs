//use auth_service::services::data_stores::hashmap_user_store::HashmapUserStore;
use auth_service::utils::constants::{prod, DATABASE_URL};
use auth_service::{app_state::AppState, create_postgres_pool, Application};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;
use auth_service::services::data_stores::hashmap_2fa_code_store::HashmapTwoFACodeStore;
use auth_service::services::data_stores::hashset_token_store::HashSetTokenStore;
use auth_service::services::data_stores::postgres_user_store::PostgresUserStore;
use auth_service::services::mock_email_client::MockEmailClient;

#[tokio::main]
async fn main()
{
	let e_build        = "Failed to build application";
	let e_run          = "Failed to run application";
	let pg_pool        = configure_postgresql().await;
	let user_store     = Arc::new(RwLock::new(PostgresUserStore::new(pg_pool)));
	let banned_tokens  = Arc::new(RwLock::new(HashSetTokenStore::new()));
	let code_store     = Arc::new(RwLock::new(HashmapTwoFACodeStore::new()));
	let email_client   = Arc::new(RwLock::new(MockEmailClient::new()));
	let app_state      = AppState::new(user_store, banned_tokens, code_store, email_client);
	let app            = Application::build(app_state, prod::APP_ADDRESS)
		.await
		.expect(e_build);
	app.run().await.expect(e_run);
}

// Configuring a Postgres connection pool means:
//
// * Create the pool
// * Running database migrations against the test database
// * Return the pool
//
async fn configure_postgresql() -> PgPool
{
	let e_pool    = "Failed to create Postgres connection pool";
	let e_migrate = "Failed to run migrations";
	let url       = &DATABASE_URL;
	let pg_pool   = create_postgres_pool(url).await.expect(e_pool);
	sqlx::migrate!().run(&pg_pool).await.expect(e_migrate);
	pg_pool
}
