use serde::{Deserialize, Serialize};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;

use crate::domain::email::Email;
use crate::domain::password::Password;
use crate::domain::error::AuthAPIError;
use crate::domain::user::User;
use crate::app_state::AppState;

#[derive(Deserialize, Debug)]
pub struct SignupRequest {
    pub email:          String,
    pub password:       String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa:   bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SignupResponse {
    pub message: String,
}

pub async fn signup(
    State(state):   State<AppState>,
    Json(request):  Json<SignupRequest>,
    ) -> impl IntoResponse {
    println!("Received signup request: {:?}", request);
    let email = match Email::parse(&request.email) {
        Ok(e) => {e},
        Err(e) => {
            println!("Invalid email: {:?}", e);
            return Err(AuthAPIError::InvalidCredentials);
        }
    };

    let password = match Password::parse(&request.password) {
        Ok(e) => {e},
        Err(e) => {
            println!("Invalid password: {:?}", e);
            return Err(AuthAPIError::InvalidCredentials);
        }
    };

    let mut store = state.user_store.write().await;
    if store.get_user(&email).await.is_ok() { 
        println!("User with email {} already exists.", &email);
        return Err(AuthAPIError::UserAlreadyExists) 
    }

    let user      = User::new(email, password, request.requires_2fa);
    let result    = store.add_user(user).await;
    match result {
        Ok(_) => {
            println!("User added successfully");
            let message  = "User created successfully!".to_owned();
            let response = Json(SignupResponse{message});
            Ok((StatusCode::CREATED, response))
        }
        Err(e) => {
            println!("Failed to add user: {:?}", e);
            Err(AuthAPIError::UnexpectedError)
        }
    }
}