use crate::helpers::TestApp;

use auth_service::routes::signup::SignupResponse;
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
pub async fn should_return_201_if_valid_input() {
    // Arrange
    let app          = TestApp::new().await;
    let random_email = get_random_email();
    let test_case    = serde_json::json!({
            "email":       random_email,
            "password":    "password123",
            "requires2FA": true,
        });
    // Act
    let response = app.post_signup(&test_case).await;
    let status   = response.status().as_u16();

    // Assert
    assert_eq!(status, 201, "Expected 201 Created, but got {}. Input: {:?}", response.status(), test_case);
} 

#[tokio::test]
pub async fn should_succeed_with_the_expected_result() {
    // Arrange
    let app          = TestApp::new().await;
    let random_email = get_random_email();
    let test_case    = serde_json::json!({
            "email":       random_email,
            "password":    "password123",
            "requires2FA": true,
        });

    // Act
    let response          = app.post_signup(&test_case).await;
    let message           = "User created successfully!".to_owned();
    let expected_response = SignupResponse{message};

    // Assert that we are getting the correct response body!
    assert_eq!(
        response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        expected_response
    );

}

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
