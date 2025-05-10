use crate::helpers_arrange::{get_2fa_code_tuple, setup_registered_user, TestUser};
use crate::helpers_assert::{assert_has_auth_cookie, assert_status};
use crate::helpers_harness::TestApp;
use auth_service::routes::{LoginRequest, SignupRequest, TwoFactorAuthResponse};


#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
    let mut app      = TestApp::new().await;
    let user         = TestUser::new();
    setup_registered_user(&app, &user).await;

    let response = app.post_login(&user.login_payload()).await;
    assert_status(&response, 200, None);
    assert_has_auth_cookie(&response);
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_206_if_valid_credentials_and_2fa_enabled() {
    let mut app      = TestApp::new().await;
    let user         = TestUser::new_with_2fa();
    setup_registered_user(&app, &user).await;

    let response  = app.post_login(&user.login_payload()).await;
    assert_status(&response, 206, None);

    let json_body = response
       .json::<TwoFactorAuthResponse>()
       .await
       .expect("Could not deserialize response body to TwoFactorAuthResponse");
    assert_eq!(json_body.message, "2FA required".to_owned());
    println!("JSON body: {:?}", json_body);

    // Verify 2FA code was generated and matches the login attempt ID
    let (login_attempt_id, _) = get_2fa_code_tuple(&app, &user.email).await;
    assert_eq!(login_attempt_id, json_body.login_attempt_id);
    app.clean_up().await;
}

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
    let mut app  = TestApp::new().await;
    let body     = r#"{"email":"a@b.com"}"#.to_owned();
    let response = app.post_login(&body).await;
    assert_eq!(response.status().as_u16(), 422);
    app.clean_up().await;
}

#[tokio::test]
pub async fn should_return_422_if_malformed_json_credentials()
{
    let mut app = TestApp::new().await;
    let body    = serde_json::json!({
            "email":       "a@b.c",
            "requires2FA": true,
        });    
    let response = app.post_login(&body).await;
    assert_eq!(response.status().as_u16(), 422);
    app.clean_up().await;
}

#[tokio::test]
pub async fn should_return_401_if_email_not_registered()
{
    let mut app  = TestApp::new().await;
    let user     = TestUser::new();
    let body     = serde_json::json!({"email": "unregistered@example.com", "password": user.password});
    setup_registered_user(&app, &user).await;
    let response = app.post_login(&body).await;
    assert_status(&response, 401, None);
    app.clean_up().await;
}

#[tokio::test]
pub async fn should_return_401_if_user_does_not_exist()
{
    let mut app  = TestApp::new().await;
    let email    = "foo@bar.com".to_owned();
    let password = "Alpha123**8".to_owned();
    let body     = LoginRequest{email, password};
    let response = app.post_login(&body).await;
    assert_eq!(response.status().as_u16(), 401);
    app.clean_up().await;
}

#[tokio::test]
pub async fn should_return_401_if_password_is_wrong()
{
    let mut app  = TestApp::new().await;
    let user     = TestUser::new();
    let body     = serde_json::json!({"email":user.email, "password":"wrong_password"});
    setup_registered_user(&app, &user).await;
    let response = app.post_login(&body).await;
    assert_status(&response, 401, None);
    app.clean_up().await;
}

#[tokio::test]
pub async fn should_return_401_if_password_is_incorrect()
{
    let mut app  = TestApp::new().await;
    let email    = "foo@bar.com".to_owned();
    let password = "Alpha123**8".to_owned();
    let passjunk = "Delta456**9".to_owned();
    let signup   = SignupRequest{email : email.clone(), password: passjunk.clone(), requires_2fa: false};
    let _        = app.post_signup(&signup).await;
    let body     = LoginRequest{email, password};
    let response = app.post_login(&body).await;
    assert_eq!(response.status().as_u16(), 401);
    app.clean_up().await;
}

#[tokio::test]
pub async fn should_return_200_after_successful_signup()
{
    let mut app  = TestApp::new().await;
    let user     = TestUser::new();
    let _        = app.post_signup(&user.signup_payload()).await;
    let response = app.post_login(&user.signup_payload()).await;
    assert_eq!(response.status().as_u16(), 200);
    app.clean_up().await;
}
