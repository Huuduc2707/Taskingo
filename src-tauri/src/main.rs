// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// modules
mod commands;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![commands::greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
