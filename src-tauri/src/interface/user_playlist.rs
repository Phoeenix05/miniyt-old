/// Generated by https://quicktype.io
use serde::{Deserialize, Serialize};

extern crate serde_json;

#[derive(Serialize, Deserialize)]
pub struct UserPlaylistsData {
    #[serde(rename = "kind")]
    kind: String,

    #[serde(rename = "etag")]
    etag: String,

    #[serde(rename = "nextPageToken")]
    next_page_token: String,

    #[serde(rename = "pageInfo")]
    page_info: PageInfo,

    #[serde(rename = "items")]
    items: Vec<Item>,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "kind")]
    kind: String,

    #[serde(rename = "etag")]
    etag: String,

    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "snippet")]
    snippet: Snippet,

    #[serde(rename = "player")]
    player: Player,
}

#[derive(Serialize, Deserialize)]
pub struct Player {
    #[serde(rename = "embedHtml")]
    embed_html: String,
}

#[derive(Serialize, Deserialize)]
pub struct Snippet {
    #[serde(rename = "publishedAt")]
    published_at: String,

    #[serde(rename = "channelId")]
    channel_id: String,

    #[serde(rename = "title")]
    title: String,

    #[serde(rename = "description")]
    description: String,

    #[serde(rename = "thumbnails")]
    thumbnails: Thumbnails,

    #[serde(rename = "channelTitle")]
    channel_title: String,

    #[serde(rename = "localized")]
    localized: Localized,
}

#[derive(Serialize, Deserialize)]
pub struct Localized {
    #[serde(rename = "title")]
    title: String,

    #[serde(rename = "description")]
    description: String,
}

#[derive(Serialize, Deserialize)]
pub struct Thumbnails {
    #[serde(rename = "default")]
    thumbnails_default: Default,

    #[serde(rename = "medium")]
    medium: Default,

    #[serde(rename = "high")]
    high: Default,

    #[serde(rename = "standard")]
    standard: Default,

    #[serde(rename = "maxres")]
    maxres: Default,
}

#[derive(Serialize, Deserialize)]
pub struct Default {
    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "width")]
    width: i64,

    #[serde(rename = "height")]
    height: i64,
}

#[derive(Serialize, Deserialize)]
pub struct PageInfo {
    #[serde(rename = "totalResults")]
    total_results: i64,

    #[serde(rename = "resultsPerPage")]
    results_per_page: i64,
}