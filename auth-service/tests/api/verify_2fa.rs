use serde_json::{json, Value};
use auth_service::domain::{Email, TwoFACode};
use auth_service::routes::Verify2FARequest;
use crate::helpers::TestApp;

// Make sure to assert the auth cookie gets set
#[tokio::test]
async fn should_return_200_if_correct_code() {
    let app          = TestApp::new().await;
    let email_str    = "a@b.com";
    let email        = Email::parse(email_str.to_string()).unwrap();
    let passw_str    = "password123**Archive";
    
    // Signup user
    let signup_body  = json!({
        "email":       email_str,
        "password":    passw_str,
        "requires2FA": true
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    // Attempt login
    let login_body  = json!({
        "email":       email_str,
        "password":    passw_str,
    });
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);

    let response_json: Value = response.json().await.unwrap();
    let logon_attempt_id     = response_json.get("loginAttemptId").unwrap().as_str().unwrap();
    let code                 = get_code_from_2fa_store(&app, &email).await.unwrap();
    
    // Verify the code
    let body = json!({
       "email":          email_str,
       "loginAttemptId": logon_attempt_id,
       "2FACode":        code
    });
    let response   = app.post_verify_2fa(&body).await;
    let cookies    = response.cookies().count();
    println!("Verify response when passing in expired code: {:?}", response.status());
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(cookies, 1);   
    
}

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
async fn should_return_401_if_valid_object_input() {
    let app          = TestApp::new().await;
    let email_str    = "a@b.com";
    let email        = Email::parse(email_str.to_string()).unwrap();
    let passw_str    = "password123**Archive";
    
    // Signup user
    let signup_body  = json!({
        "email":       email_str,
        "password":    passw_str,
        "requires2FA": true
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);
    
    // First login attempt
    let login_body  = json!({
        "email":       "a@b.com",
        "password":    "password123**Archive",
    });
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);
    
    // Get the code from the 2FA store
    let code = get_code_from_2fa_store(&app, &email).await.unwrap();
    
    // Second login attempt
    let login_body  = json!({
        "email":       "a@b.com",
        "password":    "password123**Archive",
    });
    let response             = app.post_login(&login_body).await;
    let response_json: Value = response.json().await.unwrap();
    let logon_attempt_id     = response_json.get("loginAttemptId").unwrap().as_str().unwrap();
    println!("Got the logon_attempt_id : {:?}", logon_attempt_id);
    
    // Attempt to verify the code from the first login attempt
    let body = json!({
       "email":          email_str,
       "loginAttemptId": logon_attempt_id,
       "2FACode":        code
    });
    let response = app.post_verify_2fa(&body).await;
    println!("Verify response when passing in expired code:  {:?}", response.status());
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
    let response = app.post_login(&login_body).await;
    println!("{:?}", response);
    assert_eq!(response.status().as_u16(), 206);
    println!("Got the right status. {:?}", response.status());
    let code = get_code_from_2fa_store(&app, &email).await.unwrap();
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

//
// Helper functions
//

// This helper function ensures that our read lock is dropped as soon as we've completed the read
//
async fn get_code_from_2fa_store(app: &TestApp, email: &Email) -> Option<TwoFACode> {
    let code_store  = app.two_fa_code_store.read().await;
    let entry       = code_store.get_code(&email).await.unwrap();
    let code        = entry.1;
    println!("Got the code: {:?}", code);
    Some(code)
}

