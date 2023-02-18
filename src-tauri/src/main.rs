#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// #[macro_use]
// extern crate rocket;

// use rocket::serde::json::Json;

// #[rocket::get("/playlists")]
// async fn playlists() -> Json<String> {
//     // println!("playlists");
//     Json("playlists".to_string())
// }

#[tauri::command]
async fn hallo() -> String {
    "Hallo".to_string()
}

fn main() {
    tauri::Builder::default()
        // .setup(|app| {
        //     tauri::async_runtime::spawn(
        //         rocket::build()
        //             .mount("/api", rocket::routes![playlists])
        //             .launch(),
        //     );
        //     Ok(())
        // })
        .invoke_handler(tauri::generate_handler![hallo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
