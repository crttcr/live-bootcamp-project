
use crate::domain::password::{Password, PasswordError};


#[test]
pub fn parse_empty_string_fails_as_expected()
{
    let input   = "";
    let output  = Password::parse(input);
      match output {
         Ok(_) =>  { panic!("Password should be invalid"); }
         Err(e) => { assert_eq!(e, PasswordError::BlankValue); }
      }
}

#[test]
pub fn password_of_7_characters_is_too_short()
{
    let input   = "1a3b5c7";
    let output  = Password::parse(input);
      match output {
         Ok(_) =>  { panic!("Password should be invalid"); }
         Err(e) => { assert_eq!(e, PasswordError::TooShort); }
      }
}

#[test]
pub fn password_without_uppercase_is_insecure()
{
    let input   = "abcd1234@@";
    let output  = Password::parse(input);
      match output {
         Ok(_) =>  { panic!("Password should be invalid"); }
         Err(e) => { assert_eq!(e, PasswordError::Insecure); }
      }
}

#[test]
pub fn password_without_lowercase_is_insecure()
{
    let input   = "ABCD1234@@";
    let output  = Password::parse(input);
      match output {
         Ok(_) =>  { panic!("Password should be invalid"); }
         Err(e) => { assert_eq!(e, PasswordError::Insecure); }
      }
}

#[test]
pub fn password_without_digits_is_insecure()
{
    let input   = "ABCDabcd@@";
    let output  = Password::parse(input);
      match output {
         Ok(_) =>  { panic!("Password should be invalid"); }
         Err(e) => { assert_eq!(e, PasswordError::Insecure); }
      }
}

#[test]
pub fn password_without_symbols_is_insecure()
{
    let input   = "ABCDabcd1234";
    let output  = Password::parse(input);
      match output {
         Ok(_) =>  { panic!("Password should be invalid"); }
         Err(e) => { assert_eq!(e, PasswordError::Insecure); }
      }
}

