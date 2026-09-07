use crate::AppState;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Redirect;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use url::Url;

#[test]
pub fn generate_short_code() {
    // let mut pair = HashMap::<String, String>::new();

    let choices1 = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let choices2 = ['a', 'b', 'c', 'd', 'e', 'f'];
    let choices3 = ['A', 'B', 'C', 'D', 'E', 'F'];

    let mut rng = thread_rng();
    println!("choices: {:?}", choices1.choose(&mut rng));
    println!("choices: {:?}", choices2.choose(&mut rng));
    println!("choices: {:?}", choices3.choose(&mut rng));
}

fn shortcode() -> String {
    let mut short_code = String::new();
    let choices: Vec<char> = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();
    let mut rng = thread_rng();
    for _ in 0..8 {
        // println!("choices = {}", choices.clone().choose(&mut rng).unwrap());
        let code = choices.choose(&mut rng).unwrap();
        short_code.push(*code);
    }
    println!("short_code = {}", &short_code);

    // let user_data = "https://docs.rs/tokio/latest/tokio/".to_string();
    // let mut data = HashMap::new();
    // data.insert(short_code, user_data);
    // println!("data = {:?}", &data);
    short_code
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UrlPayload {
    url: String,
}

#[derive(Serialize)]
pub struct ShortCodeResponse {
    short_code: String,
}

pub async fn get_url(
    Path(short_code): Path<String>,
    State(state): State<AppState>,
) -> Result<Redirect, StatusCode> {
    // let data = state.data.lock().await;

    let result = sqlx::query!("SELECT * FROM urls WHERE short_code = $1", short_code)
        .fetch_one(&state.pool)
        .await;

    let url = match result {
        Ok(row) => Ok(Redirect::permanent(&row.original_url)),
        Err(sqlx::Error::RowNotFound) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    url
}

pub async fn post_url(
    State(state): State<AppState>,
    Json(payload): Json<UrlPayload>,
) -> Result<Json<ShortCodeResponse>, StatusCode> {
    let input_url = payload.url.clone();
    let result = Url::parse(&input_url);
    match result {
        Ok(url) => {
            if url.scheme() == "https" || url.scheme() == "http" {
                // valid
                // let mut data = state.data.lock().await;
                let new_url = shortcode();

                let data = sqlx::query!("INSERT INTO urls (short_code, original_url)
                    VALUES ($1, $2)"
                    ,new_url, payload.url)
                    .execute(&state.pool)
                    .await;

                println!("{:?}", data);
                match data {
                    Ok(_) => {},
                    Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
                }
                Ok(Json(ShortCodeResponse {
                    short_code: new_url,
                }))
            } else {
                Err(StatusCode::BAD_REQUEST)
            }
        }
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

//     url: "https://docs.rs/tokio/latest/tokio/"

// testing shortcode

#[test]
fn test_shortcode() {
    let shortcode = shortcode();
    assert_eq!(shortcode.len(), 8 );
}

#[test]
fn test_shortcode_characters() {
    let shortcode = shortcode();
    let choices = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    for c in shortcode.chars() {
        assert!(choices.contains(c));
    }
}