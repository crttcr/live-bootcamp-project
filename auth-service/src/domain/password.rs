
#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PasswordError {
    BlankValue,
    TooShort,
    Insecure,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Password(String);


impl Password {
    pub fn parse(password: &str) -> Result<Self, PasswordError> {
      if password.is_empty()          { return Err(PasswordError::BlankValue);      }  
      if password.len() < 8        { return Err(PasswordError::TooShort); }
      let _  = Password::is_secure(password)?;
      let rv = Password(password.to_owned());
      Ok(rv)
    }

    pub fn is_secure(password: &str) -> Result<(), PasswordError> {
         if password.len() < 8 { return Err(PasswordError::TooShort); }
         if !password.chars().any(|c| c.is_uppercase()) { return Err(PasswordError::Insecure); }
         if !password.chars().any(|c| c.is_lowercase()) { return Err(PasswordError::Insecure); }
         if !password.chars().any(|c| c.is_digit(10))   { return Err(PasswordError::Insecure); }
         if !password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;':\",.<>?/`~".contains(c)) {
               return Err(PasswordError::Insecure);
         }
         Ok(())
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}