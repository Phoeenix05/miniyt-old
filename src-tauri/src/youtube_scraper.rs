// use std::ffi::CString;
// use std::os::raw::c_char;

use regex::Regex;
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT_ENCODING, ACCEPT_LANGUAGE, USER_AGENT},
    // blocking::Client,
    Client,
};
use tauri::command;

// #[no_mangle]
// pub async extern "C" fn get_youtube_search_data() -> *mut c_char {
#[command]
pub async fn get_youtube_search_data(query: String) -> Result<String, reqwest::Error> {
    let mut headers = HeaderMap::new();
    // User-Agent miniyt-scraper or miniyt/scraper
    headers.insert(USER_AGENT, HeaderValue::from_static("miniyt/scraper"));
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en"));
    headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("utf-8"));

    let client = Client::new();

    // let results = client
    //     .get("https://www.youtube.com/results?search_query=brian+x+stephanie+swoard+art+online")
    //     .headers(headers)
    //     .send()?;

    // let data = results.text().unwrap();

    // let re = Regex::new(r#"var ytInitialData = (.+?);</script>"#).unwrap();
    // let captures = re.captures(&data).unwrap();
    // let result = &captures[1];
    // result.to_string()

    // let encoded_query = "";
    let url = format!("https://www.youtube.com/results?search_query={}", query);

    let res = if let Ok(res) = client
        // .get("https://www.youtube.com/results?search_query=brian+x+stephanie+swoard+art+online")
        .get(url)
        .headers(headers)
        // .send()
        .send()
        .await
    {
        // let data = res.text().unwrap();
        let data = res.text().await.unwrap();

        let re = Regex::new(r#"var ytInitialData = (.+?);</script>"#).unwrap();
        let captures = re.captures(&data).unwrap();
        let result = &captures[1];
        result.to_string()
    } else {
        "Error occurred".to_string()
    };

    // if cfg_if!(target_arch = "wasm32") {
    // if #[cfg(target_arch = "wasm32")] {
    //     let c_res = CString::new(res).unwrap();
    //     c_res.into_raw()
    // } else {
    // }

    Ok(res)
}
