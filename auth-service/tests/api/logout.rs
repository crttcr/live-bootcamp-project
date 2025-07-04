use crate::helpers_arrange::{add_token_to_cookie_jar, setup_logged_in_user, TestUser};
use crate::helpers_assert::assert_status;
use crate::helpers_harness::TestApp;
use auth_service::domain::Email;
use auth_service::utils::auth::generate_auth_cookie;
use auth_service::utils::constants::JWT_COOKIE_NAME;
use reqwest::Url;
use secrecy::Secret;

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    let mut app        = TestApp::new().await;
    let (_user, token) = setup_logged_in_user(&app).await;
    let response       = app.post_logout().await;                  // Act
    {
        let token       = Secret::new(token);
        let token_store = app.banned_tokens.read().await;
        assert_status(&response, 200, None);
        assert!(token_store.contains_token(&token).await);
    }
    app.clean_up().await;
}


#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let mut app  = TestApp::new().await;
    let response = app.post_logout().await;
    assert_status(&response, 400, Some("Missing JWT Cookie"));
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_401_if_invalid_token_crt() {
    let mut app    = TestApp::new().await;
    let bad_cookie =  &format!("{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/", JWT_COOKIE_NAME);
    let good_url   = Url::parse("http://127.0.0.1").expect("Failed to parse URL");
    app.cookie_jar.add_cookie_str(bad_cookie, &good_url);
    let response   = app.post_logout().await;
    assert_status(&response, 401, None);
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let mut app = TestApp::new().await;
    add_token_to_cookie_jar(&app, "invalid");
    let response = app.post_logout().await;               // Act
    assert_status(&response, 401, None);                  // Assert
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
    let mut app          = TestApp::new().await;
    let (_user, _token)  = setup_logged_in_user(&app).await;
    let logout_response1 = app.post_logout().await;                    // Act
    let logout_response2 = app.post_logout().await;

    assert_status(&logout_response1, 200, None);                       // Assert
    assert_status(&logout_response2, 400, None);
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row_crt() {
    let mut app     = TestApp::new().await;
    let user        = TestUser::new();
    let email       = Secret::new(user.email);
    let email       = Email::parse(email).unwrap();
    let good_cookie = generate_auth_cookie(&email).unwrap();
    let cookie_string = good_cookie.to_string();
    println!("{:?}", cookie_string);
    println!("{:?}", good_cookie);
    let good_url    = Url::parse("http://127.0.0.1").expect("Failed to parse URL");
    app.cookie_jar.add_cookie_str(cookie_string.as_str(), &good_url);
    let _          = app.post_logout().await;
    let response   = app.post_logout().await;
    assert_eq!(response.status(), 400);
    app.clean_up().await;
}
