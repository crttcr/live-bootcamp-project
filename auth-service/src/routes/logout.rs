use axum::extract::State;
use crate::domain::AuthAPIError;
use crate::utils::{auth, constants::JWT_COOKIE_NAME};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use crate::app_state::AppState;

pub async fn logout(
   State(state):   State<AppState>,
   jar: CookieJar
   ) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
   match jar.get(JWT_COOKIE_NAME) {
      None    => { (jar, Err(AuthAPIError::MissingToken)) },
      Some(c) => {
         let token = c.value().to_owned();
         println!("Token: {}", token);
         // let banned_tokens = state.banned_tokens.read().await;
//         match auth::validate_token(&*banned_tokens, &token).await {
         match auth::validate_token(&token).await {
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
               let mut token_store = state.banned_tokens.write().await;
               match token_store.add_token(token).await {
                  Err(_) => { println!("Error adding token to banned tokens store."); },
                  Ok(_)  => { println!("Token added to banned tokens store."); }
               };
               let count      = token_store.count().await.unwrap();
               println!("Count: {}", count);
               (updated_jar, Ok(StatusCode::OK))
            }
         }
      }
   }
}
