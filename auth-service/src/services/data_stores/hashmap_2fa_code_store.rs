use std::collections::HashMap;
use color_eyre::eyre::eyre;
use crate::domain::{
	data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError},
	email::Email,
};

#[derive(Default)]
pub struct HashmapTwoFACodeStore {
	codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

impl HashmapTwoFACodeStore {
	pub fn new() -> Self {
		Self::default()
	}
}

#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
	async fn add_code(&mut self, email: Email, login_attempt_id: LoginAttemptId, code: TwoFACode)
		-> Result<(), TwoFACodeStoreError>
	{
		let value = (login_attempt_id, code);
		println!("Adding value to store: {:?}", value);
		self.codes.insert(email, value);
		println!("Count                : {:?}", self.codes.len());
		Ok(())
	}

	async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
		match self.codes.remove(email) {
			Some(_) => Ok(()),
			None    => Err(TwoFACodeStoreError::UnexpectedError(eyre!("No token to remove"))),
		}
	}

	async fn get_code(&self, email: &Email) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
		match self.codes.get(email) {
			Some(v) => Ok(v.clone()),
			None    => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
		}
	}
}
