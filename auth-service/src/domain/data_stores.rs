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

#[derive(Debug, PartialEq)]
pub enum TwoFACodeStoreError {
    LoginAttemptIdNotFound,
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
    async fn contains_token(&self, token: &str)     -> Result<bool, TokenStoreError>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoginAttemptId(String);

impl LoginAttemptId {
    pub fn new() -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        Self(id)
    }

    pub fn parse(id: String) -> Result<Self, String> {
        let _ = uuid::Uuid::parse_str(&id).map_err(|e| format!("Invalid uuid {}", e))?;
        Ok(Self(id))
    }
    
    pub fn is_match(&self, v: &str) -> bool {
        v == self.0
    }
}

impl Default for LoginAttemptId {
    fn default() -> Self { LoginAttemptId::new() }
}

impl AsRef<str> for LoginAttemptId {
    fn as_ref(&self) -> &str { &self.0 }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TwoFACode(String);

impl AsRef<str> for TwoFACode {
    fn as_ref(&self) -> &str { &self.0 }
}

impl TwoFACode {
    pub fn new() -> Self {
        use rand::Rng;
        let mut rng     = rand::thread_rng();
        let number: u32 = rng.gen_range(0..1_000_000);
        let code        = format!("{:06}", number); // Always 6 digits, zero-padded
        Self(code)
    }
    
    pub fn parse(code: String) -> Result<Self, String> {
        if !code.chars().all(|c| c.is_digit(10))   { return Err(format!("Code must contain only digits. Value({})", code)); }
        if code.len() != 6                         { return Err(format!("Code must be exactly 6 digits. Value({})", code)); }
        Ok(Self(code))
    }
    
    pub fn is_match(&self, v: &str) -> bool {
        v == self.0
    }
}

impl Default for TwoFACode {
    fn default() -> Self { TwoFACode::new() }
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
