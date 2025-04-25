use serde_json::json;
use crate::helpers::TestApp;
use auth_service::domain::Email;
use auth_service::utils::auth::generate_auth_token;
use crate::helpers::{get_random_email};

#[tokio::test]
async fn should_return_200_valid_token() {
    let app      = TestApp::new().await;
    let email    = get_random_email();
    let email    = Email::parse(email).unwrap();
    let token    = generate_auth_token(&email).unwrap();
    println!("token: {}", token);
    let body     = json!({"token": token});
    let response = app.post_verify_token(&body).await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_422_if_string_input() {
    let app      = TestApp::new().await;
    let body     = "malformed".to_owned();
    let response = app.post_verify_token(&body).await;
    assert_eq!(response.status().as_u16(), 422);
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app      = TestApp::new().await;
    let body     = json!(r#"{"malformed": "<-- that key should be 'token'"}"#);
    let response = app.post_verify_token(&body).await;
    assert_eq!(response.status().as_u16(), 422);
}


#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app      = TestApp::new().await;
    let token    = "This is not a valid token";
    let body     = json!({"token": token});
    let response = app.post_verify_token(&body).await;
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_401_if_banned_token() {
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

    // Now that we have added the token to the banned tokens set, let's
    // post to the verify token endpoint and ensure that we get a 401.
    //
    let body     = json!({"token": token});
    println!("created body {}", body);
    let response = app.post_verify_token(&body).await;
    println!("Asserting result");
    assert_eq!(response.status().as_u16(), 401);
}
