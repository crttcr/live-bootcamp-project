
use core::panic;

use crate::domain::user::User;
use crate::services::hashmap_user_store::HashmapUserStore;
use crate::domain::data_stores::UserStoreError;
use crate::domain::data_stores::UserStore;
use crate::domain::password::Password;
use crate::domain::email::Email;

#[tokio::test]
async fn test_add_user() {
   let (email, store) = construct_store_with_test_user().await;
   let hydrate        = store.get_user(&email).await;

   match hydrate {
      Ok(user) => {
         println!("Retrieved user: {:?}", user);
         assert_eq!(user.email, email);
         assert_eq!(user.requires_2fa, false);
      }
      Err(_) => {
         panic!("User not found");
      }
   }
}

#[tokio::test]
async fn test_failed_lookup_returns_user_not_found() {
   let store   = HashmapUserStore::default();
   let email   = Email::parse("somebody@missing.com").unwrap();
   let missing = store.get_user(&email).await;
   match missing {
      Err(UserStoreError::UserNotFound) => { println!("Correctly failed to find missing user"); }
      Err(e)                            => { panic!("Unexpected error: {:?}", e); }
      Ok(_)                             => { panic!("Missing user located");
      }
       
   }
}

#[tokio::test]
async fn test_user_can_be_retrieved_from_store() {
   let (email, store) = construct_store_with_test_user().await;
   let present        = store.get_user(&email).await;
   match present {
      Err(_)   => { panic!("User not found"); }
      Ok(user) => {
         println!("Successful user lookup: {:?}", user);
         assert_eq!(user.email, email);
      }
   }
}

#[tokio::test]
async fn test_validate_user() {
   let mut store = HashmapUserStore::default();
   let email     = Email::parse("joe@boo.io").unwrap();
   let password  = Password::parse("Horse1234!").unwrap();
   let user      = User::new(email.to_owned(), password.to_owned(), false);
   let _         = store.add_user(user).await;
   let result    = store.validate_user(&email, &password).await;
   match result {
      Ok(())  => { println!("Validated user with email {:?} using password {}", email, password); }
      Err(_)  => { panic!("User not found"); }
   }
}

async fn construct_store_with_test_user<'a>() -> (Email, HashmapUserStore) {
   let mut store = HashmapUserStore::default();
   let email     = Email::parse("joe@boo.io").unwrap();
   let password  = Password::parse("Horse1234!").unwrap();
   let user      = User::new(email.clone(), password, false);
   let _         = store.add_user(user).await;
   (email, store)
}
