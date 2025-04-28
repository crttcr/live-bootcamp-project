use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use crate::app_state::AppState;
use crate::domain::{AuthAPIError, Email, LoginAttemptId, TwoFACode};

/*
{
  "email": "user@example.com",
  "loginAttemptId": "string",
  "2FACode": "string"
}
 */
#[derive(Deserialize, Debug, Serialize)]
pub struct Verify2FARequest {
   pub email:            String,
   #[serde(rename = "loginAttemptId")]
   pub login_attempt_id: String,
   #[serde(rename = "2FACode")]
   pub code:             String,
}


pub async fn verify_2fa(
   State(state):  State<AppState>,
   Json(request): Json<Verify2FARequest>,
) -> Result<impl IntoResponse, AuthAPIError>
{
   let email             = Email::parse(request.email)                    .map_err(|_| AuthAPIError::InvalidCredentials)?;
   let login_attempt_id  = LoginAttemptId::parse(request.login_attempt_id).map_err(|_| AuthAPIError::InvalidCredentials)?;
   let two_fa_code       = TwoFACode::parse(request.code)                 .map_err(|_| AuthAPIError::InvalidCredentials)?;
   let two_fa_code_store = state.two_fa_code_store.write().await;
   let tuple             = match two_fa_code_store.get_code(&email).await {
      Err(_)    => return Err(AuthAPIError::IncorrectCredentials),
      Ok(tuple) => tuple,
   };
   
   if tuple.0 != login_attempt_id { return Err(AuthAPIError::IncorrectCredentials); }
   if tuple.1 != two_fa_code      { return Err(AuthAPIError::IncorrectCredentials); }

   Ok(StatusCode::OK.into_response())
}
