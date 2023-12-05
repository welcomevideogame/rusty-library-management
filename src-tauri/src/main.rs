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
fn test() -> bool{
    true
}


fn main() {
    let app = Mutex::new(app::App::new());
    app.lock().unwrap().run();

    tauri::Builder::default()
        .manage(app)    
        .invoke_handler(tauri::generate_handler![authenticate, test])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
