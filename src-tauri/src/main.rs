// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::create_app_data_dir,
            commands::app_data_dir_exists
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
