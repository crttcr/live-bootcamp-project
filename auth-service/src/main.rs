//use auth_service::services::data_stores::hashmap_user_store::HashmapUserStore;
use auth_service::utils::constants::{prod, DATABASE_URL, POSTMARK_AUTH_TOKEN, REDIS_HOST_NAME};
use auth_service::{app_state::AppState, create_postgres_pool, create_redis_client, Application};
use sqlx::PgPool;
use std::sync::Arc;
use color_eyre;
use redis::ConnectionLike;
use reqwest::Client;
use secrecy::Secret;
use tokio::sync::RwLock;
use auth_service::domain::Email;
use auth_service::services::data_stores::hashmap_2fa_code_store::HashmapTwoFACodeStore;
use auth_service::services::data_stores::redis_banned_token_store::RedisBannedTokenStore;
use auth_service::services::data_stores::postgres_user_store::PostgresUserStore;
//use auth_service::services::mock_email_client::MockEmailClient;
use auth_service::services::postmark_email_client::PostmarkEmailClient;
use auth_service::utils::tracing::init_tracing;

#[tokio::main]
async fn main()
{
	configure_logging();
	configure_tracing();
	
	let pg_pool        = configure_postgresql().await;
	let redis_cx       = configure_redis();
	let redis_cx       = Arc::new(RwLock::new(redis_cx));
	let user_store     = Arc::new(RwLock::new(PostgresUserStore::new(pg_pool)));
	let banned_tokens  = Arc::new(RwLock::new(RedisBannedTokenStore::new(redis_cx.clone())));
	let code_store     = Arc::new(RwLock::new(HashmapTwoFACodeStore::new()));
	let email_client   = Arc::new(RwLock::new(configure_postmark_email_client()));
	let app_state      = AppState::new(user_store, banned_tokens, code_store, email_client);
	let e_build        = "Failed to build application";
	let e_run          = "Failed to run application";
	let app            = Application::build(app_state, prod::APP_ADDRESS)
		.await
		.expect(e_build);
	app.run().await.expect(e_run);
}

fn configure_logging() { color_eyre::install().expect("Failed to install color_eyre"); }
fn configure_tracing() { init_tracing()       .expect("Failed to initialize tracing"); }

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
	let pg_pool   = create_postgres_pool(&DATABASE_URL).await.expect(e_pool);
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

fn configure_postmark_email_client() -> PostmarkEmailClient 
{
	let http_client = Client::builder()
		.timeout(prod::email_client::TIMEOUT)
		.build()
		.expect("Failed to build HTTP client");

	let base_url   = prod::email_client::BASE_URL.to_owned();
	let email      = prod::email_client::SENDER.to_owned();
	let email      = Secret::new(email);
	let sender     = Email::parse(email).unwrap();
	let auth_token = POSTMARK_AUTH_TOKEN.to_owned();
	PostmarkEmailClient::new(base_url, sender, auth_token, http_client)
}