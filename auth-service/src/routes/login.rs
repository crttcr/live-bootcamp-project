use crate::app_state::AppState;
use crate::domain::email::Email;
use crate::domain::error::AuthAPIError;
use crate::domain::password::Password;
use crate::utils::auth::generate_auth_cookie;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

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
    jar:            CookieJar,
    Json(request):  Json<LoginRequest>,
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError>{
    println!("Received login request: {:?}", request);
    let email    = Email::parse(request.email).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = Password::parse(&request.password).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let store    = state.user_store.read().await;
    match store.validate_user(&email, &password).await {
        Ok(_) => {
            println!("User with email {} authenticated.", &email);
            let auth_cookie = match generate_auth_cookie(&email) {
                Ok(v) => v,
                Err(_) => return Err(AuthAPIError::UnexpectedError),
            };
            let updated_jar = jar.add(auth_cookie);
            println!("Cookie jar updated. {}.", updated_jar.iter().count());
            Ok((updated_jar, StatusCode::OK.into_response()))
        },
        Err(e) => {
            println!("Validation failed: {:?}", e);
            Err(AuthAPIError::IncorrectCredentials)
        }
    }
}
