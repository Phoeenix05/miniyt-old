use std::str::FromStr;

use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, Url};
use serde_json::Value;

fn headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.append("Accept-Language", HeaderValue::from_static("en-US"));
    headers
}

async fn req(url: Url) -> Result<reqwest::Response, reqwest::Error> {
    Client::new().get(url).headers(headers()).send().await
}

fn parse_data(s: String) -> Value {
    let regex = Regex::new(r#"var ytInitialData = (.+?);</script>"#).unwrap();
    let captures = regex.captures(s.as_str()).unwrap();
    let res = &captures[1];
    Value::from_str(res).unwrap()
}

pub async fn search_query(q: String) -> Result<Value, String> {
    let url = format!("https://youtube.com/results?search_query={}&limit=100", q);
    let res = req(Url::from_str(&url).unwrap())
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let data = parse_data(res);
    Ok(data)
}
