use crate::helpers_harness::TestApp;

use crate::helpers_arrange::TestUser;
use crate::helpers_assert::{assert_error_message, assert_status};
use crate::helpers_harness::get_random_email;
use auth_service::routes::signup::SignupResponse;
use serde_json::json;

#[tokio::test]
pub async fn should_return_201_if_valid_input() {
    let app      = TestApp::new().await;
    let user     = TestUser::new();
    let response = app.post_signup(&user.signup_payload()).await; // Act
    assert_status(&response, 201, None);
}

#[tokio::test]
pub async fn should_succeed_with_the_expected_result() {
    // Arrange
    let app          = TestApp::new().await;
    let random_email = get_random_email();
    let test_case    = json!({
            "email":       random_email,
            "password":    "PassWord123!",
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
pub async fn should_return_422_upon_malformed_input()
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

// The signup route should return a 400 HTTP status code if an invalid input is sent.
// The input is considered invalid if:
// - The email is empty or does not contain '@'
// - The password is less than 8 characters
//
#[tokio::test]
async fn should_return_400_if_invalid_input() {
    // Create an array of invalid inputs. Then, iterate through the array and 
    // make HTTP calls to the signup route. Assert a 400 HTTP status code is returned.
    let bad_email    = json!({
        "email":       "doesnotcontainat.com",
        "password":    "password123",
        "requires2FA": true,
    });
    let empty_email = json!({
        "email":       "",
        "password":    "password123",
        "requires2FA": true,
    });
    let short_password = json!({
        "email":       "gt3@car.com",
        "password":    "123",
        "requires2FA": true,

    });

    let app        = TestApp::new().await;
    let test_cases = [bad_email, empty_email, short_password];
    for test_case in test_cases.iter() {
        let response = app.post_signup(&test_case).await;
        let context  = format!("Failed for request: {:?}", test_case);
        assert_status(&response, 400, Some(&context));
        assert_error_message(response, "Invalid credentials").await;
    }
}

#[tokio::test]
async fn should_return_400_if_invalid_input_jacob() {
    let app      = TestApp::new().await;
    let request  = json!({"email": "aaa", "password": "password", "requires2FA": true });
    let context  = format!("Failed for request: {:?}", request);
    let response = app.post_signup(&request).await;                       // Act
    assert_status( &response, 400, Some(&context));                       // Assert
    assert_error_message(response, "Invalid credentials").await;
}

#[tokio::test]
async fn should_return_409_if_email_already_exists() {
    let app  = TestApp::new().await;
    let user = TestUser::new();
    let r1   = app.post_signup(&user.signup_payload()).await;        // Act
    let r2   = app.post_signup(&user.signup_payload()).await;

    assert_status(&r1, 201, None);                                   // Assert
    assert_status(&r2, 409, None);
    assert_error_message(r2, "User already exists").await;
}
