use regex::Regex;
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, ACCEPT_ENCODING, ACCEPT_LANGUAGE, USER_AGENT},
};
use tauri::command;

/// `fetch_url` takes a `String` and returns a `String`
///
/// Arguments:
///
/// * `path`: The path to the video, e.g. watch?v=dQw4w9WgXcQ
///
/// Returns:
///
/// A String
pub fn fetch_url(path: String) -> String {
    let mut headers = HeaderMap::new();
    // User-Agent miniyt-scraper or miniyt/scraper
    headers.insert(USER_AGENT, HeaderValue::from_static("miniyt/scraper"));
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en"));
    headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("utf-8"));

    let client = Client::new();
    let url = format!("https://www.youtube.com/{}", path);

    let res = match client.get(&url).headers(headers).send() {
        Ok(res) => {
            let data = res.text().unwrap();
            let scraped_data = scraper(data);
            scraped_data
        }
        Err(err) => err.to_string(),
    };

    res
}

/// It takes a string, and returns a string
///
/// Arguments:
///
/// * `data`: &String - This is the data that we're going to be scraping.
///
/// Returns:
///
/// A string
pub fn scraper(data: String) -> String {
    let regular_expression = Regex::new(r#"var ytInitialData = (.+?);</script>"#).unwrap();
    let captures = regular_expression.captures(&data).unwrap();
    let result = &captures[1];
    result.to_string()
}

/// It takes a string, formats it into a path, and then fetches the data from that path
///
/// Arguments:
///
/// * `query`: String - This is the query that the user will type in.
///
/// Returns:
///
/// A string
#[command]
pub fn get_search_data(query: String) -> String {
    let path = format!("/results?search_query={}", query);
    let data = fetch_url(path);
    data
}

/// `get_user_data` takes a string, and returns a string
///
/// Arguments:
///
/// * `name`: The name of the user to get data for.
///
/// Returns:
///
/// A String
#[command]
pub fn get_user_data(name: String) -> String {
    let path = format!("/{}", name);
    let data = fetch_url(path);
    data
}
