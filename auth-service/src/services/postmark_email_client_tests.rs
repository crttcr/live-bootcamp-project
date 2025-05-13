use crate::utils::constants::test;

use crate::services::postmark_email_client::*;
use fake::faker::internet::en::SafeEmail;
use fake::faker::lorem::en::{Paragraph, Sentence};
use fake::{Fake, Faker};
use reqwest::Client;
use secrecy::Secret;
use wiremock::matchers::{any, header, header_exists, method, path};
use wiremock::{Mock, MockServer, Request, ResponseTemplate};
use crate::domain::Email;
use crate::services::postmark_email_client::POSTMARK_AUTH_HEADER;
use crate::domain::email_client::EmailClient;

// Generate fake data for email testing ...
// 
fn subject() -> String { Sentence( 1.. 2).fake() }
fn content() -> String { Paragraph(1..10).fake() }
fn email()   -> Email  {
	let email: String = SafeEmail().fake();
	let email         = Secret::new(email);
	let email         = Email::parse(email).unwrap();
	email
}

// Helper function to create a test email client
fn email_client(base_url: String) -> PostmarkEmailClient {
	let http_client = Client::builder()
		.timeout(test::email_client::TIMEOUT)
		.build()
		.unwrap();
	PostmarkEmailClient::new(base_url, email(), Secret::new(Faker.fake()), http_client)
}

// Custom matcher to validate the email request body
struct SendEmailBodyMatcher;
impl wiremock::Match for SendEmailBodyMatcher {
	fn matches(&self, request: &Request) -> bool {
		let result: Result<serde_json::Value, _> = serde_json::from_slice(&request.body);
		if let Ok(body) = result {
			body.get("From").is_some()
				&& body.get("To").is_some()
				&& body.get("Subject").is_some()
				&& body.get("HtmlBody").is_some()
				&& body.get("TextBody").is_some()
				&& body.get("MessageStream").is_some()
		} else {
			false
		}
	}
}

#[tokio::test]
async fn send_email_sends_the_expected_request() {
	let mock_server  = MockServer::start().await;
	let mock_uri     = mock_server.uri();
	let email_client = email_client(mock_uri);

	// Set up the mock server to expect a specific request
	Mock::given(header_exists(POSTMARK_AUTH_HEADER))
		.and(header("Content-Type", "application/json"))
		.and(path("/email"))
		.and(method("POST"))
		.and(SendEmailBodyMatcher)
		.respond_with(ResponseTemplate::new(200))
		.expect(1)
		.mount(&mock_server)
		.await;

	// Execute the send_email function and check the outcome
	let outcome = email_client.send_email(&email(), &subject(), &content()).await;
	assert!(outcome.is_ok());
}

// Test to handle server error responses
#[tokio::test]
async fn send_email_fails_if_the_server_returns_500() {
	let mock_server  = MockServer::start().await;
	let mock_uri     = mock_server.uri();
	let email_client = email_client(mock_uri);

	Mock::given(any())                                       // Set up the mock server to respond with 500
		.respond_with(ResponseTemplate::new(500))
		.expect(1)
		.mount(&mock_server)
		.await;

	let outcome = email_client .send_email(&email(), &subject(), &content()).await;
	assert!(outcome.is_err());
}

// Test to handle request timeouts
#[tokio::test]
async fn send_email_times_out_if_the_server_takes_too_long() {
	let mock_server  = MockServer::start().await;
	let mock_uri     = mock_server.uri();
	let email_client = email_client(mock_uri);

	// Set up the mock server to delay the response
	let delay    = std::time::Duration::from_secs(180); // Delay 3 minutes
	let response = ResponseTemplate::new(200).set_delay(delay);
	Mock::given(any())
		.respond_with(response)
		.expect(1)
		.mount(&mock_server)
		.await;

	// Execute the send_email function and check the outcome
	let outcome = email_client .send_email(&email(), &subject(), &content()).await;
	assert!(outcome.is_err());
}