use crate::app_state::AppState;
use crate::domain::AuthAPIError;
use crate::utils::auth::validate_token;
use crate::utils::constants::JWT_COOKIE_NAME;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::{cookie, CookieJar};
use color_eyre::eyre::eyre;
use tracing::{debug, warn};

#[tracing::instrument(name = "logout", skip(state))]
pub async fn logout(
   State(state):   State<AppState>,
   jar:            CookieJar,
   ) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
   let cookie = match jar.get(JWT_COOKIE_NAME) {
      Some(c) => c,
      None    => {return Err(AuthAPIError::MissingToken) }
   };
   
   // Validate token
   let token     = cookie.value();
   debug!("Token: {}", token);
   let store     = state.banned_tokens.clone();
   if let Err(_) = validate_token(token, store).await {
		warn!("Token is invalid.");
      return Err(AuthAPIError::InvalidToken);
   }
   
   // Add token to banned tokens store
   if state.banned_tokens
      .write().await
      .add_token(token.to_owned()).await
      .is_err() {
      return Err(AuthAPIError::UnexpectedError(eyre!("Failed to add token to banned tokens store.")));
   }
	
   let count = state.banned_tokens.read().await.count().await
		.unwrap();
   println!("Banned Count: {}", count);
   
   // Remove cookie and return modified jar
   let cookie_for_removal = cookie::Cookie::build(JWT_COOKIE_NAME).path("/").build();
   let updated_jar        = jar.remove(cookie_for_removal);
   println!("Cookie removed from jar. {}.", updated_jar.iter().count());
   Ok((updated_jar, StatusCode::OK))
}
