use crate::helpers::TestApp;

use auth_service::domain::error::ErrorResponse;
use auth_service::routes::signup::SignupResponse;
use crate::helpers::get_random_email;
use serde_json::json;


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
    let test_case    = json!({
            "email":       random_email,
            "password":    "pAssword123!",
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
        assert_eq!(
            response.status().as_u16(), 
            400,
            "Expected 400 Bad Request, but got {}. Input: {:?}",
            response.status(),
            test_case
        );
        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Invalid credentials".to_owned()
        );
    }
}

#[tokio::test]
async fn should_return_409_if_email_already_exists() {
    let valid_input    = json!({
        "email":       "gt3@car.com",
        "password":    "PuckerStump@123!",
        "requires2FA": true,
    });

    // Calling the signup route twice. 
    // The second request should fail with a 409 HTTP (Conflict) status code    
    //
    let app      = TestApp::new().await;
    let _        = app.post_signup(&valid_input).await;
    let response = app.post_signup(&valid_input).await;
    assert_eq!(
        response.status().as_u16(), 
        409,
        "Expected 409 Conflict, but got {}. Input: {:?}",
        response.status(),
        valid_input
    );
    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "User already exists".to_owned()
    );
}
