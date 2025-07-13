use axum::{extract::Path, routing::get, Json, Router};
use serde::Serialize;

// Data structure representing a video
#[derive(Serialize)]
struct Video {
    uid: String,
    title: String,
    duration: u32,
}

// Handler for GET /video/:uid
async fn get_video(Path(uid): Path<String>) -> Json<Video> {
    // Return fake video data
    let video = Video {
        uid: uid.clone(),
        title: "Sample Video".to_string(),
        duration: 3600,
    };
    Json(video)
}

// Router for the video endpoints
pub fn router() -> Router {
    Router::new().route("/:uid", get(get_video))
} 