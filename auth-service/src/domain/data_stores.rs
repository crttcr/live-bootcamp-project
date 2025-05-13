use super::email::Email;
use super::password::Password;
use super::user::User;
use color_eyre::eyre::{eyre, Context, Report, Result};
use secrecy::{Secret};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::fmt;

#[derive(Debug, Error)]
pub enum UserStoreError
{
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("User not found")]
    UserNotFound,
}

impl PartialEq for UserStoreError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (  Self::UserAlreadyExists,  Self::UserAlreadyExists )
            | (Self::UserNotFound,       Self::UserNotFound      )
            | (Self::InvalidCredentials, Self::InvalidCredentials)
            | (Self::UnexpectedError(_), Self::UnexpectedError(_))
            )
    }
}

#[derive(Debug, Error)]
pub enum TokenStoreError 
{
    #[error("Token is blank")]
    BlankToken,
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}

impl PartialEq for TokenStoreError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (  Self::BlankToken,         Self::BlankToken)
            | (Self::UnexpectedError(_), Self::UnexpectedError(_))
        )
    }
}

#[derive(Debug, Error)]
pub enum TwoFACodeStoreError {
    #[error("Invalid code")]
    LoginAttemptIdNotFound,
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}

impl PartialEq for TwoFACodeStoreError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (  Self::LoginAttemptIdNotFound, Self::LoginAttemptIdNotFound)
            | (Self::UnexpectedError(_),     Self::UnexpectedError(_)    )
        )
    }
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
    async fn add_token(&mut self, token: &Secret<String>)    -> Result<(),   TokenStoreError>;
    async fn clear(&mut self)                                -> Result<(),   TokenStoreError>;
    async fn count(&self)                                    -> Result<u64,  TokenStoreError>;
    async fn delete_token(&mut self, token: &Secret<String>) -> Result<(),   TokenStoreError>;
    async fn contains_token(&self, token: &Secret<String>)   -> bool;
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LoginAttemptId(String);

impl LoginAttemptId {
    pub fn new() -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        Self(id)
    }

    pub fn parse(id: String) -> Result<Self> {
        let id = uuid::Uuid::parse_str(&id).wrap_err("Invalid LoginAttemptId")?;
        let id = id.to_string();
        Ok(Self(id))
    }

    pub fn is_match(&self, other: &LoginAttemptId) -> bool {
        self.0 == other.0
    }

    pub fn is_match_str(&self, v: &str) -> bool {
        self.0 == v
    }
}

impl Default for LoginAttemptId {
    fn default() -> Self { LoginAttemptId::new() }
}

impl AsRef<String> for LoginAttemptId {
    fn as_ref(&self) -> &String { &self.0 }
}

impl fmt::Display for LoginAttemptId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TwoFACode(String);

impl AsRef<String> for TwoFACode {
    fn as_ref(&self) -> &String { &self.0 }
}

impl TwoFACode {
    pub fn new() -> Self {
        use rand::Rng;
        let mut rng     = rand::thread_rng();
        let number: u32 = rng.gen_range(0..1_000_000);
        let code        = format!("{:06}", number); // Always 6 digits, zero-padded
        Self(code)
    }
    
    pub fn parse(code: String) -> Result<Self> {
        if !code.chars().all(|c| c.is_digit(10))   { return Err(eyre!(format!("Code must contain only digits. Value({})", code))); }
        if code.len() != 6                         { return Err(eyre!(format!("Code must be exactly 6 digits. Value({})", code))); }
        Ok(Self(code))
    }
    pub fn is_match(&self, other: &TwoFACode) -> bool {
        self.0 == other.0
    }
    pub fn is_match_str(&self, v: &str) -> bool {
        self.0 == v
    }
}

impl Default for TwoFACode {
    fn default() -> Self { TwoFACode::new() }
}

impl fmt::Display for TwoFACode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// The interface that concrete 2FA stores should implement.
//
#[async_trait::async_trait]
pub trait TwoFACodeStore {
    async fn add_code(&mut self,
        email:            Email,
        login_attempt_id: LoginAttemptId,
        code:             TwoFACode,
    ) -> Result<(), TwoFACodeStoreError>;
    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError>;
    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError>;
}
