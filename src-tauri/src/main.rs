// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use tauri::State;
use types::structs::{DisplayInfo, Media};

mod app;
mod data_manager;
mod types;
mod utils;

#[tauri::command]
async fn authenticate(
    tool: State<'_, Mutex<app::App>>,
    id: u16,
    password: &str,
) -> Result<bool, String> {
    let mut app = tool.lock().map_err(|_| "Failed to acquire lock")?;
    app.authenticate_employee(id, password)
}

/// Converts a HashMap of Media objects to a Vec of Media objects.
/// 
/// # Arguments
/// 
/// * `media_guard` - A reference to the HashMap of Media objects.
/// 
/// # Returns
/// 
/// A Vec of Media objects.
/// 
/// # Example
/// 
/// ```
/// use rusty_library_management::types::structs::Media;
/// use rusty_library_management::utils::loading::hashmap_to_vec;
/// use std::collections::HashMap;
/// 
/// let mut media_guard: HashMap<String, Media> = HashMap::new();
/// // Add some Media objects to the HashMap...
/// 
/// let new_media: Vec<Media> = hashmap_to_vec(&media_guard);
/// ```
#[tauri::command]
async fn get_media(tool: State<'_, Mutex<app::App>>) -> Result<String, String> {
    let app = tool.lock().map_err(|_| "Failed to acquire lock")?;
    let media_guard = app.get_media();
    let new_media: Vec<types::structs::Media> = utils::loading::hashmap_to_vec(&*media_guard);

    serde_json::to_string(&new_media).map_err(|_| "Failed to serialize media data".into())
}

#[tauri::command]
async fn search_media(tool: State<'_, Mutex<app::App>>, search: &str) -> Result<String, String> {
    // TODO: Change the logic so that it creates a new map with the name, that way we get O(nlogn) instead of O(n^2log(n))
    let app = tool.lock().map_err(|_| "Failed to acquire lock")?;
    let mut media_vec: Vec<Media> = vec![];

    let names = app
        .search_by_id::<Media>(search)
        .ok_or_else(|| "No media found".to_string())?;
    for med in app.get_media().values() {
        let med_name = med.get_name().to_lowercase();
        if names.iter().any(|n| n == &med_name)
            && !media_vec.iter().any(|m| m.get_name() == med_name)
        {
            media_vec.push(med.clone());
        }
    }
    dbg!(&media_vec);
    serde_json::to_string(&media_vec).map_err(|_| "Failed to serialize media data".into())
}

/// This function is annotated with the `tauri::command` attribute and is called `get_rank`. 
/// It takes a `State` parameter containing a `Mutex` wrapped instance of the `App` struct. 
/// The function attempts to acquire a lock on the `App` instance and then calls the `get_permission_level` method on it.
/// If the method returns `Some`, it converts the result to a string and returns it. Otherwise, it returns an error message.
///
/// # Example
/// ```rust
/// #[tauri::command]
/// async fn get_rank(tool: State<'_, Mutex<app::App>>) -> Result<String, String> {
///     let app = tool.lock().map_err(|_| "Failed to acquire lock")?;
///     app.get_permission_level().map_or_else(
///         || Err("Could not find user".into()),
///         |res| Ok(res.to_string()),
///     )
/// }
/// ```
///
/// # Arguments
/// - `tool`: A `State` parameter containing a `Mutex` wrapped instance of the `App` struct.
///
/// # Returns
/// - If the `get_permission_level` method returns `Some`, the function returns the permission level as a string.
/// - If the `get_permission_level` method returns `None`, the function returns an error message.
#[tauri::command]
async fn get_rank(tool: State<'_, Mutex<app::App>>) -> Result<String, String> {
    let app = tool.lock().map_err(|_| "Failed to acquire lock")?;
    app.get_permission_level().map_or_else(
        || Err("Could not find user".into()),
        |res| Ok(res.to_string()),
    )
}


#[tauri::command]
async fn media_checkout(tool: State<'_, Mutex<app::App>>, cart: &str) -> Result<String, String> {
    let cart_items: Vec<types::structs::Media> = serde_json::from_str(cart)
        .map_err(|_| "Failed to parse cart data".to_string())?;
    let media_ids: Vec<u16> = cart_items.into_iter().map(|item| item.get_id()).collect();
    let mut app = tool.lock().map_err(|_| "Failed to acquire lock")?;
    for media_id in media_ids {
        _ = app.rent_media(media_id);
    }
    Ok("Checkout successful".to_string())
}


fn main() {
    let app = Mutex::new(app::App::new());
    app.lock().unwrap().run();

    tauri::Builder::default()
        .manage(app)
        .invoke_handler(tauri::generate_handler![
            authenticate,
            get_rank,
            get_media,
            search_media,
            media_checkout,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
