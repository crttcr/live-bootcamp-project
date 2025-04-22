
pub use crate::domain::data_stores::UserStore;
pub use crate::domain::data_stores::UserStoreError;
use crate::domain::email::Email;
use crate::domain::password::Password;
use crate::domain::user::User;
use std::collections::HashMap;


#[derive(Default)]
pub struct HashmapUserStore 
{
    users: HashMap<Email, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> 
    {
        if self.users.contains_key(&user.email) {
            Err(UserStoreError::UserAlreadyExists)
        } else {
            let email = user.email.clone();
            self.users.insert(email, user);
            Ok(())
        }
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        self.users.get(email).cloned().ok_or(UserStoreError::UserNotFound)
    }

    async fn validate_user(&self, email: &Email, password: &Password) -> Result<(), UserStoreError> {
        match self.users.get(email) {
            Some(user) if user.password == *password => Ok(()),
            Some(_)                                 => Err(UserStoreError::InvalidCredentials),
            None                                    => Err(UserStoreError::UserNotFound),
        }
    }
}
