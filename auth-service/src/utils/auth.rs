use super::constants::{JWT_COOKIE_NAME, JWT_SECRET, TOKEN_TTL_SECONDS};
use crate::app_state::TokenStoreType;
use crate::domain::email::Email;
use axum_extra::extract::cookie::{Cookie, SameSite};
use chrono::Utc;
use jsonwebtoken;
use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
	pub sub: String,
	pub exp: usize,
}

//
// TODO: Create our own error type independent from jsonwebtoken::errors::Error
//

#[derive(Debug)]
pub enum GenerateTokenError
{
	TokenError(Error),
	UnexpectedError,
}

// Create cookie with a new JWT auth token
//
pub fn generate_auth_cookie(email: &Email) -> Result<Cookie<'static>, GenerateTokenError> {
	let token  = generate_auth_token(email)?;
	let cookie = Cookie::build((JWT_COOKIE_NAME, token))
		.path("/")                       // apply cookie to all URLs on the server
		.http_only(true)                 // prevent JavaScript from accessing the cookie
		.same_site(SameSite::Lax)        // send cookie with "same-site" requests, and with "cross-site" top-level navigations.
		.build();
	Ok(cookie)
}

// Commented this out because it's only used in one spot, so dont' see a need for
// a separate function.
/*
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
*/

// Create JWT auth token
pub fn generate_auth_token(email: &Email) -> Result<String, GenerateTokenError> {
	let delta = chrono::Duration::try_seconds(TOKEN_TTL_SECONDS)
		.ok_or(GenerateTokenError::UnexpectedError)?;

	let exp = Utc::now()                                    // Create JWT expiration time
		.checked_add_signed(delta)
		.ok_or(GenerateTokenError::UnexpectedError)?
		.timestamp();

	let exp: usize = exp                                     // Cast exp to usize, (what Claims expects)
		.try_into()
		.map_err(|_| GenerateTokenError::UnexpectedError)?;

	let sub    = email.as_ref().to_owned();
	let claims = Claims {sub, exp};
	create_token(&claims).map_err(GenerateTokenError::TokenError)
}

// Check if JWT auth-token is valid by decoding it using the JWT secret
//
pub async fn validate_token(
	token:           &str,
	banned_tokens:   TokenStoreType,
	) -> Result<Claims, Error> {
	println!("Validating token\n\t{:?}", token);

	match banned_tokens.read().await.contains_token(token).await {
		Ok(true) => {
			println!("Token is banned: {:?}", token);
			return Err(Error::from(ErrorKind::InvalidToken));
		},
		Err(_) => {return Err(Error::from(ErrorKind::InvalidToken))},
		_ => {}
	}

	let key        = DecodingKey::from_secret(JWT_SECRET.as_bytes());
	let validation = &Validation::default();
	let data       = jsonwebtoken::decode::<Claims>(token, &key, &validation);
	let claims     = data.map(|v| v.claims);
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
