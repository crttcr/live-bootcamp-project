use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use color_eyre::eyre::eyre;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument, warn};
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

#[instrument(name = "Signup", skip_all)]
pub async fn signup(
    State(state):   State<AppState>,
    Json(request):  Json<SignupRequest>,
    ) -> impl IntoResponse {
    let email    = Email::parse(    request.email   ).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = Password::parse(&request.password).map_err(|_| AuthAPIError::InvalidCredentials)?;
    
    let mut user_store = state.user_store.write().await;
    if user_store.get_user(&email).await.is_ok() { 
        warn!("User with email {} already exists.", &email);
        return Err(AuthAPIError::UserAlreadyExists) 
    }

    let user      = User::new(email, password, request.requires_2fa);
    let result    = user_store.add_user(user).await;
    match result {
        Ok(_) => {
            info!("User added successfully");
            let message  = "User created successfully!".to_owned();
            let response = Json(SignupResponse{message});
            Ok((StatusCode::CREATED, response))
        }
        Err(e) => {
            warn!(?e, "Failed to add user");
            //Err(AuthAPIError::UnexpectedError(eyre!(e)))
            Err(AuthAPIError::UnexpectedError(e.into()))
        }
    }
}
