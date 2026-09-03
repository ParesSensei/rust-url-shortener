mod core;

use axum::{Json, Router};
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};
#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get(|| async { "URL Shortener is running!" }))
        .route("/post_url", post(post_url))
        .route("/get_url", get(get_url));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize, Serialize, Debug)]
struct UrlPayload {
    url: String,
}

// #[axum::debug_handler]
async fn post_url(
    Json(payload): Json<UrlPayload>,
) -> Json<UrlPayload> {
    // let paylod = UrlPayload{
    //     url: "https://docs.rs/tokio/latest/tokio/".to_string()
    // };
    Json(payload)
}

async fn get_url() -> String {
    "shortccode ...".to_string()
}