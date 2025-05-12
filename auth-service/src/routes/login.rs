use crate::app_state::AppState;
use crate::domain::email::Email;
use crate::domain::error::AuthAPIError;
use crate::domain::password::Password;
use crate::domain::{LoginAttemptId, TwoFACode};
use crate::routes::LoginResponse::TwoFactorAuth;
use crate::utils::auth::generate_auth_cookie;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::CookieJar;
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use tracing::debug;

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    pub email:          String,
    pub password:       Secret<String>,
}

// The login route can return 2 possible success responses.
// This enum models each response!
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}

// If a user requires 2FA, this JSON body should be returned!
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}

impl TwoFactorAuthResponse {
    pub fn new(id: LoginAttemptId) -> Self {
        let message          = "2FA required".to_string();
        let login_attempt_id = id.as_ref().to_string();
        Self {message, login_attempt_id}
    }
}

#[tracing::instrument(name = "login handler", skip_all)]
pub async fn login(
    State(state):   State<AppState>,
    jar:            CookieJar,
    Json(request):  Json<LoginRequest>,
    ) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    debug!("Received login request: {:?}", request);
    println!("Received login request: {:?}", request);
    debug!("Sending password: {}", request.password.expose_secret());
    println!("Sending password: {}", request.password.expose_secret());
    let password = match Password::parse(request.password) {
        Ok(password) => password,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let email = Secret::new(request.email);
    let email = match Email::parse(email) {
        Ok(email) => email,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

//
// This approach requires returning Result<(CookieJar, impl IntoResponse), AuthAPIError>
// Unfortunately, when we deal with two possible responses, we need to use a different approach
// because the response statuses are different and Axum doesn't support this return signature:
//
// -> Result<(CookieJar, StatusCode, impl IntoResponse), AuthAPIError>
//
//    let email    = Email::   parse(request.email    ).map_err(|_| AuthAPIError::InvalidCredentials)?;
//    let password = Password::parse(&request.password).map_err(|_| AuthAPIError::InvalidCredentials)?;
//    let user        = store.get_user(&email).await.map_err(|_| AuthAPIError::InvalidCredentials);
//

    let user_store = state.user_store.read().await;
    if user_store.validate_user(&email, &password).await.is_err() {
        debug!("User Store: validation failed");
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }
    debug!("User authenticated.");

    let user = match user_store.get_user(&email).await {
        Ok(user) => user,
        Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials)),
    };
    println!("User acquired.");
    let auth_cookie = match generate_auth_cookie(&user.email) {
        Ok(cookie) => cookie,
        Err(e)     => return (jar, Err(AuthAPIError::UnexpectedError(e.into()))),
    };

    println!("Cookie generated: {}", auth_cookie.value());
    let updated_jar = jar.add(auth_cookie);
    println!("Cookie jar updated. {}.", updated_jar.iter().count());

    match user.requires_2fa {
        true  => handle_2fa(&email, &state, updated_jar).await,
        false => handle_no_2fa(&user.email, updated_jar).await,
    }
}

#[tracing::instrument(name = "handle non 2fa", skip_all)]
async fn handle_no_2fa(email: &Email, jar: CookieJar) ->
(
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
)
{
    let auth_cookie = match generate_auth_cookie(email) {
        Ok(cookie) => cookie,
        Err(e)     => return (jar, Err(AuthAPIError::UnexpectedError(e))),
    };

    println!("Updating cookie jar");
    let cookies = jar.add(auth_cookie);
    let body    = Json(LoginResponse::RegularAuth);
    (cookies, Ok((StatusCode::OK, body)))
}

#[tracing::instrument(name = "handle 2fa", skip_all)]
async fn handle_2fa(email: &Email, state: &AppState, jar: CookieJar) ->
(
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
)
{
    let store_result = write_2fa_details_into_code_store(state, email).await;
    match store_result {
        Err(e) => (jar, Err(AuthAPIError::UnexpectedError(e.into()))),
        Ok((id, code)) => {
            let cookies    = jar;
            let emailer    = state.email_client.write().await;
            let subject    = "Login requires 2FA code";
            let content    = format!("Code [{}], Login Attempt ID: {}\n", id, code);
            match emailer.send_email(email, subject, content.as_ref()).await {
                Ok(_) => {},
                Err(e) => return (cookies, Err(AuthAPIError::UnexpectedError(e))),
            }

            let response         = TwoFactorAuthResponse::new(id);
            let response         = TwoFactorAuth(response);
            let body             = Json(response);
            (cookies, Ok((StatusCode::PARTIAL_CONTENT, body)))
        },
    }
}

// Helper function ensures that the write lock is dropped as soon as 
// the update is complete.
//
#[tracing::instrument(name = "write 2fa to code_store", skip_all)]
async fn write_2fa_details_into_code_store(state: &AppState, email: &Email) -> Result<(LoginAttemptId, TwoFACode), AuthAPIError> {
    let code             = TwoFACode::new();
    let login_attempt_id = LoginAttemptId::new();
    let mut store        = state.two_fa_code_store.write().await;
    match store.add_code(email.clone(), login_attempt_id.clone(), code.clone()).await {
        Ok(_)  => Ok((login_attempt_id, code)),
        Err(e) => Err(AuthAPIError::UnexpectedError(e.into())),

    }
}
