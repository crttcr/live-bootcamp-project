use crate::domain::{TokenStore};
use crate::domain::TokenStoreError::BlankToken;
use crate::services::data_stores::hashset_token_store::HashSetTokenStore;
use fake::{Fake, faker::internet::en::Password};

#[tokio::test]
async fn test_new_token_store_has_no_tokens() {
	let store   = HashSetTokenStore::new();
	let count   = store.count().await.unwrap();
	assert_eq!(count, 0)
}

#[tokio::test]
async fn test_storing_an_empty_token_is_an_error() {
	let mut store = HashSetTokenStore::new();
	let empty     = "".to_string();
	let error     = store.add_token(empty).await.err().unwrap();
	assert_eq!(error, BlankToken);
}

#[tokio::test]
async fn test_random_token_is_not_found_in_empty_store() {
	let store  = HashSetTokenStore::new();
	let rando  = Password(12..16).fake::<String>().to_owned();
	let exists = store.contains_token(&rando).await;
	assert_eq!(exists, false);
}

#[tokio::test]
async fn test_a_stored_token_exists() {
	let mut store = HashSetTokenStore::new();
	let token     = "Dingle".to_string();
	let _         = store.add_token(token.clone()).await.unwrap();
	let count     = store.count().await.unwrap();
	let exists    = store.contains_token(&token).await;
	assert_eq!(count, 1);
	assert_eq!(exists, true);
}

#[tokio::test]
async fn storing_a_token_twice_does_not_increase_count() {
	let mut store = HashSetTokenStore::new();
	let token     = "Dingle".to_string();
	let _         = store.add_token(token.clone()).await.unwrap();
	let _         = store.add_token(token.clone()).await.unwrap();
	let count     = store.count().await.unwrap();
	let exists    = store.contains_token(&token).await;
	assert_eq!(count, 1);
	assert_eq!(exists, true);
}

#[tokio::test]
async fn deleting_a_token_decreases_count() {
	let mut store = HashSetTokenStore::new();
	let token     = "Dingle".to_string();
	let _         = store.add_token(token.clone()).await.unwrap();
	let was_there = store.contains_token(&token).await;
	let before    = store.count().await.unwrap();
	let _         = store.delete_token(&token).await.unwrap();
	let after     = store.count().await.unwrap();
	let is_there  = store.contains_token(&token).await;
	assert_eq!(before,       1);
	assert_eq!(after,        0);
	assert_eq!(was_there, true);
	assert_eq!(is_there, false);
}
