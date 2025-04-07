use axum::serve::Serve;
use axum::Router;
use tower_http::services::ServeDir;
use crate::routes::*;
use axum::routing::post;

pub mod routes;

// This struct encapsulates our application-related logic.
//
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
    pub async fn build(address: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let router = Router::new()
            .nest_service("/",        ServeDir::new("assets"))
            .route("/signup",         post(signup))
            .route("/login",          post(login))
            .route("/logout",         post(logout))
            .route("/verify-2fa",     post(verify_2fa))
            .route("/verify-token",   post(verify_token));

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);
        let rv = Application {server, address};
        Ok(rv)
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}
