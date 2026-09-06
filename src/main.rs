mod core;

use crate::core::{get_url, post_url};
use axum::routing::{get, post};
use axum::Router;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
struct AppState {
    data: Arc<Mutex<HashMap<String, String>>>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        data: Arc::new(Mutex::new(HashMap::new())),
    };
    let app = Router::new()
        .route("/", get(|| async { "URL Shortener is running!" }))
        .route("/post_url", post(post_url))
        .route("/get_url/{short_code}", get(get_url))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
