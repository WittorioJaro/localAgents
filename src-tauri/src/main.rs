// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ollama;

use ollama::{check_ollama_status, download_ollama_model, list_ollama_models, start_ollama_server};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            check_ollama_status,
            download_ollama_model,
            list_ollama_models,
            start_ollama_server,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
