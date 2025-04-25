use validator::validate_email;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmailError {
    BadFormat,
    EmptyValue,
    MissingAtSymbol,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Email(String);


impl Email {
    pub fn parse(s: String) -> Result<Self, EmailError> {
        let has_whitespace = s.chars().any(char::is_whitespace);
        if s.is_empty()          { return Err(EmailError::EmptyValue);      }
        if has_whitespace            { return Err(EmailError::BadFormat);       }
        if !s.contains('@')      { return Err(EmailError::MissingAtSymbol); }
        let parts: Vec<&str> = s.split('@').collect();
        if parts.len() != 2 { return Err(EmailError::BadFormat); }
        if parts[0].starts_with('.') { return Err(EmailError::BadFormat); }
        if parts[0].ends_with('.')   { return Err(EmailError::BadFormat); }
        if !parts[1].contains('.')   { return Err(EmailError::BadFormat); }
        if parts[1].starts_with('.') { return Err(EmailError::BadFormat); }
        if parts[1].ends_with('.')   { return Err(EmailError::BadFormat); }
        
        if validate_email(&s) {
            Ok(Email(s))
        } else {
            Err(EmailError::BadFormat)
        }

        // match validate_email(email) {
        //     false => return Err(EmailError::BadFormat),
        //     true  => {}
        // }
        // let rv = Email(email.to_owned());
        // Ok(rv)
    }

    pub fn get_email(&self) -> &str {&self.0}
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
   }
}
