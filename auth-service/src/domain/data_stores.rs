
use super::email::Email;
use super::password::Password;
use super::user::User;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum UserStoreError 
{
    InvalidCredentials,
    UnexpectedError,
    UserAlreadyExists,
    UserNotFound,
}

#[async_trait::async_trait]
pub trait UserStore 
{
    async fn add_user(&mut self, user: User)                          -> Result<(),   UserStoreError>;
    async fn get_user(&self, email: &Email)                           -> Result<User, UserStoreError>;
    async fn validate_user(&self, email: &Email, password: &Password) -> Result<(),   UserStoreError>;
}
