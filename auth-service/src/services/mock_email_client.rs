use crate::domain::{Email, EmailClient};

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
	) -> Result<(), String> {
		println!(
			"Sending email to {} with subject: {} and content: {}",
			recipient.as_ref(),
			subject,
			content
		);
		Ok(())
	}
}
