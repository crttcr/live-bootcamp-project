use super::constants::{JWT_COOKIE_NAME, JWT_SECRET, TOKEN_TTL_SECONDS};
use crate::app_state::TokenStoreType;
use crate::domain::email::Email;
use color_eyre::eyre::{eyre, Context, ContextCompat, Result};
use axum_extra::extract::cookie::{Cookie, SameSite};
use chrono::Utc;
use color_eyre::Report;
use jsonwebtoken;
use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{debug, warn};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
	pub sub: String,
	pub exp: usize,
}

//
// TODO: Create our own error type independent from jsonwebtoken::errors::Error
//

#[derive(Debug, Error)]
pub enum GenerateTokenError
{
	#[error("Token error: {0}")]
	TokenError(Error),
	#[error("Unexpected error")]
	UnexpectedError(#[source] Report),
	#[error("Duration too long {0}")]
	DurationTooLong(String),
}

// Create cookie with a new JWT auth token
//
#[tracing::instrument(name = "generate auth cookie", skip_all)]
pub fn generate_auth_cookie(email: &Email) -> Result<Cookie<'static>> {
	let token  = generate_jwt_auth_token(email)?;
	let cookie = Cookie::build((JWT_COOKIE_NAME, token))
		.path("/")                       // apply cookie to all URLs on the server
		.http_only(true)                 // prevent JavaScript from accessing the cookie
		.same_site(SameSite::Lax)        // send cookie with "same-site" requests, and with "cross-site" top-level navigations.
		.build();
	Ok(cookie)
}

#[tracing::instrument(name = "generate JWT token", skip_all)]
pub fn generate_jwt_auth_token(email: &Email) -> Result<String> {
	let delta = chrono::Duration::try_seconds(TOKEN_TTL_SECONDS);
	let delta = delta.ok_or(eyre!("Too long to live"))
		.map_err(|e| GenerateTokenError::DurationTooLong(format!("{}", e)))?;

	let exp = Utc::now()                                    // Create JWT expiration time
		.checked_add_signed(delta)
		.ok_or(GenerateTokenError::UnexpectedError(eyre!("Bad time")))?
		.timestamp();

	let exp    = exp as usize;                              // Cast exp to usize, (what Claims expects)
	let sub    = email.as_ref().to_owned();
	let claims = Claims {sub, exp};
	create_token(&claims)
}

// Check if JWT auth-token is valid by decoding it using the JWT secret
//
#[tracing::instrument(name = "validate token", skip_all)]
pub async fn validate_token(
	token:           &str,
	banned_tokens:   TokenStoreType,
	) -> Result<Claims> {
	debug!("Validating token\n\t{:?}", token);
	match banned_tokens.read().await.contains_token(token).await {
		true => {
			warn!("Token is banned: {:?}", token);
			return Err(eyre!("Token is banned"))
		},
		_ => {}
	}

	let key        = DecodingKey::from_secret(JWT_SECRET.as_bytes());
	let validation = &Validation::default();
	let data       = jsonwebtoken::decode::<Claims>(token, &key, &validation);
	let claims     = data.map(|v| v.claims).wrap_err("Failed to decode token");
	debug!("\t{:?}", claims);
	claims
}

// Create JWT auth token by encoding claims using the JWT secret
//
#[tracing::instrument(name = "encode claims into token", skip_all)]
fn create_token(claims: &Claims) -> Result<String> {
	jsonwebtoken::encode(
		&jsonwebtoken::Header::default(),
		&claims,
		&EncodingKey::from_secret(JWT_SECRET.as_bytes()),
	).wrap_err("Failed to create token")
}
