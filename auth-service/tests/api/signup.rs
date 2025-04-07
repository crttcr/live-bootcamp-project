use crate::helpers::TestApp;
use crate::helpers::get_random_email;

/*
#[tokio::test]
pub async fn signup_should_return_200()
{
    let app      = TestApp::new().await;
    let response = app.post_signup().await;

    assert_eq!(response.status().as_u16(), 200);
//    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}
 */

#[tokio::test]
pub async fn signup_should_return_442_upon_malformed_imput()
{
    let app          = TestApp::new().await;
    let random_email = get_random_email();
    let test_cases   = [
        serde_json::json!({
            "email": random_email,
            "requires2FA": true,
        }),
        serde_json::json!({
            "password": "password123",
            "requires2FA": true,
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(&test_case).await;
        assert_eq!(
            response.status().as_u16(), 
            422,
            "Expected 422 Unprocessable Entity, but got {}. Input: {:?}",
            response.status(),
            test_case
        );
    }
}
