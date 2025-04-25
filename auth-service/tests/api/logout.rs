use crate::helpers::get_random_email;
use crate::helpers::TestApp;
use auth_service::domain::Email;
use auth_service::utils::auth::generate_auth_cookie;
use auth_service::utils::constants::JWT_COOKIE_NAME;
use reqwest::Url;

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    let app           = TestApp::new().await;
    let email         = get_random_email().to_owned();
    let email         = Email::parse(email).unwrap();
    let good_cookie   = generate_auth_cookie(&email).unwrap();
    let jwt_token     = good_cookie.value().to_owned();
    let cookie_string = good_cookie.to_string();
    println!("{:?}", jwt_token);
    println!("{:?}", cookie_string);
    println!("{:?}", good_cookie);
    let good_url    = Url::parse("http://127.0.0.1").expect("Failed to parse URL");
    app.cookie_jar.add_cookie_str(cookie_string.as_str(), &good_url);
    let response   = app.post_logout().await;
    let token_store = app.banned_tokens.read().await;
    let count      = token_store.count().await.unwrap();
    println!("Banned Tokens({:?})", count);
    let is_banned  = token_store.token_exists(&jwt_token).await.unwrap();
    assert!(is_banned);
    assert_eq!(response.status(),  200);
}


#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app      = TestApp::new().await;
    let response = app.post_logout().await;
    assert_eq!(response.status(), 400);
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app        = TestApp::new().await;
    let bad_cookie =  &format!("{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/", JWT_COOKIE_NAME);
    let good_url   = Url::parse("http://127.0.0.1").expect("Failed to parse URL");
    app.cookie_jar.add_cookie_str(bad_cookie, &good_url);
    let response   = app.post_logout().await;
    assert_eq!(response.status(), 401);
}

/*
    let cookie_string = format!("{}={}", cookie_name, jwt_token);
    let cookie = Cookie::parse_unencoded(cookie_string, Some(&url)).unwrap();
 */

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
    let app         = TestApp::new().await;
    let email       = get_random_email().to_owned();
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
}
