use crate::domain::{Email, LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError};
use crate::services::data_stores::hashmap_2fa_code_store::HashmapTwoFACodeStore;

#[tokio::test]
async fn lookup_returns_not_found_for_empty_store() {
	let store   = HashmapTwoFACodeStore::default();
	let email   = Email::parse("a@b.com".to_owned()).unwrap();
	let missing = store.get_code(&email).await;
	match missing {
		Err(TwoFACodeStoreError::LoginAttemptIdNotFound) => { println!("Correctly failed to find code"); }
		Err(e)                                           => { core::panic!("Unexpected error: {:?}", e); }
		Ok(_)                                            => { core::panic!("Missing user located");
		}
	}
}

#[tokio::test]
async fn lookup_after_adding_code_returns_code() {
	let mut store  = HashmapTwoFACodeStore::default();
	let email      = Email::parse("a@b.com".to_owned()).unwrap();
	let code       = TwoFACode::default();
	let id         = LoginAttemptId::default();
	let add_result = store.add_code(email.clone(), id.clone(), code.clone()).await;
	assert!(add_result.is_ok());

	let returned = store.get_code(&email).await;
	println!("{:?}", returned);
	assert!(returned.is_ok());
	let returned = returned.unwrap();
	assert_eq!(returned.0,   id);
	assert_eq!(returned.1, code);
}

#[tokio::test]
async fn lookup_after_add_then_remove_returns_not_found() {
	let mut store  = HashmapTwoFACodeStore::default();
	let email      = Email::parse("a@b.com".to_owned()).unwrap();
	let code       = TwoFACode::default();
	let id         = LoginAttemptId::default();
	let add_result = store.add_code(email.clone(), id.clone(), code.clone()).await;
	assert!(add_result.is_ok());
	let del_result = store.remove_code(&email).await;
	assert!(del_result.is_ok());
	let returned   = store.get_code(&email).await;
	assert!(returned.is_err());
//	assert_eq!(returned, Err(TwoFACodeStoreError::LoginAttemptIdNotFound));
}
