// TODO: Remove this after work is done
#![allow(unused)]


use crate::domain::user::User;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum UserStoreError 
{
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Default)]
pub struct HashmapUserStore 
{
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
         let rv = if self.users.contains_key(&user.email) {
               Err(UserStoreError::UserAlreadyExists)
         } else {
            let email = user.email.clone();
            self.users.insert(email, user);
            Ok(())
         };
         rv
    }

    pub fn get_user(&self, email: &str) -> Result<&User, UserStoreError> {
        self.users.get(email).ok_or(UserStoreError::UserNotFound)
    }

    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        match self.users.get(email) {
            Some(user) if user.password == password => Ok(()),
            Some(_)                                 => Err(UserStoreError::InvalidCredentials),
            None                                    => Err(UserStoreError::UserNotFound),
        }
    }
}
