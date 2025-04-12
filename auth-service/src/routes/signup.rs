
use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SignupRequest {
    pub email:          String,
    pub password:       String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa:   bool,
}

pub async fn signup(Json(request): Json<SignupRequest>) -> impl IntoResponse {
    println!("Received signup request: {:?}", request);
    // Here you would typically handle the signup logic, such as saving the user to a database.
    // For this example, we will just return a 200 OK response.
    // Simulate some processing
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
   StatusCode::OK.into_response()
}