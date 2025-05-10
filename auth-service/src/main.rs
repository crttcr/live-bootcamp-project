//use auth_service::services::data_stores::hashmap_user_store::HashmapUserStore;
use auth_service::utils::constants::{prod, DATABASE_URL, REDIS_HOST_NAME};
use auth_service::{app_state::AppState, create_postgres_pool, create_redis_client, Application};
use sqlx::PgPool;
use std::sync::Arc;
use redis::ConnectionLike;
use tokio::sync::RwLock;
use auth_service::services::data_stores::hashmap_2fa_code_store::HashmapTwoFACodeStore;
use auth_service::services::data_stores::redis_banned_token_store::RedisBannedTokenStore;
use auth_service::services::data_stores::postgres_user_store::PostgresUserStore;
use auth_service::services::mock_email_client::MockEmailClient;
use auth_service::utils::tracing::init_tracing;

#[tokio::main]
async fn main()
{
	init_tracing();
	let e_build        = "Failed to build application";
	let e_run          = "Failed to run application";
	let pg_pool        = configure_postgresql().await;
	let redis_cx       = configure_redis();
	let redis_cx       = Arc::new(RwLock::new(redis_cx));
	let user_store     = Arc::new(RwLock::new(PostgresUserStore::new(pg_pool)));
	let banned_tokens  = Arc::new(RwLock::new(RedisBannedTokenStore::new(redis_cx.clone())));
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

fn configure_redis() -> redis::Connection
{
	let e_client  = "Failed to create Redis client";
	let e_connect = "Failed to connect to Redis";
	let host =  REDIS_HOST_NAME.to_owned();
	let redis = create_redis_client(host)	
		.expect(e_client)
		.get_connection()
		.expect(e_connect);
	println!("Connected to Redis: Open? {}", redis.is_open());
	redis
}