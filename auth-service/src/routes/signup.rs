use serde::{Deserialize, Serialize};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;

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

impl SignupRequest {
    pub fn to_user(self) -> User {
        User::new(self.email, self.password, self.requires_2fa)
    }
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
    // Email validation
    let email = &request.email;
    if email.is_empty()     { return Err(AuthAPIError::InvalidCredentials) }
    if email.contains(" ")  { return Err(AuthAPIError::InvalidCredentials) }
    if !email.contains("@") { return Err(AuthAPIError::InvalidCredentials) }

    // Password validation
    let password  = &request.password;
    if password.len() < 8   { return Err(AuthAPIError::InvalidCredentials) }

    let mut store = state.user_store.write().await;
    if store.get_user(email.as_str()).is_ok() { return Err(AuthAPIError::UserAlreadyExists) }

    let user      = request.to_user();
    let result    = store.add_user(user);
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