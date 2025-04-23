use auth_service::routes::{LoginRequest, SignupRequest};
use crate::helpers::TestApp;

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

