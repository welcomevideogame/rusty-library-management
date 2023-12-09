use super::utils;
use crate::app::data_manager::manager::{DbTool, DbToolError};
use crate::types::enums::PermissionLevel;
use crate::types::structs::{DisplayInfo, Employee, Media, Trie};
use serde_json::Value;
use std::collections::HashMap;
use tokio::runtime::Runtime;

mod data_manager {
    include!("data_manager.rs");
}

pub struct App {
    db_manager: DbTool,
    employees: HashMap<u16, Employee>,
    user: u16,
    media: HashMap<u16, Media>,
    trie: HashMap<&'static str, Trie>,
    rt: Runtime,
}

impl App {
    pub fn new() -> App {
        println!("Starting the library management system...");
        let settings = utils::loading::load_db_settings();
        let rt = Runtime::new().unwrap();
        let db_manager = rt
            .block_on(DbTool::new(&settings))
            .expect("Failed to connect to the database");
        println!("Connected to the database");
        let employees: HashMap<u16, Employee> = HashMap::new();
        let media: HashMap<u16, Media> = HashMap::new();
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
        self.update_data();
    }

    pub fn authenticate_employee(&mut self, employee_id: u16, password: &str) -> bool {
        if let Some(emp) = self.employees.get(&employee_id) {
            utils::security::verify_password(emp.password(), password).unwrap_or_else(|_| false)
        }
        else {
            false
        }
    }

    pub fn get_permission_level(&self) -> Option<&PermissionLevel> {
        self.employees.get(&self.user).map(|emp| emp.perm_level())
    }


    fn update_data(&mut self) {
        self.employees = utils::loading::vec_to_hashmap(self.rt.block_on(self.db_manager.get_table()));
        self.media = utils::loading::vec_to_hashmap(self.rt.block_on(self.db_manager.get_table()));
        self.trie.insert(Employee::get_table_name(), utils::loading::hashmap_to_trie(&self.employees));
        self.trie.insert(Media::get_table_name(), utils::loading::hashmap_to_trie(&self.media));
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

    fn search_items<'a, T: DisplayInfo + ToString>(
        &self,
        _items: &'a HashMap<u16, T>
    ) -> Option<&'a T> {
        println!("Enter the name of the item you want to search");
        let name = utils::user::get_input();
        let name_vec: &Trie = self.trie.get(T::get_table_name())?;

        let names = name_vec.starts_with(name)?;
        println!("found some");
        for name in names{
            println!("{}", name);
        }
        None
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

    pub fn get_media(&self) -> &HashMap<u16, Media> {
        &self.media
    }

}