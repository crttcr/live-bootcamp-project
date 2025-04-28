use serde_json::{json, Value};
use auth_service::domain::Email;
use auth_service::routes::Verify2FARequest;
use crate::helpers::TestApp;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app      = TestApp::new().await;
    let body     = json!(r#"{"code": "123456"}"#);
    let response = app.post_verify_2fa(&body).await;
    assert_eq!(response.status().as_u16(), 422);
}

#[tokio::test]
async fn should_return_400_if_invalid_object_input() {
    let app      = TestApp::new().await;
    let body     = Verify2FARequest{ email: "not_an_email".to_string(), login_attempt_id: "466b32c2-6862-4c18-ada0-7d59eaf6e004".to_string(), code: "123456".to_string()};
    let response = app.post_verify_2fa(&body).await;
    assert_eq!(response.status().as_u16(), 400);
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

/*
#[tokio::test]
async fn should_return_200_if_valid_json_input() {
    let app  = TestApp::new().await;
    let body = serde_json::from_str::<serde_json::Value>(r#"{
        "email":          "a@b.com",
        "loginAttemptId": "466b32c2-6862-4c18-ada0-7d59eaf6e004",
        "2FACode":        "123456"
    }"#).unwrap();
    let response = app.post_verify_2fa(&body).await;
    assert_eq!(response.status().as_u16(), 200);
}
*/

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

#[tokio::test]
async fn should_return_401_if_old_code() {
    let app          = TestApp::new().await;
    let email_str    = "a@b.com";
    let email        = Email::parse(email_str.to_string()).unwrap();
    let passw_str    = "password123**Archive";
    let signup_body  = json!({
        "email":       email_str,
        "password":    passw_str,
        "requires2FA": true
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);
    let login_body  = json!({
        "email":       email_str,
        "password":    passw_str
    });
    let response = app.post_login(&signup_body).await;
    println!("{:?}", response);
    assert_eq!(response.status().as_u16(), 206);
    println!("Got the right status. {:?}", response.status());
    let response_json: Value = response.json().await.unwrap();
    println!("Here is the Json . {:?}", response_json);
        
//    let code_store = app.two_fa_code_store.read().await;
 //   let entry      = code_store.get_code(&email).await.unwrap();
    let code       = "333333"; // entry.1;
    println!("Got the code: {:?}", code);

    // Login attempt again ...
    let response = app.post_login(&login_body).await;
    let response_json: Value = response.json().await.unwrap();
    let lai                  = response_json.get("loginAttemptId").unwrap().as_str().unwrap();
    println!("Got the lai : {:?}", lai);
    let body = json!({
        "email":          email_str,
        "loginAttemptId": lai,
        "2FACode":        code
    });
    let response = app.post_verify_2fa(&body).await;
    println!("Verify response when passing in expired code:  {:?}", response.status());
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_200_if_valid_object_input() {
    let app          = TestApp::new().await;
    let email_str    = "a@b.com";
    let email        = Email::parse(email_str.to_string()).unwrap();
    let passw_str    = "password123**Archive";
    let signup_body  = json!({
        "email":       email_str,
        "password":    passw_str,
        "requires2FA": true
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);
    let login_body  = json!({
        "email":       email_str,
        "password":    passw_str
    });
    let response = app.post_login(&signup_body).await;
    println!("{:?}", response);
    assert_eq!(response.status().as_u16(), 206);
    println!("Got the right status. {:?}", response.status());

    let response_json: Value = response.json().await.unwrap();
    let lai        = response_json.get("loginAttemptId").unwrap().as_str().unwrap();
    let code       = response_json.get("2FACode").unwrap().as_str().unwrap();
    println!("Got the lai : {:?}", lai);
    println!("Got the code: {:?}", code);
    let body     = json!({
        "email":          email_str,
        "loginAttemptId": lai,
        "2FACode":        code
    });
    let response = app.post_verify_2fa(&body).await;
    println!("Verify response when passing in expired code:  {:?}", response.status());
    assert_eq!(response.status().as_u16(), 200);
}

