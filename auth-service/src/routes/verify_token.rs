use crate::utils::auth::validate_token;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};
use crate::app_state::AppState;

#[derive(Deserialize, Debug, Serialize)]
pub struct VerifyTokenRequest {
   pub token:          String,
}

#[tracing::instrument(name = "verify token handler", skip_all)]
pub async fn verify_token(
   State(state):   State<AppState>,
   Json(request):  Json<VerifyTokenRequest>,
) -> impl IntoResponse { 
   let token         = request.token;
   let banned_tokens = state.banned_tokens.clone();
   let validated     = validate_token(&token, banned_tokens).await;
   match validated {
      Ok(_) => {
         debug!("Token is valid");
         StatusCode::OK.into_response()
      },
      Err(e) => {
         warn!("Token is invalid: {:?}", e);
         StatusCode::UNAUTHORIZED.into_response()
      }
   }
}
