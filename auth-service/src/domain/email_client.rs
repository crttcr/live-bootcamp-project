use super::Email;
use color_eyre::eyre::Result;

// Interface concrete email clients should implement
//
#[async_trait::async_trait]
pub trait EmailClient 
{
	async fn send_email(&self, recipient: &Email, subject: &str, content: &str) -> Result<()>;
}
