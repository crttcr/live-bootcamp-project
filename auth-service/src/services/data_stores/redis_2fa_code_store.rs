use std::sync::Arc;
use redis::{Commands, Connection};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

const TEN_MINUTES_IN_SECONDS: u64  = 600;
const TWO_FA_CODE_PREFIX:     &str = ACTIVE_TOKEN_KEY_PREFIX;

use crate::domain::{
	data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError},
	Email,
};
use crate::utils::constants::ACTIVE_TOKEN_KEY_PREFIX;

pub struct RedisTwoFACodeStore {
	cx: Arc<RwLock<Connection>>,
}

impl RedisTwoFACodeStore {
	pub fn new(cx: Arc<RwLock<Connection>>) -> Self {
		Self { cx }
	}
}

#[async_trait::async_trait]
impl TwoFACodeStore for RedisTwoFACodeStore {
	async fn add_code(
		&mut self,
		email: Email,
		login_attempt_id: LoginAttemptId,
		code: TwoFACode,
	) -> Result<(), TwoFACodeStoreError> {
		let e_redis = |_: redis::RedisError| TwoFACodeStoreError::UnexpectedError;
		let e_serde = |_: serde_json::Error| TwoFACodeStoreError::UnexpectedError;
		let key     = get_key(TWO_FA_CODE_PREFIX, &email);
		let ttl     = TEN_MINUTES_IN_SECONDS;
		let tuple   = TwoFATuple(login_attempt_id.as_ref().to_owned(), code.as_ref().to_owned());
		let body    = serde_json::to_string(&tuple).map_err(e_serde)?;
		let _: ()   = self.cx.write().await.set_ex(&key, &body, ttl).map_err(e_redis)?;
		Ok(())
	}

	async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
		let ef      = |_: redis::RedisError| TwoFACodeStoreError::UnexpectedError; 
		let key     = get_key(TWO_FA_CODE_PREFIX, &email);
		let _: ()   = self.cx.write().await.del(&key).map_err(ef)?;
		Ok(())
	}

	async fn get_code(
		&self,
		email: &Email,
	) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
		let e_redis = |_: redis::RedisError| TwoFACodeStoreError::LoginAttemptIdNotFound;
		let e_serde = |_: serde_json::Error| TwoFACodeStoreError::UnexpectedError;
		let eparse  = |_: String           | TwoFACodeStoreError::UnexpectedError;
		let key     = get_key(TWO_FA_CODE_PREFIX, &email);
		let body    = self.cx.write().await.get::<_, String>(&key).map_err(e_redis)?;
		let object  = serde_json::from_str(&body).map_err(e_serde)?;
		let tuple   = TwoFATuple::from(object);
		let id      = LoginAttemptId::parse(tuple.0).map_err(eparse)?;
		let code    = TwoFACode::parse(tuple.1).map_err(eparse)?;
		Ok((id, code))
	}
}


#[derive(Serialize, Deserialize)]
struct TwoFATuple(pub String, pub String);


fn get_key(prefix: &str, email: &Email) -> String {
	format!("{}:{}", prefix, email.as_ref())
}