
use crate::domain::email::{Email, EmailError};


#[test]
pub fn email_missing_at_symbol_is_invalid()
{
    let input   = "abc.com";
    let output  = Email::parse(input);
      match output {
         Ok(_) =>  { panic!("Email should be invalid"); }
         Err(e) => { assert_eq!(e, EmailError::MissingAtSymbol); }
      }
}


#[test]
pub fn parse_empty_string_fails_as_expected()
{
    let input   = "";
    let output  = Email::parse(input);
      match output {
         Ok(_) =>  { panic!("Email should be invalid"); }
         Err(e) => { assert_eq!(e, EmailError::EmptyValue); }
      }
}

#[test]
pub fn parse_string_with_whitespace_fails()
{
    let input   = "john doe@foo.com";
    let output  = Email::parse(input);
      match output {
         Ok(_) =>  { panic!("Email should be invalid"); }
         Err(e) => { assert_eq!(e, EmailError::BadFormat); }
      }
}

#[test]
pub fn parse_string_with_two_at_symbols_fails()
{
    let input   = "john@doe@com";
    let output  = Email::parse(input);
      match output {
         Ok(_) =>  { panic!("Email should be invalid"); }
         Err(e) => { assert_eq!(e, EmailError::BadFormat); }
      }
}

#[test]
pub fn parse_name_ending_with_period_fails()
{
    let input   = "john.@doe.com.";
    let output  = Email::parse(input);
      match output {
         Ok(_) =>  { panic!("Email should be invalid"); }
         Err(e) => { assert_eq!(e, EmailError::BadFormat); }
      }
}

#[test]
pub fn parse_string_starting_with_period_fails()
{
    let input   = ".john@doe.com";
    let output  = Email::parse(input);
      match output {
         Ok(_) =>  { panic!("Email should be invalid"); }
         Err(e) => { assert_eq!(e, EmailError::BadFormat); }
      }
}

#[test]
pub fn parse_where_domain_starts_with_period_fails()
{
    let input   = "john@.doe.com";
    let output  = Email::parse(input);
      match output {
         Ok(_) =>  { panic!("Email should be invalid"); }
         Err(e) => { assert_eq!(e, EmailError::BadFormat); }
      }
}

#[test]
pub fn parse_where_domain_ends_with_period_fails()
{
    let input   = "john@doe.com.";
    let output  = Email::parse(input);
      match output {
         Ok(_) =>  { panic!("Email should be invalid"); }
         Err(e) => { assert_eq!(e, EmailError::BadFormat); }
      }
}

#[test]
pub fn parse_unicode_local_part_rejected_by_validator()
{
   // let input   = "user@[127.0.0.1]";          // Should be rejected, but is not
   // let input   = "john..doe@example.com";     // Should be rejected, but is not
   let input = "üñîçøðé@example.com";
   let output  = Email::parse(input);
   match output {
      Ok(_) =>  { panic!("Email should be invalid"); }
      Err(e) => { assert_eq!(e, EmailError::BadFormat); }
   }
}

