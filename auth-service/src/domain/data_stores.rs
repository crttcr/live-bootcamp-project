
use crate::domain::user::User;
use super::password::Password;
use super::email::Email;


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum UserStoreError 
{
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[async_trait::async_trait]
pub trait UserStore 
{
    async fn add_user(&mut self, user: User)                          -> Result<(),   UserStoreError>;
    async fn get_user(&self, email: &Email)                           -> Result<User, UserStoreError>;
    async fn validate_user(&self, email: &Email, password: &Password) -> Result<(),   UserStoreError>;
}
