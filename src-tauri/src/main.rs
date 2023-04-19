#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]



#[tauri::command]
fn search_query(q: String) -> String {
    
    
    "".to_string()
}

fn main() {
    tauri::Builder::default()
        // .setup(|_| Ok(())) // setup not used currently
        .invoke_handler(tauri::generate_handler![search_query])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
