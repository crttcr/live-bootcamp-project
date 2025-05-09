use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env as std_env;

pub const JWT_COOKIE_NAME:   &str = "jwt";
pub const TOKEN_TTL_SECONDS: i64  = 600; // 10 minutes

pub mod prod {
	pub const APP_ADDRESS:   &str = "0.0.0.0:3000";
	pub const URI_APP:       &str = "https://161.35.106.43:8000";
	pub const URI_AUTH:      &str = "https://161.35.106.43:3000";
}

pub mod test {
	pub const APP_ADDRESS:   &str = "127.0.0.1:0";
	pub const URI_APP:       &str = "https://localhost:8000";
	pub const URI_AUTH:      &str = "https://localhost:3000";
}

lazy_static! {
	pub static ref JWT_SECRET:        String = set_token();
	pub static ref POSTGRES_PASSWORD: String = set_pg_password();
	pub static ref DATABASE_URL:      String = set_db_url();
}

fn set_db_url() -> String {
	let password = set_pg_password();
	let url      = format!("postgres://postgres:{}@localhost:5444", password);
	url
}

fn set_pg_password() -> String {
	dotenv().ok();
	let secret = std_env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD not set");
	if secret.is_empty() {
		panic!("POSTGRES_PASSWORD must not be empty. Set it in the .env file or in environment variables.");
	}
	if secret.len() < 8 {
		eprintln!("POSTGRES_PASSWORD is too short. It should be at least 8 characters long.")
	}
	secret
}

fn set_token() -> String {
	dotenv().ok();
	let secret = std_env::var("JWT_SECRET").expect("JWT_SECRET not set");
	if secret.is_empty() {
		panic!("JWT_SECRET must not be empty. Set it in the .env file or in environment variables.");
	}
	if secret.len() < 16 {
		eprintln!("JWT_SECRET is too short. It should be at least 16 characters long.")
	}
	secret
}

pub mod env {
	pub const JWT_SECRENT_ENV_VAR: &str = "JWT_SECRET";
}
