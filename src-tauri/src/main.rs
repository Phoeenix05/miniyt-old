#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// #[macro_use]
// extern crate rocket;

#[rocket::get("/playlists")]
async fn playlists() -> &'static str {
    "playlists"
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            tauri::async_runtime::spawn(
                rocket::build().mount("/api", rocket::routes![playlists]).launch()
            );
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
