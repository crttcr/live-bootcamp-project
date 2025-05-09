use auth_service::app_state::{AppState, TokenStoreType, TwoFactorCodeStoreType};
use auth_service::services::data_stores::hashmap_2fa_code_store::HashmapTwoFACodeStore;
use auth_service::services::data_stores::hashset_token_store::HashSetTokenStore;
use auth_service::services::data_stores::postgres_user_store::PostgresUserStore;
use auth_service::services::mock_email_client::MockEmailClient;
use auth_service::utils::constants::{test, DATABASE_URL};
use auth_service::Application;
use reqwest::cookie::Jar;
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Executor, PgPool};
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
}

impl TestApp {
	pub async fn new() -> Self {
		let pg_pool           = configure_postgresql().await;
		let user_store        = PostgresUserStore::new(pg_pool);
		let user_store        = Arc::new(RwLock::new(user_store));
		let token_store       = HashSetTokenStore::new();
		let banned_tokens     = Arc::new(RwLock::new(token_store));
		let two_fa_code_store = HashmapTwoFACodeStore::new();
		let two_fa_code_store = Arc::new(RwLock::new(two_fa_code_store));
		let email_client      = Arc::new(RwLock::new(MockEmailClient::new()));
		let app_state         = AppState::new(user_store, banned_tokens.clone(), two_fa_code_store.clone(), email_client);
		let app               = Application::build(app_state, test::APP_ADDRESS)
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
		Self{address, banned_tokens, cookie_jar, two_fa_code_store, http_client}
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

pub fn get_random_email() -> String { format!("{}@example.com", Uuid::new_v4()) }

async fn configure_postgresql() -> PgPool {
	let e_create  = "Failed to create Postgres connection pool!";
	let pg_cx_url = DATABASE_URL.to_owned();
	let db_name   = Uuid::new_v4().to_string();

	// Create new database for each test case with a unique name.
	configure_database(&pg_cx_url, &db_name).await;

	let url = format!("{}/{}", pg_cx_url, db_name);
	let url = url.as_str();
	PgPoolOptions::new().max_connections(3).connect(url).await.expect(e_create)
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
