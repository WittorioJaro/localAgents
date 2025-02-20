// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ollama;

use ollama::{
    check_ollama_status, delete_ollama_model, download_ollama_model, ensure_python_service,
    execute_crewai_task, list_ollama_models, start_ollama_server,
};
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(move |app| {
            // Start both services when the app launches
            let app_handle = app.handle().clone();

            tauri::async_runtime::spawn(async move {
                println!("Starting services...");

                // Start Ollama server
                if let Err(e) = start_ollama_server(app_handle.clone()).await {
                    eprintln!("Failed to start Ollama server: {}", e);
                }

                // Start Python service
                if let Err(e) = ensure_python_service(app_handle.clone()).await {
                    eprintln!("Failed to start Python service: {}", e);
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            check_ollama_status,
            download_ollama_model,
            list_ollama_models,
            start_ollama_server,
            delete_ollama_model,
            execute_crewai_task,
            ensure_python_service,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
