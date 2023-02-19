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

// #[tauri::command]
// async fn hallo() -> String {
//     "Hallo".to_string()
// }

// use tauri::{
//     api::{file::read_string, path},
//     Config,
// };

// #[tauri::command]
// async fn read_channels(profile: String) -> Option<Vec<String>> {
//     // let mut config = Config {};
//     let data = read_string("file");
//     // path::app_data_dir("config");
//     todo!()
// }

// // #[tauri::command]
// // async fn write_channels() -> bool {
// //     todo!()
// // }

// #[tauri::command]
// async fn update_channels() -> bool {
//     todo!()
// }

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
        // .invoke_handler(tauri::generate_handler![hallo])
        // .invoke_handler(tauri::generate_handler![read_channels, write_channels, update_channels])
        // .invoke_handler(tauri::generate_handler![read_channels, update_channels])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
