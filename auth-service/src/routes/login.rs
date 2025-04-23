
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use axum::Json;

use crate::domain::email::Email;
use crate::domain::password::Password;
use crate::domain::error::AuthAPIError;
use crate::app_state::AppState;

#[derive(Deserialize, Debug, Serialize)]
pub struct LoginRequest {
    pub email:          String,
    pub password:       String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginResponse {
    pub message: String,
}

pub async fn login(
    State(state):   State<AppState>,
    Json(request):  Json<LoginRequest>,
) -> impl IntoResponse {
   println!("Received login request: {:?}", request);

   let email    = Email::parse(   &request.email   ).map_err(|_| AuthAPIError::InvalidCredentials)?;
   let password = Password::parse(&request.password).map_err(|_| AuthAPIError::InvalidCredentials)?;
   let store    = state.user_store.read().await;

   match store.validate_user(&email, &password).await {
      Ok(_) => {
         println!("User with email {} authenticated.", &email);
         let message  = "User authenticated".to_owned();
         let response = Json(LoginResponse{message});
         Ok((StatusCode::OK, response))
      },
      Err(e) => {
         println!("Error validating user: {:?}", e);
         Err(AuthAPIError::IncorrectCredentials)
      }
   }
}
