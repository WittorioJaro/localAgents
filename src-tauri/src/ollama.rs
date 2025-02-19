use tauri::{AppHandle, Manager};
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt;

#[tauri::command]
pub async fn start_ollama_server(app: AppHandle) -> Result<(), String> {
    let (mut rx, child) = app
        .shell()
        .sidecar("ollama")
        .expect("failed to create sidecar command")
        .args(["serve"])
        .spawn()
        .map_err(|e| e.to_string())?;

    // Store the child process handle in app state for later use
    app.manage(std::sync::Mutex::new(child));

    // Monitor the process output
    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(line) => {
                    println!("Ollama stdout: {}", String::from_utf8_lossy(&line));
                }
                CommandEvent::Stderr(line) => {
                    eprintln!("Ollama stderr: {}", String::from_utf8_lossy(&line));
                }
                _ => {}
            }
        }
    });

    Ok(())
}

#[tauri::command]
pub async fn check_ollama_status(app: AppHandle) -> Result<bool, String> {
    let output = app
        .shell()
        .sidecar("ollama")
        .expect("failed to create sidecar command")
        .args(["--version"])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    Ok(output.status.success())
}

#[tauri::command]
pub async fn download_ollama_model(app: AppHandle, model_name: String) -> Result<(), String> {
    let output = app
        .shell()
        .sidecar("ollama")
        .expect("failed to create sidecar command")
        .args(["pull", &model_name])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[tauri::command]
pub async fn list_ollama_models(app: AppHandle) -> Result<Vec<String>, String> {
    let output = app
        .shell()
        .sidecar("ollama")
        .expect("failed to create sidecar command")
        .args(["list"])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        // Skip header line and parse model names
        let models: Vec<String> = output_str
            .lines()
            .skip(1)
            .filter_map(|line| line.split_whitespace().next())
            .map(String::from)
            .collect();
        Ok(models)
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
