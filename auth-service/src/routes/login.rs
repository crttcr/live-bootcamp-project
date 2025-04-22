
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use axum::Json;

use crate::domain::email::Email;
use crate::domain::password::Password;
use crate::domain::error::AuthAPIError;
use crate::domain::user::User;
use crate::app_state::AppState;

#[derive(Deserialize, Debug)]
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
   let email = match Email::parse(&request.email) {
      Ok(e)  => {e},
      Err(e) => {
         println!("Invalid email: {:?}", e);
         return Err(AuthAPIError::InvalidCredentials);
      }
   };

   let password = match Password::parse(&request.password) {
      Ok(e)  => {e},
      Err(e) => {
         println!("Invalid password: {:?}", e);
         return Err(AuthAPIError::InvalidCredentials);
      }
   };

   let store = state.user_store.read().await;
   match store.get_user(&email).await {
      Ok(user) => {
         println!("User with email {} located.", &email);
         if user.password == password {
            let message  = "User located".to_owned();
            let response = Json(LoginResponse{message});
            Ok(StatusCode::OK.into_response())
         } else {
            println!("Invalid password for user with email {}", &email);
            Err(AuthAPIError::InvalidCredentials)
         }
      },
      Err(e) => {
         println!("Error locating user: {:?}", e);
         Err(AuthAPIError::InvalidCredentials)
      } 
   }
}
