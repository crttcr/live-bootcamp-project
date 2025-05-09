use serde_json::json;
use crate::helpers_harness::TestApp;
use auth_service::domain::Email;
use auth_service::utils::auth::generate_auth_token;
use crate::helpers_arrange::setup_logged_in_user;
use crate::helpers_assert::assert_status;
use crate::helpers_harness::{get_random_email};

#[tokio::test]
async fn should_return_200_valid_token() {
    let app            = TestApp::new().await;                     // Arrange
    let (_user, token) = setup_logged_in_user(&app).await;
    let body           = json!({"token": token});
    let response       = app.post_verify_token(&body).await;       // Act
    assert_status(&response, 200, None);                           // Assert
}

#[tokio::test]
async fn should_return_422_if_string_input() {
    let app      = TestApp::new().await;
    let body     = "malformed".to_owned();
    let response = app.post_verify_token(&body).await;
    assert_status(&response, 422, None);                           // Assert
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app      = TestApp::new().await;
    let body     = json!(r#"{"malformed": "<-- that key should be 'token'"}"#);
    let response = app.post_verify_token(&body).await;             // Act
    assert_status(&response, 422, None);                           // Assert
}


#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app      = TestApp::new().await;
    let token    = "This is not a valid token";
    let body     = json!({"token": token});
    let response = app.post_verify_token(&body).await;             // Act
    assert_status(&response, 401, None);                           // Assert
}

#[tokio::test]
async fn should_return_401_if_token_is_banned() {
    let app             = TestApp::new().await;              // Arrange
    let (_user, token)  = setup_logged_in_user(&app).await;
    let body            = json!({"token": token});
    let logout_response = app.post_logout().await;          // Ban token by logging out
    assert_status(&logout_response, 200, None);

    let response        = app.post_verify_token(&body).await;     // Act

    // Assert
    let banned_tokens = app.banned_tokens.read().await;
    let token         = token.as_str();
    assert!(banned_tokens.contains_token(&token).await);
    assert_status(&response, 402, None);
}


#[tokio::test]
async fn should_return_401_if_banned_token_crt() {
    let app      = TestApp::new().await;
    let email    = get_random_email();
    let email    = Email::parse(email).unwrap();
    let token    = generate_auth_token(&email).unwrap();
    println!("token: {}", token);

    // Now that we have a token, before we post it, let's add it to the
    // banded tokens set. After that, we should get a 401.
    // We do this in a nested scope so our write lock is dropped before
    // we post to the verify token endpoint.
    //
    {
        let mut tokens = app.banned_tokens.write().await;
        tokens.add_token(token.clone()).await.unwrap();
        println!("token added");
    }

    // Token has been added to the banned tokens set; post to the 
    // verify token endpoint and ensure that we get a 401.
    //
    let body     = json!({"token": token});
    println!("created body {}", body);
    let response = app.post_verify_token(&body).await;
    println!("Asserting result");
    assert_eq!(response.status().as_u16(), 401);
}
