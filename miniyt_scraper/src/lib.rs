// use serde::Serialize;

// #[derive(Serialize)]
// pub struct Query;

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};

// #[tauri::command]
pub async fn search_query(q: String) -> Result<String, String> {
    let mut headers = HeaderMap::new();
    headers.append("Accept-Language", HeaderValue::from_static("en-US"));

    let client = Client::new();
    let url = format!("https://youtube.com/results?search_query={}", q);
    let res = client
        .get(url)
        .headers(headers)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    Ok(res)
}

// fn main() {
//     println!("Hello, world!");
// }
