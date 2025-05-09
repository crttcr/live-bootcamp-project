use serde_json::json;
use auth_service::routes::Verify2FARequest;
use crate::helpers_arrange::*;
use crate::helpers_assert::*;
use crate::helpers_harness::TestApp;


#[tokio::test]
async fn should_return_200_if_correct_code() {
    // Arrange
    let app                 = TestApp::new().await;
    let (user, two_fa_data) = setup_2fa_login_started(&app).await;
    let payload             = create_2fa_payload(&user.email, &two_fa_data);

    // Act
    let response            = app.post_verify_2fa(&payload).await; 
    
    // Assert
    assert_status(&response, 200, None);
    assert_has_auth_cookie(&response);
}

#[tokio::test]
async fn should_return_401_if_old_code() {
    let app                 = TestApp::new().await;
    let (user, two_fa_data) = setup_2fa_login_started(&app).await;
    let payload             = create_2fa_payload(&user.email, &two_fa_data);

    // Act
    app.post_login(&user.login_payload()).await;         // Second login call (invalidates the first login attempt)
    let response = app.post_verify_2fa(&payload).await;  // Verify with old id and code
    
    // Assert
    assert_status(&response, 401, None);
}

#[tokio::test]
async fn should_return_401_if_same_code_twice() {
    // Arrange
    let app                 = TestApp::new().await;
    let (user, two_fa_data) = setup_2fa_login_started(&app).await;
    let request_body        = create_2fa_payload(&user.email, &two_fa_data);

    // Act
    let first_response  = app.post_verify_2fa(&request_body).await;
    let second_response = app.post_verify_2fa(&request_body).await;

    // Assert
    assert_status(&first_response, 200, None);
    assert_has_auth_cookie(&first_response);
    assert_status(&second_response, 401, None);
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app      = TestApp::new().await;
    let body     = json!(r#"{"code": "123456"}"#);
    let response = app.post_verify_2fa(&body).await;
    assert_status(&response, 422, None);
}

#[tokio::test]
async fn should_return_400_if_invalid_object_input() {
    let app      = TestApp::new().await;
    let body     = Verify2FARequest{ email: "not_an_email".to_string(), login_attempt_id: "466b32c2-6862-4c18-ada0-7d59eaf6e004".to_string(), code: "123456".to_string()};
    let response = app.post_verify_2fa(&body).await;
    assert_status(&response, 400, None);
}

#[tokio::test]
async fn should_return_400_if_email_is_malformed() {
    let app  = TestApp::new().await;
    let body = serde_json::from_str::<serde_json::Value>(r#"{
        "email":          "not_an_email",
        "loginAttemptId": "466b32c2-6862-4c18-ada0-7d59eaf6e004",
        "2FACode":        "123456"
    }"#).unwrap();
    let response = app.post_verify_2fa(&body).await;
    assert_eq!(response.status().as_u16(), 400);
}


#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app  = TestApp::new().await;
    let body  = json!({
        "email":          "a@b.com",
        "loginAttemptId": "466b32c2-6862-4c18-ada0-7d59eaf6e004",
        "2FACode":        "123456"
    });
    let response = app.post_verify_2fa(&body).await;
    assert_eq!(response.status().as_u16(), 401);
}

