// TODO: Remove this after work is done
#![allow(unused)]

use async_trait::async_trait;
use crate::domain::user::User;
use crate::domain::data_stores::UserStoreError;
use crate::domain::data_stores::UserStore;
use std::collections::HashMap;


#[derive(Default)]
pub struct HashmapUserStore 
{
    users: HashMap<String, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
         let rv = if self.users.contains_key(&user.email) {
               Err(UserStoreError::UserAlreadyExists)
         } else {
            let email = user.email.clone();
            self.users.insert(email, user);
            Ok(())
         };
         rv
    }

    async fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        self.users.get(email).cloned().ok_or(UserStoreError::UserNotFound)
    }

    async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        match self.users.get(email) {
            Some(user) if user.password == password => Ok(()),
            Some(_)                                 => Err(UserStoreError::InvalidCredentials),
            None                                    => Err(UserStoreError::UserNotFound),
        }
    }
}
