use crate::app_state::AppState;
use crate::domain::email::Email;
use crate::domain::error::AuthAPIError;
use crate::domain::password::Password;
use crate::routes::LoginResponse::TwoFactorAuth;
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
    pub fn new(message: String, login_attempt_id: String) -> Self {
        Self {message, login_attempt_id}
    }

    // TODO: Return a TwoFactorAuthResponse. The message should be "2FA required".
    // The login attempt ID should be "123456". We will replace this hard-coded login attempt ID soon!
    pub fn default() -> Self {
        let message          = "2FA required".to_string();
        let login_attempt_id = "123456".to_string();
        Self {message, login_attempt_id}
    }
}


pub async fn login(
    State(state):   State<AppState>,
    jar:            CookieJar,
    Json(request):  Json<LoginRequest>,
    ) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
//    ) -> Result<(CookieJar, impl IntoResponse), crate::domain::error::AuthAPIError >{
    println!("Received login request: {:?}", request);
    let password = match Password::parse(&request.password) {
        Ok(password) => password,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let email = match Email::parse(request.email) {
        Ok(email) => email,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

//
// This approach requires returning Result<(CookieJar, impl IntoResponse), AuthAPIError>
// Unfortunately, when we deal with two possible responses, we need to use a different approach
// because the responses statuses are different and Axum doesn't support this return signature:
//
// -> Result<(CookieJar, StatusCode, impl IntoResponse), AuthAPIError>
//
//    let email    = Email::   parse(request.email    ).map_err(|_| AuthAPIError::InvalidCredentials)?;
//    let password = Password::parse(&request.password).map_err(|_| AuthAPIError::InvalidCredentials)?;
//    let user        = store.get_user(&email).await.map_err(|_| AuthAPIError::InvalidCredentials);
//
    
    let store = state.user_store.read().await;
    if store.validate_user(&email, &password).await.is_err() {
        println!("User validation failed");
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }
    println!("User with email {} authenticated.", &email);

    let user = match store.get_user(&email).await {
        Ok(user) => user,
        Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials)),
    };    
    let auth_cookie = match generate_auth_cookie(&user.email) {
        Ok(cookie) => cookie,
        Err(_)     => return (jar, Err(AuthAPIError::UnexpectedError)),
    };

    let updated_jar = jar.add(auth_cookie);
    println!("Cookie jar updated. {}.", updated_jar.iter().count());

    match user.requires_2fa {
        true  => handle_2fa(updated_jar).await,
        false => handle_no_2fa(&user.email, updated_jar).await,
    }
}

//    Result<(CookieJar, StatusCode, Json<LoginResponse>), AuthAPIError>,
//    Result<WithCookies<Json<LoginResponse>>, AuthAPIError>
// New!
async fn handle_no_2fa(email: &Email, jar: CookieJar) ->
(
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
)
{
    let auth_cookie = match generate_auth_cookie(email) {
        Ok(cookie) => cookie,
        Err(_)     => return (jar, Err(AuthAPIError::UnexpectedError)),
    };

    println!("Updating cookie jar");
    let cookies = jar.add(auth_cookie);
    let body    = Json(LoginResponse::RegularAuth);
    (cookies, Ok((StatusCode::OK, body)))
}

async fn handle_2fa(jar: CookieJar) ->
(
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError >,
)
{
    let cookies  = jar;
    let response = TwoFactorAuthResponse::default();
    let response = TwoFactorAuth(response);
    let body     = Json(response);
    (cookies, Ok((StatusCode::PARTIAL_CONTENT, body)))
}
