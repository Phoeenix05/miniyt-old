#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// mod youtube_scraper;
// use youtube_scraper::{get_search_data, get_user_data};

mod data;
mod interface;

fn main() {
    tauri::Builder::default()
        // .invoke_handler(tauri::generate_handler![get_search_data, get_user_data])
        .invoke_handler(tauri::generate_handler![data::get_user_playlists])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
