use crate::utils::auth::validate_token;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct VerifyRequest {
   pub token:          String,
}


pub async fn verify_token(
   Json(request):  Json<VerifyRequest>,
) -> impl IntoResponse { 
   println!("Verify Request: {:?}", request);

   let token     = request.token;
   let validated = validate_token(&token).await;
   match validated {
      Ok(_) => {
         println!("Token is valid");
         StatusCode::OK.into_response()
      },
      Err(e) => {
         println!("Token is invalid: {:?}", e);
         StatusCode::UNAUTHORIZED.into_response()
      }
   }
}
