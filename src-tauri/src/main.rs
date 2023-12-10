// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::State;
use types::structs;
use std::sync::Mutex;

mod app;
mod types;
mod utils;
mod data_manager;

#[tauri::command]
async fn authenticate(
    tool: State<'_, Mutex<app::App>>, 
    id: u16, 
    password: &str
) -> Result<bool, String> {
    let mut app = tool.lock().map_err(|_| "Failed to acquire lock")?;
    app.authenticate_employee(id, password)
}


#[tauri::command]
async fn get_media(tool: State<'_, Mutex<app::App>>) -> Result<String, String> {
    let app = tool.lock().map_err(|_| "Failed to acquire lock")?;
    let media_guard = app.get_media(); // This is a MutexGuard
    let new_media: Vec<types::structs::Media> = utils::loading::hashmap_to_vec(&*media_guard);

    serde_json::to_string(&new_media).map_err(|_| "Failed to serialize media data".into())
}


#[tauri::command]
async fn search_media(
    tool: State<'_, Mutex<app::App>>,
    search: &str
) -> Result<String, String> {
    // TODO: After getting the Vec<String>, search the HashMap to get the Media data to show user
    let app = tool.lock().map_err(|_| "Failed to acquire lock")?;
    app.search_items::<structs::Media>(search)
        .ok_or_else(|| "No media found".to_string())
        .and_then(|media| {
            serde_json::to_string(&media).map_err(|_| "Failed to serialize media data".into())
        })
}


#[tauri::command]
async fn get_rank(tool: State<'_, Mutex<app::App>>) -> Result<String, String> {
    let app = tool.lock().map_err(|_| "Failed to acquire lock")?;
    app.get_permission_level().map_or_else(|| Err("Could not find user".into()), |res| Ok(res.to_string()))
}


fn main() {
    let app = Mutex::new(app::App::new());
    app.lock().unwrap().run();

    tauri::Builder::default()
        .manage(app)    
        .invoke_handler(tauri::generate_handler![authenticate, get_rank, get_media, search_media])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
