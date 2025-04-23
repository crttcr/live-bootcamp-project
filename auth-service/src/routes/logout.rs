use crate::domain::AuthAPIError;
use crate::utils::{auth::validate_token, constants::JWT_COOKIE_NAME};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;

pub async fn logout(jar: CookieJar) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
   match jar.get(JWT_COOKIE_NAME) {
      None    => { (jar, Err(AuthAPIError::MissingToken)) },
      Some(c) => {
         let token = c.value().to_owned();
         match validate_token(&token).await {
            Err(_) => (jar, Err(AuthAPIError::InvalidToken)),
            Ok(_)  => {
               // let updated_jar = jar.remove(c);
               // To remove a cookie, create a minimal Cookie with just the name and the same path
               //
               let cookie = Cookie::build(JWT_COOKIE_NAME.to_owned())
                  .path("/") // must match original cookie's path
                  .build();
               let updated_jar = jar.remove(cookie);
               println!("Cookie removed from jar. {}.", updated_jar.iter().count());
               (updated_jar, Ok(StatusCode::OK))
            }
         }
      }
   }
}
