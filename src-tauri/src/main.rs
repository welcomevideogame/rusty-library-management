// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::State;
use std::sync::Mutex;

mod app;
mod types;
mod utils;

#[tauri::command]
async fn authenticate(
    tool: State<'_, Mutex<app::App>>, 
    id: u16, 
    password: &str
) -> Result<bool, String> {
    let mut app = tool.lock().map_err(|_| "Failed to acquire lock")?;
    if app.authenticate_employee(id, password) { Ok(true) } else { Err("Authentication failed".into()) }
}


#[tauri::command]
async fn get_media(tool: State<'_, Mutex<app::App>>) -> Result<String, String> {
    let app = tool.lock().map_err(|_| "Failed to acquire lock")?;
    let new_media: Vec<types::structs::Media> = utils::loading::hashmap_to_vec(app.get_media());

    serde_json::to_string(&new_media).map_err(|_| "Failed to serialize media data".into())
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
        .invoke_handler(tauri::generate_handler![authenticate, get_rank, get_media])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
