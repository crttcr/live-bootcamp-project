use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;
use crate::domain::email::Email;
use crate::domain::error::AuthAPIError;
use crate::domain::password::Password;
use crate::domain::user::User;

#[derive(Deserialize, Debug, Serialize)]
pub struct SignupRequest {
    pub email:          String,
    pub password:       String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa:   bool,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct SignupResponse {
    pub message: String,
}

pub async fn signup(
    State(state):   State<AppState>,
    Json(request):  Json<SignupRequest>,
    ) -> impl IntoResponse {
    println!("Received signup request: {:?}", request);
    let email    = Email::parse(    request.email   ).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = Password::parse(&request.password).map_err(|_| AuthAPIError::InvalidCredentials)?;
    
    let mut user_store = state.user_store.write().await;
    if user_store.get_user(&email).await.is_ok() { 
        println!("User with email {} already exists.", &email);
        return Err(AuthAPIError::UserAlreadyExists) 
    }

    let user      = User::new(email, password, request.requires_2fa);
    let result    = user_store.add_user(user).await;
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
