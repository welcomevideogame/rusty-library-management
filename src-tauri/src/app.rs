use super::utils;
use crate::app::data_manager::manager::{DbTool, DbToolError};
use crate::types::enums::PermissionLevel;
use crate::types::structs::{DisplayInfo, Employee, Media, Trie};
use serde_json::Value;
use std::collections::HashMap;
use std::mem;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

mod data_manager {
    include!("data_manager.rs");
}


pub struct App {
    db_manager: DbTool,
    employees: Arc<Mutex<HashMap<u16, Employee>>>,
    user: u16,
    media: Arc<Mutex<HashMap<u16, Media>>>,
    trie: HashMap<&'static str, Trie>,
    rt: Runtime,
}

impl App {
    pub fn new() -> App {
        println!("Starting the library management system...");
        let settings =
            utils::loading::load_db_settings().expect("Failed to load database settings");
        let rt = Runtime::new().unwrap();
        let db_manager = rt
            .block_on(DbTool::new(&settings))
            .expect("Failed to connect to the database");
        println!("Connected to the database");
        let employees = Arc::new(Mutex::new(HashMap::<u16, Employee>::new()));
        let media = Arc::new(Mutex::new(HashMap::<u16, Media>::new()));
        let trie: HashMap<&'static str, Trie> = HashMap::new();
        App {
            db_manager,
            employees,
            user: 0,
            media,
            trie,
            rt,
        }
    }

    pub fn run(&mut self) {
        self.refresh_all_data();
    }

    pub fn authenticate_employee(
        &mut self,
        employee_id: u16,
        password: &str,
    ) -> Result<bool, String> {
        let employees = self
            .employees
            .lock()
            .map_err(|_| "Failed to acquire lock".to_string())?;

        employees
            .get(&employee_id)
            .and_then(|emp| utils::security::verify_password(emp.password(), password).ok())
            .map_or(Ok(false), |res| {
                if res {
                    self.user = employee_id;
                }
                Ok(res)
            })
            .or_else(|_: String| Err("Password verification failed".into()))
    }

    pub fn get_permission_level(&self) -> Option<PermissionLevel> {
        let employees = self.employees.lock().ok()?;
        employees
            .get(&self.user)
            .map(|emp| emp.perm_level().to_owned())
    }

    pub fn search_by_id<'a, T: DisplayInfo + ToString>(&self, search: &str) -> Option<Vec<String>> {
        let name_vec: &Trie = self.trie.get(T::get_table_name())?;
        name_vec.starts_with(search.into())
    }

    fn update_data<T: DisplayInfo>(&mut self, data: Vec<T>, storage: &Arc<Mutex<HashMap<u16, T>>>) {
        let mut storage_guard = storage.lock().unwrap_or_else(|e| e.into_inner());
        *storage_guard = utils::loading::vec_to_hashmap(data);
        self.trie.insert(
            T::get_table_name(),
            utils::loading::hashmap_to_trie(&storage_guard),
        );
    }

    pub fn refresh_all_data(&mut self) {
        // TODO: Redesign this so there is no need to temporarily take ownership of the Arc<Mutex<...>> fields
        let temp_employees = mem::take(&mut self.employees);
        let temp_media = mem::take(&mut self.media);
        let emp_data = self.rt.block_on(self.db_manager.get_table::<Employee>());
        let media_data = self.rt.block_on(self.db_manager.get_table::<Media>());

        self.update_data(emp_data, &temp_employees);
        self.update_data(media_data, &temp_media);
        self.employees = temp_employees;
        self.media = temp_media;
    }

    pub async fn rent_media(&mut self, media_id: u16) -> Result<(), String> {
        let current_user = self.get_current_user()?;
        let media_guard = self.media.lock().map_err(|_| "Failed to acquire lock")?;
        let media = media_guard
            .get(&media_id)
            .ok_or_else(|| "Media not found".to_string())?;
        let mut media = media.clone();
        media.set_renter(current_user.get_name().to_owned());
        self.db_manager.database_update(&media).await
            .map_err(|_| "Failed to update on database".to_string())?;
        Ok(())
    }

    fn get_current_user(&self) -> Result<Employee, String> {
        self.employees
            .lock()
            .map_err(|_| "Failed to acquire lock".to_string())?
            .get(&self.user)
            .cloned()
            .ok_or_else(|| "User not found".to_string())
    }

    fn create_obj<T: DisplayInfo + Default + serde::Serialize + serde::de::DeserializeOwned>(
        &self,
    ) -> Result<(), String> {
        let mut json_obj = serde_json::to_value(T::default()).unwrap();
        let keys: Vec<String> = json_obj.as_object().unwrap().keys().cloned().collect();
        for key in &keys {
            println!("Enter a new value for {}", key);
            let input = utils::user::get_input();
            let original_value = &json_obj[&key];
            let new_value = match original_value {
                Value::Bool(_) => Value::Bool(
                    input
                        .parse()
                        .map_err(|_| format!("Expected a boolean value for {}", key))?,
                ),
                Value::Number(_) => Value::Number(
                    input
                        .parse()
                        .map_err(|_| format!("Expected a numeric value for {}", key))?,
                ),
                _ => Value::String(input),
            };
            json_obj[&key] = new_value;
        }
        let mut obj: T = serde_json::from_value(json_obj).map_err(|e| e.to_string())?;
        obj.additional_setup();
        self.rt
            .block_on(self.db_manager.database_insert(&obj))
            .map_err(|_| "Failed to update on database".to_string())?;
        Ok(())
    }

    fn list_items<'a, T: DisplayInfo + ToString>(
        &self,
        items: &'a HashMap<u16, T>,
    ) -> Option<&'a T> {
        items
            .iter()
            .for_each(|(key, val)| println!("{} -> {}", key, val.get_name()));

        let response: u16 = match utils::user::get_input().parse() {
            Ok(n) => n,
            Err(_) => return None,
        };
        items.get(&response)
    }

    fn update_item<T: DisplayInfo>(&self, obj: &T) -> Result<(), &str> {
        let mut json_obj = serde_json::to_value(obj).unwrap();
        let keys: Vec<String> = json_obj.as_object().unwrap().keys().cloned().collect();

        println!("Enter the value that you want to change");
        for (key, value) in json_obj.as_object().unwrap().keys().enumerate() {
            println!("{} - {}", key + 1, value)
        }

        let response: usize = match utils::user::get_input().trim().parse() {
            Ok(num) => num,
            Err(_) => return Err("Invalid number"),
        };
        let field_name = &keys[response - 1];
        println!("Enter a new value for {}", field_name);

        let new_value = utils::user::get_input().trim().to_string();
        json_obj[field_name] = Value::String(new_value);

        let mut updated_obj: T =
            serde_json::from_value(json_obj).map_err(|_| "Failed to update object")?;
        updated_obj.additional_setup();

        self.rt
            .block_on(self.db_manager.database_update(&updated_obj))
            .map_err(|_| "Failed to update on database")?;
        Ok(())
    }

    fn delete_item<T: DisplayInfo>(&self, item: &T) -> Result<(), DbToolError> {
        self.rt.block_on(self.db_manager.database_delete(item))?;
        Ok(())
    }

    pub fn get_media(&self) -> std::sync::MutexGuard<HashMap<u16, Media>> {
        self.media.lock().expect("Failed to lock media mutex")
    }
}
