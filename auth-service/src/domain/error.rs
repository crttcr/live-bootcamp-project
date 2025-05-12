use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use color_eyre::eyre::Report;
use serde::{Deserialize, Serialize};
use thiserror::Error;


#[derive(Debug, Error)]
pub enum AuthAPIError 
{
   #[error("Credentials are incorrect")]
   IncorrectCredentials,
   #[error("Credentials are invalid")]
   InvalidCredentials,
   #[error("Token is invalid")]
   InvalidToken,
   #[error("Token is missing")]
   MissingToken,
   #[error("Unexpected error")]
   UnexpectedError(#[source] Report),
   #[error("User already exists")]
   UserAlreadyExists,
}

impl IntoResponse for AuthAPIError
{
   fn into_response(self) -> Response 
   {
      log_error_chain(&self);
      let (status, error_message) = match self {
         AuthAPIError::IncorrectCredentials  => (StatusCode::UNAUTHORIZED,          "Authorization failure"),
         AuthAPIError::InvalidCredentials    => (StatusCode::BAD_REQUEST,           "Invalid credentials"  ),
         AuthAPIError::InvalidToken          => (StatusCode::UNAUTHORIZED,          "Invalid token "       ),
         AuthAPIError::MissingToken          => (StatusCode::BAD_REQUEST,           "Missing token"        ),
         AuthAPIError::UnexpectedError(_)    => (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error"     ),
         AuthAPIError::UserAlreadyExists     => (StatusCode::CONFLICT,              "User already exists"  ),
      };
      let error = error_message.to_string();
      let error = ErrorResponse{error};
      let body  = Json(error);
      (status, body).into_response()
   }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse 
{
    pub error: String,
}

fn log_error_chain(e: &(dyn std::error::Error + 'static)) {
   let separator   = "\n-----------------------------------------------------------------------------------\n";
   let mut report  = format!("{}{:?}\n", separator, e);
   let mut current = e.source();
   while let Some(cause) = current 
   {
      let str = format!("Caused by:\n\n{:?}", cause);
      report  = format!("{}\n{}", report, str);
      current = cause.source();
   }
   report = format!("{}\n{}", report, separator);
   tracing::error!("{}", report);
}