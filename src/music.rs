use axum::{extract::Path, routing::get, Json, Router};
use serde::Serialize;

// Data structure representing a music track
#[derive(Serialize)]
struct Music {
    uid: String,
    title: String,
    artist: String,
}

// Handler for GET /music/:uid
async fn get_music(Path(uid): Path<String>) -> Json<Music> {
    // Return fake music data
    let music = Music {
        uid: uid.clone(),
        title: "Sample Song".to_string(),
        artist: "Sample Artist".to_string(),
    };
    Json(music)
}

// Router for the music endpoints
pub fn router() -> Router {
    Router::new().route("/:uid", get(get_music))
} 