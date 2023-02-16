use std::{str::FromStr, sync::Mutex};

use lazy_static::lazy_static;
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, ACCEPT_ENCODING, ACCEPT_LANGUAGE, USER_AGENT, ACCEPT, AUTHORIZATION},
};

use crate::interface::user_playlist::UserPlaylistsData;

struct Config {
    headers: HeaderMap,
}

impl Config {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        // User-Agent miniyt-scraper or miniyt/scraper
        headers.insert(USER_AGENT, HeaderValue::from_static("miniyt/scraper"));
        headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en"));
        headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("utf-8"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        // headers.insert(AUTHORIZATION, HeaderValue::from_str(&std::env::var("YT_DATA_API_KEY").unwrap_or("".to_string())));

        Self { headers }
        // todo!()
    }
}

lazy_static! {
    static ref CONFIG_INSTANCE: Mutex<Config> = Mutex::new(Config::new());
}

pub fn query_param(param_name: &str, value: &str) -> String {
    format!("{}={}", param_name, value)
    // todo!()
}

/// It takes a user id, makes a request to the YouTube API, and returns the response as a JSON string
///
/// Arguments:
///
/// * `user_id`: The ID of the user whose playlists you want to retrieve.
///
/// Returns:
///
/// A string of JSON
#[tauri::command]
// pub async fn get_user_playlists(user_id: String) -> String {
pub async fn get_user_playlists(id: String) -> String {
    let config = CONFIG_INSTANCE.lock().unwrap();
    // config.headers

    // let url = format!("https://www.googleapis.com/youtube/v3/playlists");
    let mut url =
        reqwest::Url::from_str("https://www.googleapis.com/youtube/v3/playlists").unwrap();
    // url.set_query(Some(format!("channelId={}", name).as_str()));
    url.set_query(Some(&query_param("channelId", &id)));
    url.set_query(Some(&query_param("part", "snippet,player")));

    let client = Client::new();

    let res: Option<UserPlaylistsData> =
        match client.get(url).headers(config.headers.clone()).send() {
            Ok(res) => {
                let data = res.text().unwrap();
                let json: UserPlaylistsData = serde_json::from_str(&data).unwrap();
                // todo!()
                Some(json)
            }
            // Err(err) => todo!(),
            Err(err) => {
                eprintln!("{}", err);
                None
            }
        };

    serde_json::to_string(&res).unwrap()
    // res.unwrap()
    // todo!()
}
