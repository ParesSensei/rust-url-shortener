mod core;

use crate::core::{get_url, post_url};
use axum::routing::{get, post};
use axum::Router;
use sqlx::PgPool;

#[derive(Clone)]
struct AppState {
    // data: Arc<Mutex<HashMap<String, String>>>,
    pool: PgPool,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");

    let pool = sqlx::postgres::PgPool::connect(&db_url)
        .await
        .expect("Failed to create postgre database pool");

    let state = AppState {
        pool,
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
