use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use tracing::debug;
use crate::app_state::AppState;
use crate::domain::{AuthAPIError, Email, LoginAttemptId, TwoFACode};
use crate::utils::auth::generate_auth_cookie;

#[derive(Deserialize, Debug, Serialize)]
pub struct Verify2FARequest {
   pub email:            String,
   #[serde(rename = "loginAttemptId")]
   pub login_attempt_id: String,
   #[serde(rename = "2FACode")]
   pub code:             String,
}

/*
// This is what the request looks like in Json
{
  "email":          "user@example.com",
  "loginAttemptId": "b59d20f4-dfcf-4fc5-af99-e312e0e2d2aa",
  "2FACode":        "123456"
}
*/


#[tracing::instrument(name = "verify 2fa", skip_all)]
pub async fn verify_2fa(
   State(state):  State<AppState>,
   jar:           CookieJar,
   Json(request): Json<Verify2FARequest>,
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError>
{
   let email             = Email::parse(request.email)                    .map_err(|_| AuthAPIError::InvalidCredentials)?;
   let login_attempt_id  = LoginAttemptId::parse(request.login_attempt_id).map_err(|_| AuthAPIError::InvalidCredentials)?;
   let two_fa_code       = TwoFACode::parse(request.code)                 .map_err(|_| AuthAPIError::InvalidCredentials)?;
   let tuple             = get_tuple_from_store(&state, &email).await?;

   // Verify the user's post contains the information we have in the store.
   if tuple.0 != login_attempt_id { return Err(AuthAPIError::IncorrectCredentials); }
   if tuple.1 != two_fa_code      { return Err(AuthAPIError::IncorrectCredentials); }

   debug!("Credentials verified");
   
   // Verified.
   // * Remove the entry from the store
   // * Generate a new auth cookie
   // * Return success result
   //
   remove_entry_from_store(&state, &email).await?;
   let auth_cookie = match generate_auth_cookie(&email) {
      Ok(cookie) => cookie,
      Err(e)     => return Err(AuthAPIError::UnexpectedError(e)),
   };

   debug!("Adding to cookie jar");
   let cookies = jar.add(auth_cookie);
   Ok((cookies, StatusCode::OK.into_response()))
}

// This helper function ensures that our read lock is dropped as soon as we've completed the read
//
#[tracing::instrument(name = "get tuple from store", skip_all)]
async fn get_tuple_from_store(
   state: &AppState,
   email: &Email,
) -> Result<(LoginAttemptId, TwoFACode), AuthAPIError>
{
   let two_fa_code_store = state.two_fa_code_store.read().await;
   let raw               = two_fa_code_store.get_code(email).await;
   raw.map_err(|_| AuthAPIError::IncorrectCredentials)
}

#[tracing::instrument(name = "remove entry from store", skip_all)]
async fn remove_entry_from_store(
   state: &AppState,
   email: &Email,
) -> Result<(), AuthAPIError>
{
   let mut two_fa_code_store = state.two_fa_code_store.write().await;
   let raw                   = two_fa_code_store.remove_code(email).await;
   raw.map_err(|_| AuthAPIError::IncorrectCredentials)
}
