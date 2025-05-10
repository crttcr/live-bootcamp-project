use crate::domain::data_stores::{UserStore, UserStoreError};
use crate::domain::{Email, Password, User};
use crate::utils::hash_utils;
use crate::utils::hash_utils::hash_password_async;
use sqlx::PgPool;

#[derive(Clone, Debug, sqlx::FromRow, serde::Deserialize, serde::Serialize)]
pub struct UserRecord {
	pub email:          String,
	pub password_hash:  String,
	pub requires_2fa:   bool,
}

impl UserRecord {
	pub fn into_user(self) -> User {
		let email    = Email::parse(self.email).unwrap();
		let password = Password::parse(self.password_hash.as_str()).unwrap();
		User::new(email, password, self.requires_2fa)
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
			println!("UserStore.add_user({}): User exists: {:?}", user.email, lookup.unwrap());
			return Err(UserStoreError::UserAlreadyExists);
		}

		let email          = user.email.as_ref();
		let password       = user.password.to_string();
		let hash_result    = hash_password_async(password).await;
		let hash           = hash_result.map_err(|_| UserStoreError::UnexpectedError)?;

		sqlx::query(
			r#"
	        INSERT INTO users (email, password_hash, requires_2fa)
	        VALUES ($1, $2, $3)
	        "#
			)
			.bind(&email)
			.bind(hash)
			.bind(user.requires_2fa)
			.execute(&self.pool)
			.await
			.map_err(|_| UserStoreError::UnexpectedError)?;
			Ok(())
	}

	#[tracing::instrument(name = "Retrieve user from PostgreSQL", skip_all)] // New!
	async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
		let email  = email.get_email().to_owned();
		let result = sqlx::query_as!(UserRecord, "SELECT email, password_hash, requires_2fa FROM users WHERE email = $1", email)
			.fetch_optional(&self.pool)
			.await.map_err(|_| UserStoreError::UnexpectedError)?;
		match result {
			None      => Err(UserStoreError::UserNotFound),
			Some(dto) => Ok(dto.into_user())
		}
	}

	#[tracing::instrument(name = "Validate user credentials", skip_all)] // New!
	async fn validate_user(&self, email: &Email, password: &Password) -> Result<(), UserStoreError> {
		let password_str  = password.to_string();
		let user          = self.get_user(&email).await?;
		let password_hash = user.password.to_string();
		let result        = hash_utils::verify_password_async(password_hash, password_str).await;
		result.map_err(|_| UserStoreError::InvalidCredentials)
	}
}
