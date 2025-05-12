use crate::domain::email::{Email, EmailError};
use fake::faker::internet::en::SafeEmail;
use fake::Fake;
use secrecy::Secret;

#[derive(Debug, Clone)]
struct ValidEmailFixture(pub String);

impl quickcheck::Arbitrary for ValidEmailFixture {
   fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
      let email = SafeEmail().fake_with_rng(g);
      Self(email)
   }
}

#[quickcheck_macros::quickcheck]
fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
   Email::parse(Secret::new(valid_email.0)).is_ok() // Updated!
}

#[test]
pub fn email_missing_at_symbol_is_invalid()
{
   let input   = "abc.com".to_owned();
   let input   = Secret::new(input);
   let output  = Email::parse(input);
   match output {
      Ok(_) =>  { panic!("Email should be invalid"); }
      Err(e) => { assert_eq!(e, EmailError::MissingAtSymbol); }
   }
}

#[test]
pub fn parse_empty_string_fails_as_expected()
{
   let input   = "".to_owned();
   let input   = Secret::new(input);
   let output  = Email::parse(input);
   match output {
      Ok(_) =>  { panic!("Email should be invalid"); }
      Err(e) => { assert_eq!(e, EmailError::EmptyValue); }
   }
}

#[test]
pub fn parse_string_with_whitespace_fails()
{
   let input   = "john doe@foo.com".to_owned();
   let input   = Secret::new(input);
   let output  = Email::parse(input);
   match output {
      Ok(_) =>  { panic!("Email should be invalid"); }
      Err(e) => { assert_eq!(e, EmailError::BadFormat); }
   }
}

#[test]
pub fn email_missing_subject_is_rejected()
{
   let input   = "@doe@com".to_owned();
   let input   = Secret::new(input);
   let output  = Email::parse(input);
   match output {
      Ok(_) =>  { panic!("Email should be invalid"); }
      Err(e) => { assert_eq!(e, EmailError::BadFormat); }
   }
}

#[test]
pub fn parse_string_with_two_at_symbols_fails()
{
   let input   = "john@doe@com".to_owned();
   let input   = Secret::new(input);
   let output  = Email::parse(input);
   match output {
      Ok(_) =>  { panic!("Email should be invalid"); }
      Err(e) => { assert_eq!(e, EmailError::BadFormat); }
   }
}

#[test]
pub fn parse_name_ending_with_period_fails()
{
   let input   = "john.@doe.com.".to_owned();
   let input   = Secret::new(input);
   let output  = Email::parse(input);
   match output {
      Ok(_) =>  { panic!("Email should be invalid"); }
      Err(e) => { assert_eq!(e, EmailError::BadFormat); }
   }
}

#[test]
pub fn parse_string_starting_with_period_fails()
{
   let input   = ".john@doe.com".to_owned();
   let input   = Secret::new(input);
   let output  = Email::parse(input);
   match output {
      Ok(_) =>  { panic!("Email should be invalid"); }
      Err(e) => { assert_eq!(e, EmailError::BadFormat); }
   }
}

#[test]
pub fn parse_where_domain_starts_with_period_fails()
{
   let input   = "john@.doe.com".to_owned();
   let input   = Secret::new(input);
   let output  = Email::parse(input);
   match output {
      Ok(_) =>  { panic!("Email should be invalid"); }
      Err(e) => { assert_eq!(e, EmailError::BadFormat); }
   }
}

#[test]
pub fn parse_where_domain_ends_with_period_fails()
{
   let input   = "john@doe.com.".to_owned();
   let input   = Secret::new(input);
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
   let input = "üñîçøðé@example.com".to_owned();
   let input   = Secret::new(input);
   let output  = Email::parse(input);
   match output {
      Ok(_) =>  { panic!("Email should be invalid"); }
      Err(e) => { assert_eq!(e, EmailError::BadFormat); }
   }
}
