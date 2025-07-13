// Import the user, video, music, and server modules
mod user;
mod video;
mod music;
mod server;
use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;

// Entry point of the application
#[tokio::main]
async fn main() {
    // Set up the main router and nest the user, video, music, and server routers
    let app = Router::new()
        .nest("/server", server::router())
        .nest("/user", user::router())
        .nest("/video", video::router())
        .nest("/music", music::router());

    // Bind the server to localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    // Start the Axum server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
