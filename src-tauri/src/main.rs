#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod youtube_scraper;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![youtube_scraper::get_youtube_search_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
