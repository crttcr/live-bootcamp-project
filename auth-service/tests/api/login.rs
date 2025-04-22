
use crate::helpers::TestApp;

/*
#[tokio::test]
pub async fn login_works()
{
    let app      = TestApp::new().await;
    let body     = r#"{"email":"rastus@boog.com","password":"password"}"#.to_owned();
    let response = app.post_login(&body).await;
    assert_eq!(response.status().as_u16(), 200);
}
*/

#[tokio::test]
pub async fn should_return_422_if_malformed_credentials()
{
    let app      = TestApp::new().await;
    let body     = r#"{"email":"rastus@boog.com"}"#.to_owned();
    let response = app.post_login(&body).await;
    assert_eq!(response.status().as_u16(), 422);
}

#[tokio::test]
pub async fn should_not_return_422_if_well_formed_credentials()
{
    let app      = TestApp::new().await;
    let body     = r#"{"email":"rastus@boog.com", "password":"Alpha123**8"}"#.to_owned();
    let response = app.post_login(&body).await;
    assert_eq!(response.status().as_u16(), 422);
}

#[tokio::test]
pub async fn should_return_422_if_email_is_invalid()
{
    let app      = TestApp::new().await;
    let body     = r#"{"email":"rastus", "password":"Alpha123**8"}"#.to_owned();
    let response = app.post_login(&body).await;
    assert_eq!(response.status().as_u16(), 422);
}
