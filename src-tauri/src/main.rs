// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::State;
use std::sync::Mutex;

mod app;
mod types;
mod utils;

#[tauri::command]
async fn authenticate<'a>(
    tool: State<'_, Mutex<app::App>>, 
    id: u16, 
    password: &'a str
) -> Result<bool, String> {
    match tool.lock() {
        Ok(app) => {
            println!("{}", app.authenticate_employee(id, password));
            match app.authenticate_employee(id, password) {
                true => Ok(true),
                false => Err("Authentication failed".to_string()),
            }
        },
        Err(_) => Err("Failed to acquire lock".to_string()),
    }
}

#[tauri::command]
async fn get_media(tool: State<'_, Mutex<app::App>>) -> Result<String, String>{
    match tool.lock() {
        Ok(app) => {
            let new_media: Vec<types::structs::Media> = app.media().values().cloned().collect();
            match serde_json::to_string(&new_media) {
                Ok(json) => {
                    Ok(json)
                }
                Err(_) => Err("Failed to serialize media data".to_string())
            }
        },
        Err(_) => Err("Failed to acquire lock".to_string()),
    }
}


fn main() {
    let app = Mutex::new(app::App::new());
    app.lock().unwrap().run();

    tauri::Builder::default()
        .manage(app)    
        .invoke_handler(tauri::generate_handler![authenticate, get_media])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
