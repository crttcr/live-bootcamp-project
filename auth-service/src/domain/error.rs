use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};

pub enum AuthAPIError 
{
   IncorrectCredentials,
   InvalidCredentials,
   InvalidToken,
   MissingToken,
   UnexpectedError,
   UserAlreadyExists,
}

impl IntoResponse for AuthAPIError
{
   fn into_response(self) -> Response 
   {
      let (status, error_message) = match self {
         AuthAPIError::IncorrectCredentials  => (StatusCode::UNAUTHORIZED,          "Authorization failure"),
         AuthAPIError::InvalidCredentials    => (StatusCode::BAD_REQUEST,           "Invalid credentials"  ),
         AuthAPIError::InvalidToken          => (StatusCode::UNAUTHORIZED,          "Invalid token "       ),
         AuthAPIError::MissingToken          => (StatusCode::BAD_REQUEST,           "Missing token"        ),
         AuthAPIError::UserAlreadyExists     => (StatusCode::CONFLICT,              "User already exists"  ),
         AuthAPIError::UnexpectedError       => (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error"     ),
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
