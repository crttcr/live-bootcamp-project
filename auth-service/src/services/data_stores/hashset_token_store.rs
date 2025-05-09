use std::collections::HashSet;
use crate::domain::{TokenStore, TokenStoreError};
use crate::domain::TokenStoreError::BlankToken;

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
	async fn add_token(&mut self, token: String) -> Result<(), TokenStoreError> {
		if token.is_empty() { Err(BlankToken)                   }
		else                { self.tokens.insert(token); Ok(()) }
	}

	async fn clear(&mut self) -> Result<(), TokenStoreError> {
		self.tokens.clear();
		Ok(())
	}
	async fn count(&self) -> Result<u64, TokenStoreError> {
		Ok(self.tokens.len() as u64)
	}	
	async fn delete_token(&mut self, token: &str) -> Result<(), TokenStoreError> {
		if token.is_empty() { Err(BlankToken)                   }
		else                { self.tokens.remove(token); Ok(()) }
	}
	
	async fn contains_token(&self, token: &str) -> bool {
		self.tokens.contains(token)
	}
}
