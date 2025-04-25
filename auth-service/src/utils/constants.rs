use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env as std_env;

pub const JWT_COOKIE_NAME:         &str = "jwt";
pub const TOKEN_TTL_SECONDS:       i64  = 600; // 10 minutes

pub mod prod {
	pub const APP_ADDRESS:            &str = "0.0.0.0:3000";
	pub const URI_APP:   &str = "https://161.35.106.43:8000";
}

pub mod test {
	pub const APP_ADDRESS: &str = "127.0.0.1:0";
	pub const URI_APP:     &str = "https://localhost:8000";
}

lazy_static! {
	pub static ref JWT_SECRET: String = set_token();
}

fn set_token() -> String {
	dotenv().ok();
	std_env::var("JWT_SECRET").unwrap();
	let secret = std_env::var("JWT_SECRET").expect("JWT_SECRET not set");
	if secret.is_empty() {
		panic!("JWT_SECRET mukkst not be empty. Set it in the .env file or in environment variables.");
	}
	if secret.len() < 16 {
		eprintln!("JWT_SECRET is too short. It should be at least 16 characters long.")
	}
	secret
}

pub mod env {
	pub const JWT_SECRENT_ENV_VAR: &str = "JWT_SECRET";
}