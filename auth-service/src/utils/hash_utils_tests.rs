use super::hash_utils::verify_password_sync;
use super::hash_utils::{hash_password_async, hash_password_sync, verify_password_async};

#[test]
fn test_that_we_can_hash_a_password() {
	let password       = "SomePasswordForTest".to_owned();
	let result         = hash_password_sync(password);
	println!("{:?}", result);
	assert!(result.is_ok());
}

#[tokio::test]
async fn test_that_we_can_hash_a_password_async() {
	let password       = "SomePasswordForTest".to_owned();
	let result         = hash_password_async(password).await;
	println!("{:?}", result);
	assert!(result.is_ok());
}


#[test]
fn test_that_we_can_verify_a_password() {
	let password       = "SomePasswordForTest".to_owned();
	let existing_hash  = hash_password_sync(password.clone()).unwrap();
	let result         = verify_password_sync(existing_hash, password);
	println!("{:?}", result);
	assert!(result.is_ok());
}

#[test]
fn test_verify_password_hash_with_different_case() {
	let password       = "SomePasswordForTest".to_owned();
	let other_password = "ThisIsSomethingElse".to_owned();
	let existing_hash  = hash_password_sync(password.clone()).unwrap();
	let result         = verify_password_sync(existing_hash, other_password);
	println!("{:?}", result);
	assert!(result.is_err());
}

#[tokio::test]
async fn test_verify_async_works() {
	let password       = "SomePasswordForTest".to_owned();
	let existing_hash  = hash_password_sync(password.clone()).unwrap();
	let result         = verify_password_async(existing_hash, password).await;
	println!("{:?}", result);
	assert!(result.is_ok());
}

