use crate::domain::data_stores::TokenStore;
use crate::domain::data_stores::TokenStoreError;
use crate::utils::constants::BANNED_TOKEN_KEY_PREFIX;
use color_eyre::eyre::WrapErr;
use color_eyre::eyre::{eyre, Result};
use redis::Commands;
use redis::Connection;
use std::convert::TryInto;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, warn};

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
    #[tracing::instrument(name = "add token", skip_all)]
    async fn add_token(&mut self, token: String) -> Result<(), TokenStoreError> {
        let key      = make_key(BANNED_TOKEN_KEY_PREFIX, token.as_str());
        debug!(?key, "Adding key in Redis");
        let ttl: u64 = BANNED_TOKEN_TTL_SECONDS.try_into().unwrap_or(60 * 60 * 2);
        let _: ()    = self.cx.write().await
           .set_ex(key, true, ttl)
           .wrap_err("Failed to add token to Redis")
           .map_err(TokenStoreError::UnexpectedError)?;
        Ok(())
    }

    // NOTE: This is not a production implementation.
    // KEYS blocks the Redis server and scans the entire keyspace.
    // For large production databases, this is dangerous
    //
    #[tracing::instrument(name = "count tokens", skip_all)]
    async fn count(&self) -> Result<u64, TokenStoreError> {
        let prefix  = BANNED_TOKEN_KEY_PREFIX;
        let pattern = format!("{}*", prefix);
        let mut cx  = self.cx.write().await;
        let keys: Vec<String> = cx.keys(pattern)
           .wrap_err("Failed to count banned tokens in Redis")
           .map_err(TokenStoreError::UnexpectedError)?;
        let count   = keys.len();
        Ok(count as u64)
    }

    #[tracing::instrument(name = "clear tokens", skip_all)]
    async fn clear(&mut self) -> Result<(), TokenStoreError> {
        warn!("Unsupported operation: Clearing all banned tokens.");
        let err = eyre!("Clearing all banned tokens is not supported.");
        Err(TokenStoreError::UnexpectedError(err))
    }

    #[tracing::instrument(name = "contains token", skip_all)]   
    async fn contains_token(&self, token: &str) -> bool {
        let key = make_key(BANNED_TOKEN_KEY_PREFIX, token);
        debug!(?key, "Checking key in Redis");
        self.cx.write().await.exists(key).unwrap_or_else(|_| false)
    }

    #[tracing::instrument(name = "delete token", skip_all)]
    async fn delete_token(&mut self, token: &str) -> Result<(), TokenStoreError> {
        let key   = make_key(BANNED_TOKEN_KEY_PREFIX, token);
        debug!(?key, "Deleting key in Redis");
        let _: () = self.cx
            .write().await
            .del(key)
            .wrap_err("Failed to delete token from Redis")   
            .map_err(TokenStoreError::UnexpectedError)?;
        Ok(())      
    }
}

fn make_key(prefix: &str, token: &str) -> String {
    format!("{}:{}", prefix, token)
}