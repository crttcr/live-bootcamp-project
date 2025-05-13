use crate::domain::data_stores::{UserStore, UserStoreError};
use crate::domain::{Email, EmailError, Password, PasswordError, User};
use crate::utils::hash_utils;
use crate::utils::hash_utils::hash_password_async;
use color_eyre::eyre::{eyre, Result};
use log::debug;
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;

#[derive(Clone, Debug, sqlx::FromRow, serde::Deserialize, serde::Serialize)]
pub struct UserRecord {
	pub email:          String,
	pub password_hash:  String,
	pub requires_2fa:   bool,
}

impl UserRecord {
	pub fn into_user(self) -> Result<User, UserStoreError> {
		let e_email  = |e: EmailError|    UserStoreError::UnexpectedError(eyre!(e));
		let e_pword  = |e: PasswordError| UserStoreError::UnexpectedError(eyre!(e));
		let email    = Secret::new(self.email);
		let email    = Email::parse(email).map_err(e_email)?;
		let password = Secret::new(self.password_hash);
		let password = Password::parse(password).map_err(e_pword)?;
		let user     = User::new(email, password, self.requires_2fa);
		Ok(user)
	}
}

pub struct PostgresUserStore {
	pool: PgPool,
}

impl PostgresUserStore {
	pub fn new(pool: PgPool) -> Self {
		Self { pool }
	}
}

#[async_trait::async_trait]
impl UserStore for PostgresUserStore {
	#[tracing::instrument(name = "Add user to PostgreSQL", skip_all)] // New!
	async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
		// Make sure we do not have this user in the database ...
		let lookup = self.get_user(&user.email).await;
		if lookup.is_ok() {
			debug!("UserStore.add_user: User already exists");
			return Err(UserStoreError::UserAlreadyExists);
		}

		let email          = user.email.as_ref();
		let password       = user.password.expose().to_owned();
		let hash_result    = hash_password_async(password).await;
		let hash           = hash_result.map_err(UserStoreError::UnexpectedError)?;
		sqlx::query(
			r#"
	        INSERT INTO users (email, password_hash, requires_2fa)
	        VALUES ($1, $2, $3)
	        "#
			)
			.bind(&email.expose_secret())
			.bind(hash)
			.bind(user.requires_2fa)
			.execute(&self.pool)
			.await
			.map_err(|e| UserStoreError::UnexpectedError(e.into()))?;
			Ok(())
	}

	#[tracing::instrument(name = "Retrieve user from PostgreSQL", skip_all)] // New!
	async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
		let email  = email.expose_secret().to_owned();
		let result = sqlx::query_as!(UserRecord, "SELECT email, password_hash, requires_2fa FROM users WHERE email = $1", email)
			.fetch_optional(&self.pool)
			.await.map_err(|e| UserStoreError::UnexpectedError(e.into()))?;
		match result {
			None      => Err(UserStoreError::UserNotFound),
			Some(dto) => match dto.into_user() {
				Ok(user) => Ok(user),
				Err(e)   => Err(e),
			}
		}
	}

	#[tracing::instrument(name = "Validate user credentials", skip_all)] // New!
	async fn validate_user(&self, email: &Email, password: &Password) -> Result<(), UserStoreError> {
		let password      = password.expose().to_owned();
		let user          = self.get_user(&email).await?;
		let password_hash = user.password.expose().to_owned();
		let result        = hash_utils::verify_password_async(password_hash, password).await;
		result.map_err(|_| UserStoreError::InvalidCredentials)
	}
}
