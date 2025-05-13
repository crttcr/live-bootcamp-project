use auth_service::app_state::{AppState, TokenStoreType, TwoFactorCodeStoreType};
use auth_service::services::data_stores::postgres_user_store::PostgresUserStore;
use auth_service::services::data_stores::redis_2fa_code_store::RedisTwoFACodeStore;
use auth_service::services::data_stores::redis_banned_token_store::RedisBannedTokenStore;
use auth_service::services::mock_email_client::MockEmailClient;
use auth_service::utils::constants::{test, DATABASE_URL, DEFAULT_REDIS_HOSTNAME};
use auth_service::{create_redis_client, Application};
use reqwest::cookie::Jar;
use secrecy::ExposeSecret;
use serde::Serialize;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct TestApp
{
	pub address:           String,
	pub banned_tokens:     TokenStoreType,
	pub cookie_jar:        Arc<Jar>,
	pub two_fa_code_store: TwoFactorCodeStoreType,
	pub http_client:       reqwest::Client,
	pub db_name:           String,
	pub clean_up_called:   bool,
}

impl TestApp {
	pub async fn new() -> Self {
		let (pg_pool, db_name) = configure_postgresql().await;
		let user_store         = PostgresUserStore::new(pg_pool);
		let user_store         = Arc::new(RwLock::new(user_store));
		let redis_cx           = Arc::new(RwLock::new(configure_redis()));
		let banned_tokens      = Arc::new(RwLock::new(RedisBannedTokenStore::new(redis_cx.clone())));
		let two_fa_code_store  = Arc::new(RwLock::new(RedisTwoFACodeStore::new(redis_cx)));
		let email_client       = Arc::new(RwLock::new(MockEmailClient::new()));
		let app_state          = AppState::new(user_store, banned_tokens.clone(), two_fa_code_store.clone(), email_client);
		let app                = Application::build(app_state, test::APP_ADDRESS)
			.await
			.expect("Failed to build app");

		// Have to gather the address before we run the app.
		let address = format!("http://{}", app.address.clone());

		// Run the auth service in a separate async task
		// to avoid blocking the main test thread.
		//
		#[allow(clippy::let_underscore_future)]
		let _             = tokio::spawn(app.run());
		let cookie_jar    = Arc::new(Jar::default());
		let http_client   = reqwest::Client::builder()
			.cookie_provider(cookie_jar.clone())
			.build()
			.expect("Failed to build an http client");
		let clean_up_called = false;
		Self{
			address,
			banned_tokens,
			cookie_jar,
			two_fa_code_store,
			http_client,
			db_name,
			clean_up_called,
		}
	}

	pub async fn clean_up(&mut self) {
		if self.clean_up_called {
			println!("Cleanup already called.");
			return
		}
		delete_database(self.db_name.as_str()).await;
		self.clean_up_called = true;
	}

	pub async fn get_root(&self) -> reqwest::Response {
		let address = format!("{}/", &self.address);
		self.http_client
			.get(&address)
			.send()
			.await
			.expect("Failed to execute root request.")
	}

	pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response 
	where
		Body: Serialize,
	{
		let url = format!("{}/signup", &self.address);
		self.http_client
			.post(url)
			.json(body)
			.send()
			.await
			.expect("Failed to execute signup request.")
	}

	pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response 
	where
		Body: Serialize + std::fmt::Debug,
	{
		println!("Calling post_login");
		println!("Body: {:?}", body);
		let url = format!("{}/login", &self.address);
		let rv  = self.http_client
			.post(url)
			.json(body)
			.send()
			.await
			.expect("Failed to execute login request.");
		println!("Returning {}", rv.status());
		rv
	}

	pub async fn post_logout(&self) -> reqwest::Response {
		let url = format!("{}/logout", &self.address);
		self.http_client
			.post(url)
			.send()
			.await
			.expect("Failed to execute logout request.")
	}

	pub async fn post_verify_2fa<Body>(&self, body: &Body) -> reqwest::Response
		where Body: Serialize
	{
		let url = format!("{}/verify-2fa", &self.address);
		self.http_client
			.post(url)
			.json(body)
			.send()
			.await
			.expect("Failed to execute verify_2fa request.")
	}

	pub async fn post_verify_token<Body>(&self, body: &Body) -> reqwest::Response 
		where Body: Serialize
	{
		let url = format!("{}/verify-token", &self.address);
		self.http_client
			.post(url)
			.json(body)
			.send()
			.await
			.expect("Failed verify_token request.")
	}
}

impl Drop for TestApp {
	fn drop(&mut self) {
		if !self.clean_up_called {
			panic!("TestApp was not cleaned up!");
		}
	}
}
pub fn get_random_email() -> String { format!("{}@example.com", Uuid::new_v4()) }

async fn configure_postgresql() -> (PgPool, String) {
	let e_create  = "Failed to create Postgres connection pool!";
	let db_name   = Uuid::new_v4().to_string();

	// Create a new database for each test case with a unique name.
	//
	let pg_cx_url = DATABASE_URL.expose_secret().as_str();
	configure_database(pg_cx_url, &db_name).await;
	let url       = format!("{}/{}", pg_cx_url, db_name);
	let url       = url.as_str();
	let pool      = PgPoolOptions::new()
		.max_connections(3)
		.connect(url).await
		.expect(e_create);
	(pool, db_name)
}

async fn configure_database(db_conn_string: &str, db_name: &str) {
	let e_create_database = "Failed to create the database.";
	let e_create_pool     = "Failed to create the Postgres connection pool.";
	let e_migrate         = "Failed to migrate the database";

	// Create connection
	let connection = PgPoolOptions::new()
		.connect(db_conn_string)
		.await
		.expect(e_create_pool);

	// Create database
	let stmt = format!(r#"CREATE DATABASE "{}";"#, db_name);
	connection
		.execute(stmt.as_str())
		.await
		.expect(e_create_database);

	// Connect
	let db_conn_string = format!("{}/{}", db_conn_string, db_name);
	let connection     = PgPoolOptions::new()
		.connect(&db_conn_string)
		.await
		.expect(e_create_pool);

	// Run migrations
	sqlx::migrate!()
		.run(&connection)
		.await
		.expect(e_migrate);
}

async fn delete_database(db_name: &str) {
	let e_drop_database = "Failed to drop the database.";
	let e_parse_cx_str  = "Failed to parse the connection string.";
	let e_connect       = "Failed to connect to Postgres.";
	let e_kill_cx       = "Failed to kill any active connections.";
	let pg_cx_url       = DATABASE_URL.expose_secret().as_str();
	let cx_options      = PgConnectOptions::from_str(pg_cx_url).expect(e_parse_cx_str);
	let mut cx          = PgConnection::connect_with(&cx_options).await.expect(e_connect);
	let cmd_drop_db     = format!(r#"DROP DATABASE "{}";"#, db_name);
	let cmd_kill_active = format!(r#"
                SELECT pg_terminate_backend(pg_stat_activity.pid)
                FROM  pg_stat_activity
                WHERE pg_stat_activity.datname = '{}'
                  AND pid <> pg_backend_pid();
      "#, db_name );

	// Kill any active connections, then
	// Drop the database
	cx.execute(cmd_kill_active.as_str()).await.expect(e_kill_cx);
	cx.execute(cmd_drop_db.as_str()    ).await.expect(e_drop_database);
}


fn configure_redis() -> redis::Connection {
	let e_client = "Failed to get Redis client.";
	let e_cx     = "Failed to get Redis connection.";
	let redis_hostname = DEFAULT_REDIS_HOSTNAME.to_owned();
	create_redis_client(redis_hostname)
		.expect(e_client)
		.get_connection()
		.expect(e_cx)
}