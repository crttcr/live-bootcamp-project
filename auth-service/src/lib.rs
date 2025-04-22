use crate::domain::error::AuthAPIError;
use crate::routes::*;
use app_state::AppState;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::serve::Serve;
use axum::Json;
use axum::Router;
use serde::Deserialize;
use serde::Serialize;
use tower_http::services::ServeDir;

pub mod app_state;
pub mod domain;
pub mod routes;
pub mod services;

// This struct encapsulates our application-related logic.
// address is exposed as a public field so we have access to it in tests.
//
pub struct Application {
    server:      Serve<Router, Router>,
    pub address: String,
}

// Here we are using ip 0.0.0.0 so the service is listening on all the configured network interfaces.
// This is needed for Docker to work, which we will add later on.
// See: https://stackoverflow.com/questions/39525820/docker-port-forwarding-not-working
//
impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let router = Router::new()
            .nest_service("/",        ServeDir::new("assets"))
            .route("/signup",         post(signup))
            .route("/login",          post(login))
            .route("/logout",         post(logout))
            .route("/verify-2fa",     post(verify_2fa))
            .route("/verify-token",   post(verify_token))
            .with_state(app_state);

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address  = listener.local_addr()?.to_string();
        let server   = axum::serve(listener, router);
        let rv       = Application {server, address};
        Ok(rv)
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError 
{
   fn into_response(self) -> Response {
       let (status, error_message) = match self 
       {
           AuthAPIError::UserAlreadyExists  => (StatusCode::CONFLICT,              "User already exists"),
           AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST,           "Invalid credentials"),
           AuthAPIError::UnexpectedError    => (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error"),
       };
       let error = error_message.to_string();
       let error = ErrorResponse{error};
       let body  = Json(error);
       (status, body).into_response()
   }
}
