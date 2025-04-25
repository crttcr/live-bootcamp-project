use crate::app_state::AppState;
use crate::domain::AuthAPIError;
use crate::utils::auth::validate_token;
use crate::utils::constants::JWT_COOKIE_NAME;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::{cookie, CookieJar};

pub async fn logout(
   State(state):   State<AppState>,
   jar:            CookieJar,
   ) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
   let cookie = match jar.get(JWT_COOKIE_NAME) {
      Some(c) => c,
      None    => {return (jar, Err(AuthAPIError::MissingToken)); }
   };
   
   // Validate token
   let token = cookie.value();
   println!("Token: {}", token);
   let store = state.banned_tokens.clone();
   if let Err(_) = validate_token(token, store).await {
      return (jar, Err(AuthAPIError::InvalidToken));
   }
   
   // Add token to banned tokens store
   if state.banned_tokens
      .write().await
      .add_token(token.to_owned())
      .await.is_err() {
      return (jar, Err(AuthAPIError::UnexpectedError));
   }
   let count = state.banned_tokens.read().await.count().await.unwrap();
   println!("Banned Count: {}", count);
   
   // Remove token from cookie jar and return modified jar
   let cookie = cookie::Cookie::from(JWT_COOKIE_NAME);
   let jar    = jar.remove(cookie);
   println!("Cookie removed from jar. {}.", jar.iter().count());
   (jar, Ok(StatusCode::OK))
}
