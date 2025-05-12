use std::sync::Arc;
use color_eyre::eyre::{eyre, Context};
use redis::{Commands, Connection};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::debug;

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

	#[tracing::instrument(name = "add code to redis", skip_all)]
	async fn add_code(
		&mut self,
		email: Email,
		login_attempt_id: LoginAttemptId,
		code: TwoFACode,
	) -> Result<(), TwoFACodeStoreError> {
		let key     = make_key(TWO_FA_CODE_PREFIX, &email);
		debug!(?key, ?email, ?login_attempt_id, ?code, "Adding code to redis");
		let ttl     = TEN_MINUTES_IN_SECONDS;
		let tuple   = TwoFATuple(login_attempt_id.as_ref().to_owned(), code.as_ref().to_owned());
		let body    = serde_json::to_string(&tuple)
			.wrap_err("Failed to serialize TwoFA code")
			.map_err(TwoFACodeStoreError::UnexpectedError)?;
		let _: ()   = self.cx.write().await
			.set_ex(&key, &body, ttl)
			.wrap_err("Failed to write TwoFA code to redis")
			.map_err(TwoFACodeStoreError::UnexpectedError)?;
		Ok(())
	}

	#[tracing::instrument(name = "remove code from redis", skip_all)]
	async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
		let key     = make_key(TWO_FA_CODE_PREFIX, &email);
		debug!(?key, "Key to remove");
		let _: ()   = self.cx.write().await
			.del(&key)
			.wrap_err("Failed to delete TwoFA from redis")
			.map_err(TwoFACodeStoreError::UnexpectedError)?;
		Ok(())
	}

	#[tracing::instrument(name = "lookup code in redis", skip_all)]
	async fn get_code(
		&self,
		email: &Email,
	) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
		let key     = make_key(TWO_FA_CODE_PREFIX, &email);
		debug!(?key, "Key to lookup");
		let body    = self.cx.write().await
			.get::<_, String>(&key)
			.wrap_err("Failed to get TwoFA bytes from redis")
			.map_err(TwoFACodeStoreError::UnexpectedError)?;
		let object  = serde_json::from_str(&body)
			.wrap_err("Failed to deserialize TwoFA tuple retrieved from redis")
			.map_err(TwoFACodeStoreError::UnexpectedError)?;
		let tuple   = TwoFATuple::from(object);
		let id      = LoginAttemptId::parse(tuple.0).map_err(TwoFACodeStoreError::UnexpectedError)?;
		let code    = TwoFACode::parse(tuple.1).map_err(TwoFACodeStoreError::UnexpectedError)?;
		Ok((id, code))
	}
}


#[derive(Serialize, Deserialize)]
struct TwoFATuple(pub String, pub String);


fn make_key(prefix: &str, email: &Email) -> String {
	format!("{}:{}", prefix, email.as_ref())
}
