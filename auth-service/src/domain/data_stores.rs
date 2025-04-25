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

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenStoreError 
{
    BlankToken,
    UnexpectedError,
}

#[async_trait::async_trait]
pub trait UserStore 
{
    async fn add_user(&mut self, user: User)                          -> Result<(),   UserStoreError>;
    async fn get_user(&self, email: &Email)                           -> Result<User, UserStoreError>;
    async fn validate_user(&self, email: &Email, password: &Password) -> Result<(),   UserStoreError>;
}

#[async_trait::async_trait]
pub trait TokenStore
{
    async fn add_token(&mut self, token: String)    -> Result<(),   TokenStoreError>;
    async fn clear(&mut self)                       -> Result<(),   TokenStoreError>;
    async fn count(&self)                           -> Result<u64,  TokenStoreError>;
    async fn delete_token(&mut self, token: &str)   -> Result<(),   TokenStoreError>;
    async fn token_exists(&self, token: &str)       -> Result<bool, TokenStoreError>;
}
