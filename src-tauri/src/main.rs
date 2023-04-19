#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// use miniyt_scraper::*;

#[tauri::command]
async fn search_query(q: String) -> Result<String, String> {
    // let query = miniyt_scraper::Query {};
    miniyt_scraper::search_query(q).await
}

fn main() {
    tauri::Builder::default()
        // .setup(|_| Ok(())) // setup not used currently
        .invoke_handler(tauri::generate_handler![search_query])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
