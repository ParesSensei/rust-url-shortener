mod core;

use crate::core::{get_url, post_url};
use axum::routing::{get, post};
use axum::Router;
use sqlx::PgPool;
use tower_http::services::ServeDir;

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");

    let pool = sqlx::postgres::PgPool::connect(&db_url)
        .await
        .expect("Failed to create postgre database pool");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    let state = AppState {
        pool,
    };

    let static_file = ServeDir::new("static");

    let app = Router::new()
        .route("/post_url", post(post_url))
        .route("/url/{short_code}", get(get_url))
        .fallback_service(static_file)
        .with_state(state);

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string());

    let addr = format!("0.0.0.0:{port}");

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();

    println!("Listening on: {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
