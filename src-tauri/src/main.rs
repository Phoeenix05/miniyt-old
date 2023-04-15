#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command]
fn init_data_files() -> bool {
    true
}

#[tauri::command]
fn data_files_exist() {
    todo!()
}

fn main() {
    tauri::Builder::default()
        .setup(|_| Ok(())) // setup not used currently
        .invoke_handler(tauri::generate_handler![init_data_files, data_files_exist])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
