use auth_service::domain::{Email, LoginAttemptId};
use crate::helpers::{get_random_email, TestApp};
use auth_service::routes::{LoginRequest, SignupRequest, TwoFactorAuthResponse};
use auth_service::utils::constants::JWT_COOKIE_NAME;

// NOTE: Malformed credentials: the framework failed to
// convert the request body into a JSON object containing both
// an "email" key and a "password" key. As a result, the body 
// cannot be turned into an instance of the LoginRequest struct.
// 
// Axum handles this case automatically when this extraction fails
//     Json(request):  Json<LoginRequest>,
//
#[tokio::test]
pub async fn should_return_422_if_malformed_string_credentials()
{
    let app      = TestApp::new().await;
    let body     = r#"{"email":"a@b.com"}"#.to_owned();
    let response = app.post_login(&body).await;
    assert_eq!(response.status().as_u16(), 422);
}

#[tokio::test]
pub async fn should_return_422_if_malformed_json_credentials()
{
    let app  = TestApp::new().await;
    let body = serde_json::json!({
            "email":       "a@b.c",
            "requires2FA": true,
        });    
    let response = app.post_login(&body).await;
    assert_eq!(response.status().as_u16(), 422);
}

#[tokio::test]
pub async fn should_return_401_if_user_does_not_exist()
{
    let app      = TestApp::new().await;
    let email    = "foo@bar.com".to_owned();
    let password = "Alpha123**8".to_owned();
    let body     = LoginRequest{email, password};
    let response = app.post_login(&body).await;
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
pub async fn should_return_401_if_password_is_incorrect()
{
    let app      = TestApp::new().await;
    let email    = "foo@bar.com".to_owned();
    let password = "Alpha123**8".to_owned();
    let passjunk = "Delta456**9".to_owned();
    let signup   = SignupRequest{email : email.clone(), password: passjunk.clone(), requires_2fa: false};
    let _        = app.post_signup(&signup).await;
    let body     = LoginRequest{email, password};
    let response = app.post_login(&body).await;
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
pub async fn should_return_200_after_successful_signup()
{
    let app      = TestApp::new().await;
    let email    = "foo@bar.com".to_owned();
    let password = "Alpha123**8".to_owned();
    let signup   = SignupRequest{email : email.clone(), password: password.clone(), requires_2fa: false};
    let _        = app.post_signup(&signup).await;
    let body     = LoginRequest{email, password};
    let response = app.post_login(&body).await;
    assert_eq!(response.status().as_u16(), 200);
}


#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
    let app          = TestApp::new().await;
    let random_email = get_random_email();
    let signup_body  = serde_json::json!({
        "email": random_email,
        "password": "password123**Archive",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    println!("Signup response {} .", response.status());
    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password":"password123**Archive",
    });

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");
    println!("Auth cookie: {}", auth_cookie.value());
    assert!(!auth_cookie.value().is_empty());
}

#[tokio::test]
async fn should_return_206_if_valid_credentials_and_2fa_enabled() {
    let app          = TestApp::new().await;
    let random_email = get_random_email();
    let signup_body  = serde_json::json!({
        "email":       random_email,
        "password":    "password123**Archive",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;
    println!("Signup response {} .", response.status());
    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email":     random_email,
        "password":  "password123**Archive",
    });

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);
    
    let json_body = response.json::<TwoFactorAuthResponse>()
       .await.expect("Could not deserialize response body to TwoFactorAuthResponse");
    println!("JSON body: {:?}", json_body);
    
    let login_attempt_id = LoginAttemptId::parse(json_body.login_attempt_id).unwrap();
    let code_store       = app.two_fa_code_store.read().await;
    let email_key        = Email::parse(random_email.clone()).unwrap();
    let foo              = code_store.get_code(&email_key).await;
    println!("Code store result: {:?}", foo);
    match code_store.get_code(&email_key).await {
        Ok(tuple) => {
            assert_eq!(tuple.0, login_attempt_id);
        },
        Err(x) => {
            panic!("Email [{}] not found in code store: {:?}", random_email, x);
        }       
    }
}