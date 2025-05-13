use crate::domain::TokenStoreError::BlankToken;
use crate::domain::{TokenStore, TokenStoreError};
use secrecy::{ExposeSecret, Secret};
use std::collections::HashSet;

#[derive(Default)]
pub struct HashSetTokenStore
{
	tokens: HashSet<String>,
}

impl HashSetTokenStore {
	pub fn new() -> Self {
		Self::default()
	}
}

#[async_trait::async_trait]
impl TokenStore for HashSetTokenStore {
	async fn add_token(&mut self, token: &Secret<String>) -> Result<(), TokenStoreError> {
		let secret = token.expose_secret();
		let copy   = secret.clone();
		if secret.is_empty() { Err(BlankToken)                   }
		else                { self.tokens.insert(copy); Ok(()) }
	}

	async fn clear(&mut self) -> Result<(), TokenStoreError> {
		self.tokens.clear();
		Ok(())
	}
	
	async fn count(&self) -> Result<u64, TokenStoreError> {
		Ok(self.tokens.len() as u64)
	}	
	async fn delete_token(&mut self, token: &Secret<String>) -> Result<(), TokenStoreError> {
		let token = token.expose_secret();
		if token.is_empty() { Err(BlankToken)                   }
		else                { self.tokens.remove(token); Ok(()) }
	}
	
	async fn contains_token(&self, token: &Secret<String>) -> bool {
		let token = token.expose_secret();
		self.tokens.contains(token)
	}
}
