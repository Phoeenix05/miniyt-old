use regex::Regex;
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, ACCEPT_ENCODING, ACCEPT_LANGUAGE, USER_AGENT},
};
use tauri::command;

// pub enum YtUrls {
//     Search
// }

pub fn fetch_url(path: String) -> String {
    let mut headers = HeaderMap::new();
    // User-Agent miniyt-scraper or miniyt/scraper
    headers.insert(USER_AGENT, HeaderValue::from_static("miniyt/scraper"));
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en"));
    headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("utf-8"));

    let client = Client::new();
    let url = format!("https://www.youtube.com/{}", path);

    // let res: String = if let Ok(res) = client.get(&url).headers(headers).send() {
    //     let data = res.text().unwrap();
    //     let scraped_data = scraper(&data);
    //     scraped_data
    // } else {
    //     "Error occurred".to_string()
    // };

    // res

    let res = match client.get(&url).headers(headers).send() {
        Ok(res) => {
            let data = res.text().unwrap();
            let scraped_data = scraper(data);
            scraped_data
            // todo!()
        }
        Err(err) => err.to_string(),
    };

    res
    // todo!()
}

// pub fn scraper(data: &String) -> String
pub fn scraper(data: String) -> String {
    let regular_expression = Regex::new(r#"var ytInitialData = (.+?);</script>"#).unwrap();
    let captures = regular_expression.captures(&data).unwrap();
    let result = &captures[1];
    result.to_string()
}

#[command]
pub fn get_search_data(query: String) -> String {
    // let data = fetch_url("path".to_string());
    let path = format!("/results?search_query={}", query);
    let data = fetch_url(path);
    data
}

#[command]
pub fn get_user_data(name: String) -> String {
    let path = format!("{}", name);
    let data = fetch_url(path);
    data
}
