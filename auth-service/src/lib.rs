extern crate core;

use axum::handler::{Handler, HandlerWithoutStateExt};
use crate::routes::*;
use crate::utils::constants::{prod, test};
use app_state::AppState;
use axum::http::Method;
use axum::routing::{post, post_service};
use axum::serve::Serve;
use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

pub mod app_state;
pub mod domain;
pub mod routes;
pub mod services;
pub mod utils;

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
        let allowed_origins = [
            prod::URI_APP.parse()?,
            test::URI_APP.parse()?,
        ];
        
        let cors = CorsLayer::new()
           .allow_methods([Method::GET, Method::POST])
           .allow_credentials(true)                        // Allow cookies to be included in requests
           .allow_origin(allowed_origins);
        
        let router = Router::new()
            .nest_service("/",        ServeDir::new("assets"))
            .route("/signup",         post(signup))
            .route("/login",          post(login))
            .route("/logout",         post_service(logout.with_state(app_state.clone())))
            .route("/verify-2fa",     post(verify_2fa))
            .route("/verify-token",   post_service(verify_token.with_state(app_state.clone())))
            .with_state(app_state)
            .layer(cors);

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
