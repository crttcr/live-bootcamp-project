
use std::sync::Arc;
use redis::Commands;
use redis::Connection;
use tokio::sync::RwLock;


use crate::domain::data_stores::TokenStore;
use crate::domain::data_stores::TokenStoreError;
use crate::utils::constants::BANNED_TOKEN_KEY_PREFIX;

const BANNED_TOKEN_TTL_SECONDS: u64 = 60 * 60 * 2; // 2 hours

pub struct RedisBannedTokenStore {
    cx: Arc<RwLock<Connection>>,
}

impl RedisBannedTokenStore {
    pub fn new(cx: Arc<RwLock<Connection>>) -> Self {
        Self {cx}
    }
}

#[async_trait::async_trait]
impl TokenStore for RedisBannedTokenStore {
    async fn add_token(&mut self, token: String) -> Result<(), TokenStoreError> {
        let key      = get_key(BANNED_TOKEN_KEY_PREFIX, token.as_str());
        let ttl: u64 = BANNED_TOKEN_TTL_SECONDS.try_into().unwrap_or(60 * 60 * 2);
        let _: ()    = self.cx.write().await
           .set_ex(key, true, ttl)
           .map_err(|_| TokenStoreError::UnexpectedError)?;
        Ok(())
    }

    // NOTE: This is not a production implementation.
    // KEYS blocks the Redis server and scans the entire keyspace.
    // For large production databases, this is dangerous
    //
    async fn count(&self) -> Result<u64, TokenStoreError> {
        let prefix  = BANNED_TOKEN_KEY_PREFIX;
        let pattern = format!("{}*", prefix);
        let mut cx  = self.cx.write().await;
        let keys: Vec<String> = cx.keys(pattern).map_err(|_| TokenStoreError::UnexpectedError)?;
        let count   = keys.len();
        Ok(count as u64)
    }

    async fn clear(&mut self) -> Result<(), TokenStoreError> {
        Err(TokenStoreError::UnexpectedError)
    }
    
    async fn contains_token(&self, token: &str) -> bool {
        let key = get_key(BANNED_TOKEN_KEY_PREFIX, token);
        self.cx.write().await.exists(key).unwrap_or_else(|_| false)
    }
    
    async fn delete_token(&mut self, token: &str) -> Result<(), TokenStoreError> {
        let key   = get_key(BANNED_TOKEN_KEY_PREFIX, token);
        let _: () = self.cx
           .write().await
           .del(key)
            .map_err(|_| TokenStoreError::UnexpectedError)?;
        Ok(())      
    }
}

fn get_key(prefix: &str, token: &str) -> String {
    format!("{}:{}", prefix, token)
}