use regex::Regex;
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT_ENCODING, ACCEPT_LANGUAGE, USER_AGENT},
    blocking::Client,
};
use tauri::command;

#[command]
pub fn get_youtube_search_data(query: String) -> String {
    let mut headers = HeaderMap::new();
    // User-Agent miniyt-scraper or miniyt/scraper
    headers.insert(USER_AGENT, HeaderValue::from_static("miniyt/scraper"));
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en"));
    headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("utf-8"));

    let client = Client::new();
    let url = format!("https://www.youtube.com/results?search_query={}", query);

    let res = if let Ok(res) = client
        .get(url)
        .headers(headers)
        .send()
    {
        let data = res.text().unwrap();

        let re = Regex::new(r#"var ytInitialData = (.+?);</script>"#).unwrap();
        let captures = re.captures(&data).unwrap();
        let result = &captures[1];
        result.to_string()
    } else {
        "Error occurred".to_string()
    };

    res
}
