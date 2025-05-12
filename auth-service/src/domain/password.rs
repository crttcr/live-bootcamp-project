use color_eyre::eyre::Result;
use secrecy::{ExposeSecret, Secret};
use thiserror::Error;
use log::debug;

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum PasswordError {
   #[error("Password is blank")]
    BlankValue,
   #[error("Password is insecure")]
    Insecure,
   #[error("Password is too short")]
    TooShort,
}

#[derive(Debug, Clone)]
pub struct Password(Secret<String>);

impl Password {
   pub fn parse(s: Secret<String>) -> Result<Self, PasswordError> {
      debug!("Parse   password: {}", s.expose_secret());
      println!("Parse   password: {}", s.expose_secret());
      let _ = Password::is_acceptable(&s)?;
//    let _ = Password::is_secure(    &s)?;
      Ok(Self(s))
   }

   pub fn expose(&self) -> &String {
      self.0.expose_secret()
   }  
   
   // Allow simple passwords for development/testing. It's too tedious to type and remember
   // secure passwords when the component is non-production
   //
   #[tracing::instrument(name = "Check password security", skip_all)]
   pub fn is_acceptable(password: &Secret<String>) -> Result<(), PasswordError> {
      let password = password.expose_secret();
      if password.is_empty()                         { return Err(PasswordError::BlankValue);      }
      if password.len() < 4                          { return Err(PasswordError::TooShort); }
      Ok(())
   }

   #[tracing::instrument(name = "Check password security", skip_all)]
   pub fn is_secure(password: &Secret<String>) -> Result<(), PasswordError> {
      let password = password.expose_secret();
      if password.is_empty()                         { return Err(PasswordError::BlankValue);      }
      if password.len() < 8                          { return Err(PasswordError::TooShort); }
      if !password.chars().any(|c| c.is_uppercase()) { return Err(PasswordError::Insecure); }
      if !password.chars().any(|c| c.is_lowercase()) { return Err(PasswordError::Insecure); }
      if !password.chars().any(|c| c.is_digit(10))   { return Err(PasswordError::Insecure); }
      if !password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;':\",.<>?/`~".contains(c)) {
         return Err(PasswordError::Insecure);
      }
      Ok(())
   }            
}

impl PartialEq for Password {
   fn eq(&self, other: &Self) -> bool {
      self.0.expose_secret() == other.0.expose_secret()      // Controlled access. No leaking.
   }
}

impl AsRef<Secret<String>> for Password {
    fn as_ref(&self) -> &Secret<String> {
        &self.0
    }
}
