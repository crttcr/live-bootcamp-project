use std::hash::{Hash, Hasher};
use std::fmt::Write;
use twox_hash::Xxh3Hash128;
use color_eyre::eyre::Result;
use secrecy::{ExposeSecret, Secret};
use thiserror::Error;
use tracing::debug;
use validator::validate_email;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum EmailError {
    #[error("Email format is not valid")]
    BadFormat,
    #[error("Email value is empty")]   
    EmptyValue,
    #[error("Email value is missing @ symbol")]  
    MissingAtSymbol,
}

#[derive(Clone, Debug)]
pub struct Email(Secret<String>);

impl Email {
    pub fn parse(email: Secret<String>) -> Result<Self, EmailError> {
        let s                = email.expose_secret();
        let parts: Vec<&str> = s.split('@').collect();
        let has_whitespace   = s.chars().any(char::is_whitespace);
        if s.is_empty()              { return Err(EmailError::EmptyValue);      }
        if has_whitespace            { return Err(EmailError::BadFormat);       }
        if !s.contains('@')          { return Err(EmailError::MissingAtSymbol); }
        if parts.len() != 2          { return Err(EmailError::BadFormat);       }
        if parts[0].starts_with('.') { return Err(EmailError::BadFormat);       }
        if parts[0].ends_with('.')   { return Err(EmailError::BadFormat);       }
        if !parts[1].contains('.')   { return Err(EmailError::BadFormat);       }
        if parts[1].starts_with('.') { return Err(EmailError::BadFormat);       }
        if parts[1].ends_with('.')   { return Err(EmailError::BadFormat);       }
        if validate_email(s) {
            Ok(Email(email))
        } else {
            let err = format!("Email format is not valid: {}", s);
            debug!(err);
            Err(EmailError::BadFormat)
        }
    }

    pub fn expose_secret(&self) -> &str {&self.0.expose_secret()}
    pub fn hash_secret_twox128(&self) -> String {
        let mut hasher = Xxh3Hash128::with_seed(0); // Seed = 0 for stability
        self.0.expose_secret().hash(&mut hasher);
        let hash    = hasher.finish();
        let mut buf = String::with_capacity(32);
        write!(&mut buf, "{:032x}", hash).unwrap();
        buf
    }
}

impl Eq        for Email {}
impl PartialEq for Email {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}

impl Hash for Email {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.expose_secret().hash(state);
    }
}

impl AsRef<Secret<String>> for Email {
    fn as_ref(&self) -> &Secret<String> {
        &self.0
    }
}
