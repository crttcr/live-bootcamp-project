extern crate core;

use crate::routes::*;
use crate::utils::constants::{prod, test};
use app_state::AppState;
use axum::http::Method;
use axum::routing::post;
use axum::serve::Serve;
use axum::Router;
use redis;
use redis::RedisResult;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::info;
use crate::utils::tracing::{make_span_with_request_id, on_request, on_response};

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

        let trace = TraceLayer::new_for_http()
           .make_span_with(make_span_with_request_id)
           .on_request(on_request)
           .on_response(on_response);
        
        let router = Router::new()
            .nest_service("/",        ServeDir::new("assets"))
            .route("/signup",         post(signup))
            .route("/login",          post(login))
            .route("/logout",         post(logout))
            .route("/verify-2fa",     post(verify_2fa))
            .route("/verify-token",   post(verify_token))
            .with_state(app_state)
            .layer(cors)
            .layer(trace);

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address  = listener.local_addr()?.to_string();
        let server   = axum::serve(listener, router);
        let rv       = Application {server, address};
        Ok(rv)
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        info!("listening on {}", &self.address);
        self.server.await
    }
}

pub async fn create_postgres_pool(url: &str) -> Result<PgPool, sqlx::Error> {
    let result = PgPoolOptions::new().max_connections(3).connect(url).await;
    info!("PostgreSQL pool created: {:?}", result);
    result
}

pub fn create_redis_client(redis_hostname: String) -> RedisResult<redis::Client> {
    let redis_url = format!("redis://{}/", redis_hostname);
    let client    = redis::Client::open(redis_url);
    info!("Redis client created: {:?}", client);
    client
}
