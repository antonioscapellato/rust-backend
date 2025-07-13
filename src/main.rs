use axum::{
    routing::get,
    Json, Router,
};
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct HelloResponse {
    msg: String,
}

async fn hello_handler() -> Json<HelloResponse> {
    let response = HelloResponse {
        msg: "hello, world!".to_string(),
    };
    Json(response)
}

#[tokio::main]
async fn main() {
    // Set up router
    let app = Router::new()
        .route("/hello", get(hello_handler));

    // Bind address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    // Start server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
