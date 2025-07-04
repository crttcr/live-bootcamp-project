use crate::helpers_arrange::setup_logged_in_user;
use crate::helpers_assert::assert_status;
use crate::helpers_harness::TestApp;
use crate::helpers_harness::get_random_email;
use auth_service::domain::Email;
use auth_service::utils::auth::generate_jwt_auth_token;
use secrecy::Secret;
use serde_json::json;

#[tokio::test]
async fn should_return_200_valid_token() {
    let mut app        = TestApp::new().await;                     // Arrange
    let (_user, token) = setup_logged_in_user(&app).await;
    let body           = json!({"token": token});
    let response       = app.post_verify_token(&body).await;       // Act
    assert_status(&response, 200, None);                           // Assert
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_422_if_string_input() {
    let mut app      = TestApp::new().await;
    let body     = "malformed".to_owned();
    let response = app.post_verify_token(&body).await;
    assert_status(&response, 422, None);                           // Assert
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let mut app  = TestApp::new().await;
    let body     = json!(r#"{"malformed": "<-- that key should be 'token'"}"#);
    let response = app.post_verify_token(&body).await;             // Act
    assert_status(&response, 422, None);                           // Assert
    app.clean_up().await;
}


#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let mut app  = TestApp::new().await;
    let token    = "This is not a valid token";
    let body     = json!({"token": token});
    let response = app.post_verify_token(&body).await;             // Act
    assert_status(&response, 401, None);                           // Assert
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_401_if_token_is_banned() {
    let mut app         = TestApp::new().await;                   // Arrange
    let (_user, token)  = setup_logged_in_user(&app).await;
    let body            = json!({"token": token});
    let logout_response = app.post_logout().await;                // Ban token by logging out
    assert_status(&logout_response, 200, None);
    let response        = app.post_verify_token(&body).await;     // Act

    // Assert
    assert_status(&response, 401, None);
    {
        let secret        = Secret::new(token.clone());
        let banned_tokens = app.banned_tokens.read().await;
        assert!(banned_tokens.contains_token(&secret).await);
    }
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_401_if_banned_token_crt() {
    let mut app  = TestApp::new().await;
    let email    = get_random_email();
    let email    = Secret::new(email);
    let email    = Email::parse(email).unwrap();
    let token    = generate_jwt_auth_token(&email).unwrap();
    println!("token: {}", token);

    // Now that we have a token, before we post it, let's add it to the
    // banded tokens set. After that, we should get a 401.
    // We do this in a nested scope so our write lock is dropped before
    // we post to the verify token endpoint.
    //
    {
        let mut tokens = app.banned_tokens.write().await;
        let secret     = Secret::new(token.clone());
        tokens.add_token(&secret).await.unwrap();
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
    app.clean_up().await;
}
