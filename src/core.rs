use std::collections::HashMap;
use axum::extract::State;
use axum::Json;
use rand::thread_rng;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use crate::{AppState};

#[test]
pub fn generate_short_code() {
    // let mut pair = HashMap::<String, String>::new();

    let choices1 = [0,1,2,3,4,5,6,7,8,9];
    let choices2 = ['a','b','c','d','e','f'];
    let choices3 = ['A','B','C','D','E','F'];

    let mut rng = thread_rng();
    println!("choices: {:?}", choices1.choose(&mut rng));
    println!("choices: {:?}", choices2.choose(&mut rng));
    println!("choices: {:?}", choices3.choose(&mut rng));
}


fn shortcode() -> String {
    let mut short_code = String::new();
    let choices: Vec<char> = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let mut rng = thread_rng();
    for _ in 0..8{
        // println!("choices = {}", choices.clone().choose(&mut rng).unwrap());
        let code = choices.choose(&mut rng).unwrap();
        short_code.push(*code);
    };
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

pub async fn get_url() -> String {
    "shortcode ...".to_string()
}

pub async fn post_url(
    State(state): State<AppState>,
    Json(payload): Json<UrlPayload>,
) -> Json<UrlPayload> {
    // let paylod = UrlPayload{
    //     url: "https://docs.rs/tokio/latest/tokio/".to_string()
    // };
    let mut data = state.data.lock().await;
    let mut rng = thread_rng();

    Json(payload)
}