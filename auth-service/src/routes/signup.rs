use serde::{Deserialize, Serialize};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;

use crate::domain::user::User;
use crate::app_state::AppState;

#[derive(Deserialize, Debug)]
pub struct SignupRequest {
    pub email:          String,
    pub password:       String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa:   bool,
}

impl SignupRequest {
    pub fn to_user(self) -> User {
        User::new(self.email, self.password, self.requires_2fa)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SignupResponse {
    pub message: String,
}
pub async fn signup(
    State(state):   State<AppState>,
    Json(request):  Json<SignupRequest>,
    ) -> impl IntoResponse {
    println!("Received signup request: {:?}", request);
    let mut store = state.user_store.write().await;
    let user      = request.to_user();
    let result    = store.add_user(user);
    let result    = result.unwrap();
    println!("User added: {:?}", result);
    /*
    match result {
        Ok(_) => println!("User added successfully"),
        Err(e) => {
            println!("Failed to add user: {:?}", e);
            return (StatusCode::BAD_REQUEST, Json(SignupResponse{message: "Failed to add user".to_owned()}));
        }
    }
    */

    // Here you would typically handle the signup logic, such as saving the user to a database.
    // For this example, we will just return a 200 OK response.
    // Simulate some processing
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    let message  = "User created successfully!".to_owned();
    let response = Json(SignupResponse{message});
    (StatusCode::CREATED, response)
}