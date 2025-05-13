use crate::domain::{Email, EmailClient};
use color_eyre::Result;
use tracing::debug;

// MockEmailClient simply logs the recipient, subject, and content to standard output
//
pub struct MockEmailClient;

impl MockEmailClient {
	pub fn new() -> Self {Self{}}
}

#[async_trait::async_trait]
impl EmailClient for MockEmailClient 
{
	async fn send_email(
		&self,
		recipient:     &Email,
		subject:       &str,
		content:       &str,
	) -> Result<()> {
		let email = recipient.expose_secret();
		debug!(
			"Sending email to {} with subject: {} and content: {}",
			email,
			subject,
			content
		);
		Ok(())
	}
}
