use axum::{extract::Path, routing::get, Json, Router};
use serde::Serialize;
use serde::Deserialize;

// Data structure representing a user
#[derive(Serialize)]
struct User {
    username: String,
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
    bio: String,
}

// Handler for GET /user/:username
async fn get_user(Path(username): Path<String>) -> Json<User> {
    // Return fake user data
    let user = User {
        username: username.clone(),
        name: "John Doe".to_string(),
        email: format!("{}@example.com", username),
    };
    Json(user)
}

// Handler for POST /user/create
async fn create_user(Json(payload): Json<CreateUser>) -> Json<String> {
    // Simulate user creation
    let msg = format!("User '{}' created successfully with bio: {}", payload.username, payload.bio);
    println!("{}", msg);
    Json(msg)
}

// Router for the user endpoints
pub fn router() -> Router {
    Router::new()
        .route("/:username", get(get_user))
        .route("/create", axum::routing::post(create_user))
} 