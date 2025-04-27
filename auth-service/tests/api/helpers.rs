use auth_service::app_state::{AppState, TokenStoreType};
use auth_service::services::hashmap_user_store::HashmapUserStore;
use auth_service::services::hashset_token_store::HashSetTokenStore;
use auth_service::utils::constants::test;
use auth_service::Application;
use reqwest::cookie::Jar;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct TestApp
{
	pub address:        String,
	pub banned_tokens:  TokenStoreType,
	pub cookie_jar:     Arc<Jar>,
	pub http_client:    reqwest::Client,
}

impl TestApp {
	pub async fn new() -> Self {
		let user_store    = HashmapUserStore::default();
		let user_store    = Arc::new(RwLock::new(user_store));
		let token_store   = HashSetTokenStore::new();
		let banned_tokens = Arc::new(RwLock::new(token_store));
		let app_state     = AppState::new(user_store, banned_tokens.clone());
		let app           = Application::build(app_state, test::APP_ADDRESS)
			.await
			.expect("Failed to build app");

		let cookie_jar = Arc::new(Jar::default());
		let address    = format!("http://{}", app.address.clone());

		// Run the auth service in a separate async task
		// to avoid blocking the main test thread.
		#[allow(clippy::let_underscore_future)]
		let _             = tokio::spawn(app.run());
		let http_client   = reqwest::Client::builder()
			.cookie_provider(cookie_jar.clone())
			.build()
			.expect("Failed to build http client");
		Self{address, banned_tokens, cookie_jar, http_client}
	}

	pub async fn get_root(&self) -> reqwest::Response {
		self.http_client
			.get(&format!("{}/", &self.address))
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
		let rv = self.http_client
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

	pub async fn post_verify_2fa(&self) -> reqwest::Response 
	{
		let url = format!("{}/verify-2fa", &self.address);
		self.http_client
			.post(url)
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

pub fn get_random_email() -> String {
	format!("{}@example.com", Uuid::new_v4())
}
