use super::constants::{JWT_COOKIE_NAME, JWT_SECRET, TOKEN_TTL_SECONDS};
use crate::domain::email::Email;
use crate::domain::TokenStore;
use axum_extra::extract::cookie::{Cookie, SameSite};
use chrono::Utc;
use jsonwebtoken;
use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};

// Create cookie with a new JWT auth token
//
pub fn generate_auth_cookie(email: &Email) -> Result<Cookie<'static>, GenerateTokenError> {
	let token = generate_auth_token(email)?;
	Ok(create_auth_cookie(token))
}

// Create cookie and set the value to the passed-in token string
//
pub fn create_auth_cookie(token: String) -> Cookie<'static> {
	let cookie = Cookie::build((JWT_COOKIE_NAME, token))
		.path("/")                       // apply cookie to all URLs on the server
		.http_only(true)                 // prevent JavaScript from accessing the cookie
		.same_site(SameSite::Lax)        // send cookie with "same-site" requests, and with "cross-site" top-level navigations.
		.build();
	cookie
}

#[derive(Debug)]
pub enum GenerateTokenError
{
	TokenError(jsonwebtoken::errors::Error),
	UnexpectedError,
}

// This value determines how long the JWT auth token is valid for
//

// Create JWT auth token
pub fn generate_auth_token(email: &Email) -> Result<String, GenerateTokenError> {
	let delta = chrono::Duration::try_seconds(TOKEN_TTL_SECONDS).ok_or(GenerateTokenError::UnexpectedError)?;

	// Create JWT expiration time
	let exp = Utc::now()
		.checked_add_signed(delta)
		.ok_or(GenerateTokenError::UnexpectedError)?
		.timestamp();

	// Cast exp to a usize, which is what Claims expects
	let exp: usize = exp
		.try_into()
		.map_err(|_| GenerateTokenError::UnexpectedError)?;

	let sub    = email.as_ref().to_owned();
	let claims = Claims {sub, exp};
	create_token(&claims).map_err(GenerateTokenError::TokenError)
}

// Check if JWT auth token is valid by decoding it using the JWT secret
//
pub async fn validate_token(token: &str) -> Result<Claims, Error> {
	println!("Validating token\n\t{:?}", token);

	/*
	if let Ok(true) = banned_tokens.token_exists(token).await {
		println!("Token is banned: {:?}", token);
		return Err(Error::from(ErrorKind::InvalidAlgorithmName));
	}
	*/

	let key        = DecodingKey::from_secret(JWT_SECRET.as_bytes());
	let validation = &Validation::default();
	let data       = jsonwebtoken::decode::<Claims>(token, &key, &validation);
	let claims     = data.map(|data| data.claims);
	println!("\t{:?}", claims);
	claims
}

// Create JWT auth token by encoding claims using the JWT secret
//
fn create_token(claims: &Claims) -> Result<String, jsonwebtoken::errors::Error> {
	jsonwebtoken::encode(
		&jsonwebtoken::Header::default(),
		&claims,
		&EncodingKey::from_secret(JWT_SECRET.as_bytes()),
	)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
	pub sub: String,
	pub exp: usize,
}
