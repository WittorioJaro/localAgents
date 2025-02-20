use reqwest;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt;
use tokio::time::sleep;

#[derive(serde::Serialize, Debug, Clone)]
struct DownloadProgress {
    percent: f32,
    downloaded: String,
    total: String,
    speed: Option<String>,
    eta: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CrewAIConfig {
    model_name: String,
    task: String,
    role: String,
    goal: String,
    backstory: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
struct CrewAIResponse {
    result: String,
}

#[derive(serde::Deserialize, Debug)]
struct CrewAIError {
    detail: String,
    traceback: Option<String>,
}

#[tauri::command]
pub async fn start_ollama_server(app: AppHandle) -> Result<(), String> {
    // First check if Ollama is already running
    if check_ollama_status(app.clone()).await.unwrap_or(false) {
        return Ok(());
    }

    println!("Attempting to start Ollama server...");

    // Try to start Ollama using the system command
    let (mut rx, child) = app
        .shell()
        .command("ollama")
        .args(["serve"])
        .spawn()
        .map_err(|e| format!("Failed to start Ollama: {}", e))?;

    // Store the child process handle in app state for later use
    app.manage(std::sync::Mutex::new(child));

    // Monitor the process output in a separate task
    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(line) => {
                    println!("Ollama stdout: {}", String::from_utf8_lossy(&line));
                }
                CommandEvent::Stderr(line) => {
                    eprintln!("Ollama stderr: {}", String::from_utf8_lossy(&line));
                }
                CommandEvent::Error(err) => {
                    eprintln!("Ollama process error: {}", err);
                }
                CommandEvent::Terminated(status) => {
                    eprintln!("Ollama process terminated with status: {:?}", status);
                }
                _ => {}
            }
        }
    });

    // Wait for the server to be ready by polling the status
    println!("Waiting for Ollama server to be ready...");
    let mut attempts = 0;
    while attempts < 10 {
        sleep(Duration::from_secs(1)).await;
        match check_ollama_status(app.clone()).await {
            Ok(true) => {
                println!("Ollama server is ready!");
                return Ok(());
            }
            Ok(false) => {
                println!("Attempt {}: Ollama not ready yet", attempts + 1);
                attempts += 1;
            }
            Err(e) => {
                println!("Attempt {}: Error checking status: {}", attempts + 1, e);
                attempts += 1;
            }
        }
    }

    Err("Failed to start Ollama server after multiple attempts".to_string())
}

#[tauri::command]
pub async fn check_ollama_status(app: AppHandle) -> Result<bool, String> {
    let output = app
        .shell()
        .command("ollama")
        .args(["--version"])
        .output()
        .await
        .map_err(|e| format!("Failed to check Ollama status: {}", e))?;

    Ok(output.status.success())
}

#[tauri::command]
pub async fn download_ollama_model(app: AppHandle, model_name: String) -> Result<(), String> {
    println!("Starting download of model: {}", model_name);

    let (mut rx, child) = app
        .shell()
        .command("ollama")
        .args(["pull", &model_name])
        .spawn()
        .map_err(|e| format!("Failed to start model download: {}", e))?;

    // Store the child process handle in app state
    app.manage(std::sync::Mutex::new(child));

    // Monitor the download progress
    while let Some(event) = rx.recv().await {
        match event {
            CommandEvent::Stdout(line) => {
                let line = String::from_utf8_lossy(&line);
                println!("Download stdout: {}", line);
            }
            CommandEvent::Stderr(line) => {
                let line = String::from_utf8_lossy(&line);
                // Filter out ANSI escape codes and progress indicators
                let clean_line = line.replace(
                    |c: char| !c.is_ascii_graphic() && !c.is_ascii_whitespace(),
                    "",
                );

                if clean_line.contains("writing manifest") {
                    // This is not an error, just a progress step
                    println!("Writing manifest...");
                    continue;
                }

                if clean_line.contains("pulling") {
                    // Try to parse progress information
                    if let Some(progress) = parse_progress(&clean_line) {
                        println!("Progress: {:?}", progress);
                        // Emit progress event to frontend
                        app.emit("download-progress", progress)
                            .unwrap_or_else(|e| eprintln!("Failed to emit progress: {}", e));
                    }
                } else if !clean_line.is_empty()
                    && !clean_line.contains("verifying")
                    && !clean_line.contains("success")
                {
                    eprintln!("Download stderr: {}", clean_line);
                    return Err(clean_line);
                }
            }
            CommandEvent::Error(err) => {
                eprintln!("Download error: {}", err);
                return Err(err.to_string());
            }
            CommandEvent::Terminated(status) => {
                println!("Download process terminated with status: {:?}", status);
                match status.code {
                    Some(0) => {
                        // Immediately list models after successful download
                        if let Ok(models) = list_ollama_models(app.clone()).await {
                            app.emit("models-updated", models).unwrap_or_else(|e| {
                                eprintln!("Failed to emit models update: {}", e)
                            });
                        }
                        return Ok(());
                    }
                    _ => return Err(format!("Download failed with status: {:?}", status)),
                }
            }
            _ => {}
        }
    }

    Ok(())
}

fn parse_progress(line: &str) -> Option<DownloadProgress> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    for (i, part) in parts.iter().enumerate() {
        if part.ends_with('%') {
            let percent = part.trim_end_matches('%').parse::<f32>().ok()?;
            let downloaded = parts.get(i + 1)?.to_string();
            let total = parts.get(i + 3)?.to_string();
            let speed = parts.get(i + 4).map(|s| s.to_string());
            let eta = parts.get(i + 5).map(|s| s.to_string());

            return Some(DownloadProgress {
                percent,
                downloaded,
                total,
                speed,
                eta,
            });
        }
    }
    None
}

#[tauri::command]
pub async fn delete_ollama_model(app: AppHandle, model_name: String) -> Result<(), String> {
    println!("Deleting model: {}", model_name);

    let output = app
        .shell()
        .command("ollama")
        .args(["rm", &model_name])
        .output()
        .await
        .map_err(|e| format!("Failed to delete model: {}", e))?;

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
        .command("ollama")
        .args(["list"])
        .output()
        .await
        .map_err(|e| format!("Failed to list models: {}", e))?;

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

#[tauri::command]
pub async fn execute_crewai_task(app: AppHandle, config: CrewAIConfig) -> Result<String, String> {
    println!("Executing CrewAI task with model: {}", config.model_name);

    // First check if the model is installed
    let installed_models = list_ollama_models(app.clone()).await?;
    if !installed_models.contains(&config.model_name) {
        return Err(format!("Model {} is not installed", config.model_name));
    }

    // Start Python service if not already running
    ensure_python_service(app).await?;

    // Make HTTP request to Python service
    let client = reqwest::Client::new();
    let response = client
        .post("http://127.0.0.1:3001/execute")
        .json(&config)
        .send()
        .await
        .map_err(|e| format!("Failed to execute CrewAI task: {}", e))?;

    if !response.status().is_success() {
        let error = response
            .json::<CrewAIError>()
            .await
            .map_err(|_| "Unknown error".to_string())?;

        if let Some(traceback) = error.traceback {
            eprintln!("Python traceback:\n{}", traceback);
        }
        return Err(error.detail);
    }

    let crew_response = response
        .json::<CrewAIResponse>()
        .await
        .map_err(|e| format!("Failed to parse CrewAI response: {}", e))?;

    Ok(crew_response.result)
}

#[tauri::command]
pub async fn ensure_python_service(app: AppHandle) -> Result<(), String> {
    println!("Starting Python service...");

    // Start Python service as a sidecar
    let (mut rx, child) = app
        .shell()
        .sidecar("python")
        .map_err(|e| format!("Failed to create Python sidecar: {}", e))?
        .args(["-m", "crew_wrapper", "3001"])
        .spawn()
        .map_err(|e| format!("Failed to start Python service: {}", e))?;

    // Store the child process handle in app state
    app.manage(std::sync::Mutex::new(child));

    // Monitor the service output in a separate task
    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(line) => {
                    println!("Python service stdout: {}", String::from_utf8_lossy(&line));
                }
                CommandEvent::Stderr(line) => {
                    eprintln!("Python service stderr: {}", String::from_utf8_lossy(&line));
                }
                CommandEvent::Error(err) => {
                    eprintln!("Python service error: {}", err);
                }
                CommandEvent::Terminated(status) => {
                    eprintln!("Python service terminated with status: {:?}", status);
                }
                _ => {}
            }
        }
    });

    // Wait for service to be ready
    println!("Waiting for Python service to be ready...");
    let mut attempts = 0;
    while attempts < 10 {
        if let Ok(response) = reqwest::get("http://127.0.0.1:3001/docs").await {
            if response.status().is_success() {
                println!("Python service is ready!");
                return Ok(());
            }
        }
        println!("Attempt {}: Python service not ready yet", attempts + 1);
        sleep(Duration::from_secs(1)).await;
        attempts += 1;
    }

    Err("Failed to start Python service after multiple attempts".to_string())
}
