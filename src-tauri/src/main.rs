// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::{Deserialize, Serialize};
use tauri::State;
use std::sync::Mutex;


#[derive(Deserialize, Serialize)]
struct DBManager{
    i: i32,
}

impl DBManager{
    fn new() -> Self{
        DBManager {
            i: 10,
        }
    }

    fn get_connection(&self) -> bool {
        false
    }

    fn get_data(&self) -> String{
        String::from("This is a test")
    }

    fn log_in(&self) -> bool{
        true
    }
}

#[tauri::command]
fn test(manager: State<Mutex<DBManager>>) -> bool {
    manager.lock().unwrap().log_in()
}

#[tauri::command]
fn log_in(username: &str, password: &str) -> bool{
    username == password
}

fn main() {
    let db_manager = Mutex::new(DBManager::new());

    tauri::Builder::default()
        .manage(db_manager)    
        .invoke_handler(tauri::generate_handler![log_in, test])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
