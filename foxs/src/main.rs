use axum::{routing::post, Json, Router};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio;

#[derive(Deserialize)]
struct SaveRequest {
    url: String,
    content: String,
}

async fn save(Json(payload): Json<SaveRequest>) {
    println!("Received data: URL = {}\nContent = {}", payload.url, payload.content);
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/save", post(save));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
