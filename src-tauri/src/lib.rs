use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
mod constants;
use constants::render_prompt;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn prompt_enhancer(prompt: &str) -> Result<String, String> {
    let ollama = Ollama::default();
    let model = "llama3.2:1b".to_string();

    let mut request = GenerationRequest::new(model, prompt.to_string());
    request = request.system(render_prompt(prompt));

    let res = ollama.generate(request).await;
    match res {
        Ok(response) => {
            return Ok(response.response);
        }
        Err(e) => {
            return Err(format!("Error: {}", e));
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, prompt_enhancer])
        .run(tauri::generate_context!())
        .expect("error while running Psyborg application");
}
