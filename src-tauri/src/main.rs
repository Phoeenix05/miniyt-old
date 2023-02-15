#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod youtube_scraper;
use youtube_scraper::{get_search_data, get_user_data};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_search_data, get_user_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
