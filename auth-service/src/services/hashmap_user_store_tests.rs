
use core::panic;

use crate::domain::user::User;
use crate::services::hashmap_user_store::HashmapUserStore;
use crate::services::hashmap_user_store::UserStoreError;

#[tokio::test]
async fn test_add_user() {
   let (email, store) = construct_store_with_test_user();
   let hydrate        = store.get_user(email);

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
   let missing = store.get_user("missing");
   match missing {
      Err(UserStoreError::UserNotFound) => { println!("Correctly failed to find missing user"); }
      Err(e)                            => { panic!("Unexpected error: {:?}", e); }
      Ok(_)                             => { panic!("Missing user located");
      }
       
   }
}

#[tokio::test]
async fn test_user_can_be_retrieved_from_store() {
   let (email, store) = construct_store_with_test_user();
   let present        = store.get_user(email);
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
   let email     = "AbC";
   let password  = "1234";
   let user      = User::new(email.to_owned(), password.to_owned(), false);
   let _         = store.add_user(user);
   let result    = store.validate_user(email, password);
   match result {
      Ok(())  => { println!("Validated user with email {:?} using password {}", email, password); }
      Err(_)  => { panic!("User not found"); }
   }
}

fn construct_store_with_test_user<'a>() -> (&'a str, HashmapUserStore) {
   let mut store = HashmapUserStore::default();
   let email     = "AbC";
   let password  = "1234";
   let user      = User::new(email.to_owned(), password.to_owned(), false);
   let _         = store.add_user(user);
   (email, store)
}
