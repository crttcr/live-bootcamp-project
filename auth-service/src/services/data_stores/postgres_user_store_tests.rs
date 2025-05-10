use crate::create_postgres_pool;
use crate::domain::{Email, Password, User, UserStore};
use crate::services::data_stores::postgres_user_store::PostgresUserStore;
use crate::utils::constants::DATABASE_URL;

#[tokio::test]
async fn query_for_non_existing_user_returns_none() {
	let email          = "nobody@nowhere.edu".to_owned();
	let email          = Email::parse(email).unwrap();
	let url            = DATABASE_URL.to_owned();
	println!("{:?}", url);
	let pg_pool        = create_postgres_pool(&url).await.unwrap();
	let user_store     = PostgresUserStore::new(pg_pool);
	let result         = user_store.get_user(&email).await;
	println!("{:?}", result);
	assert!(result.is_err());
}

/*
#[tokio::test]
async fn query_for_existing_user_returns_ok() {
	let email          = "one@two.com".to_owned();
	let email          = Email::parse(email).unwrap();
	let url            = DATABASE_URL.to_owned();
	println!("{:?}", url);
	let pg_pool        = create_postgres_pool(&url).await.unwrap();
	let user_store     = PostgresUserStore::new(pg_pool);
	let result         = user_store.get_user(&email).await;
	println!("{:?}", result);
	assert!(result.is_ok());
}
*/

/*
#[tokio::test]
async fn add_user_returns_ok_result() {
	let email          = "one@two.com".to_owned();
	let email          = Email::parse(email).unwrap();
	let password       = Password::parse("ArcBinCanDoe594***").unwrap();
	let user           = User::new(email, password, true);
	let url            = DATABASE_URL.to_owned();
	println!("{:?}", url);
	let pg_pool        = create_postgres_pool(&url).await.unwrap();
	let mut user_store = PostgresUserStore::new(pg_pool);
	let result         = user_store.add_user(user).await;
	println!("{:?}", result);
	assert!(result.is_ok());
}
*/

#[tokio::test]
async fn add_user_fails_when_email_already_exists() {
	let email          = "one@two.com".to_owned();
	let email          = Email::parse(email).unwrap();
	let password       = Password::parse("ArcBinCanDoe594***").unwrap();
	let user           = User::new(email, password, true);
	let url            = DATABASE_URL.to_owned();
	println!("Postgres url: {:?}", url);
	let pg_pool        = create_postgres_pool(&url).await.unwrap();
	let mut user_store = PostgresUserStore::new(pg_pool);
	let result         = user_store.add_user(user).await;
	println!("{:?}", result);
	assert!(result.is_err());
}

/*
#[tokio::test]
async fn validate_user_password() {
	let email          = "one@two.com".to_owned();
	let email          = Email::parse(email).unwrap();
	let password       = Password::parse("ArcBinCanDoe594***").unwrap();
	let url            = DATABASE_URL.to_owned();
	let pg_pool        = create_postgres_pool(&url).await.unwrap();
	let user_store     = PostgresUserStore::new(pg_pool);
	let result         = user_store.validate_user(&email, &password).await;
	println!("{:?}", result);
	assert!(result.is_ok());
}
*/

#[tokio::test]
async fn validate_user_password_fails_when_wrong_password() {
	let email          = "one@two.com".to_owned();
	let email          = Email::parse(email).unwrap();
	let password       = Password::parse("This is not the password").unwrap();
	let url            = DATABASE_URL.to_owned();
	let pg_pool        = create_postgres_pool(&url).await.unwrap();
	let user_store     = PostgresUserStore::new(pg_pool);
	let result         = user_store.validate_user(&email, &password).await;
	println!("{:?}", result);
	assert!(result.is_err());
}
