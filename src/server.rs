use axum::{routing::get, Json, Router};
use serde::Serialize;
use chrono::prelude::*;
use os_info;

// Data structure for server info response
#[derive(Serialize)]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub time: String,
    pub os_type: String,
    pub os_version: String,
}

// Handler for GET /server/info
async fn server_info_handler() -> Json<ServerInfo> {
    let info = ServerInfo {
        name: "Rust Axum Modular API".to_string(),
        version: "1.0.0".to_string(),
        description: "A modular Axum backend with user, video, music, and server info endpoints.".to_string(),
        time: Utc::now().to_rfc3339(),
        os_type: os_info::get().os_type().to_string(),
        os_version: os_info::get().version().to_string(),
    };
    Json(info)
}

// Router for the server endpoints
pub fn router() -> Router {
    Router::new().route("/info", get(server_info_handler))
}
