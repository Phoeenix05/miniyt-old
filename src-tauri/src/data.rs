use std::{str::FromStr, sync::Mutex};

use lazy_static::lazy_static;
use reqwest::{
    header::{
        HeaderMap, HeaderValue, ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, AUTHORIZATION, USER_AGENT,
    },
    // blocking::Client,
    Client,
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
    }
}

lazy_static! {
    static ref CONFIG_INSTANCE: Mutex<Config> = Mutex::new(Config::new());
}

pub fn query_param(param_name: &str, value: &str) -> String {
    format!("{}={}", param_name, value)
}

// /// It takes a user id, makes a request to the YouTube API, and returns the response as a JSON string
// ///
// /// Arguments:
// ///
// /// * `user_id`: The ID of the user whose playlists you want to retrieve.
// ///
// /// Returns:
// ///
// /// A string of JSON
// #[tauri::command]
// pub async fn get_user_playlists(user_id: String) -> Result<String, Box<dyn std::error::Error>> {
//     let config = CONFIG_INSTANCE.lock().unwrap();

//     let youtube_playlists_url = "https://www.googleapis.com/youtube/v3/playlists";
//     let mut url = reqwest::Url::from_str(youtube_playlists_url).unwrap();
//     url.set_query(Some(&query_param("channelId", &user_id)));
//     url.set_query(Some(&query_param("part", "snippet,player")));

//     let client = Client::new();

//     let res = client.get(url).headers(config.headers.clone()).send().await?;
//     let data = res.text().await?;
//     let json: UserPlaylistsData = serde_json::from_str(&data)?;

//     Ok(serde_json::to_string(&Some(json)).unwrap())
// }
