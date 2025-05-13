use axum_extra::extract::cookie::{Cookie, SameSite};
use chrono::Utc;
use crate::domain::email::Email;
use crate::services::data_stores::hashset_token_store::HashSetTokenStore;
use crate::utils::auth::*;
use crate::utils::constants::*;
use std::sync::Arc;
use secrecy::Secret;
use tokio::sync::RwLock;

fn assert_basic_cookie_properties(c: &Cookie) {
	assert_eq!(c.name(),      JWT_COOKIE_NAME);
	assert_eq!(c.path(),      Some("/"));
	assert_eq!(c.http_only(), Some(true));
	assert_eq!(c.same_site(), Some(SameSite::Lax));
}

#[tokio::test]
async fn test_generate_auth_cookie() {
	let email   = Secret::new("test@example.com".to_owned());
	let email  = Email::parse(email).unwrap();
	let cookie = generate_auth_cookie(&email).unwrap();
	assert_basic_cookie_properties(&cookie);
	assert_eq!(cookie.value().split('.').count(), 3);
}

#[tokio::test]
async fn test_generate_auth_token_result_has_3_parts() {
	let email   = Secret::new("test@example.com".to_owned());
	let email  = Email::parse(email).unwrap();
	let result = generate_jwt_auth_token(&email).unwrap();
	assert_eq!(result.split('.').count(), 3);
}

#[tokio::test]
async fn test_valid_token_passes_validation() {
	let email   = Secret::new("a@b.com".to_owned());
	let email         = Email::parse(email).unwrap();
	let token         = generate_jwt_auth_token(&email).unwrap();
	let banned_tokens =  Arc::new(RwLock::new(HashSetTokenStore::new()));
	let result        = validate_token(token.as_str(), banned_tokens).await.unwrap();
	assert_eq!(result.sub, "a@b.com");

	let exp = Utc::now()
		.checked_add_signed(chrono::Duration::try_minutes(9).expect("valid duration"))
		.expect("valid timestamp")
		.timestamp();
	assert!(result.exp > exp as usize);
}

#[tokio::test]
async fn test_invalid_token_fails_validation() {
	let token         = "invalid_token".to_owned();
	let banned_tokens = Arc::new(RwLock::new(HashSetTokenStore::new()));
	let result        = validate_token(&token, banned_tokens).await;
	assert!(result.is_err());
}
