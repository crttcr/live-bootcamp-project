use super::email::Email;
use super::password::Password;

#[derive(Debug, Clone, PartialEq)]
pub struct User {
      pub email:        Email,
      pub password:     Password,
//      pub salt:         String,
      pub requires_2fa: bool,
}

impl User {
   pub fn new(email: Email, password: Password, requires_2fa: bool) -> Self {
      User {email, password, requires_2fa}
   }
   // pub fn new(email: Email, password: Password, salt: String, requires_2fa: bool) -> Self {
   //    User {email, password, salt, requires_2fa}
   // }
}
