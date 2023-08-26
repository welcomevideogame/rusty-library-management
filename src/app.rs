use super::utils;
use crate::app::data_manager::manager::{DbTool, DbToolError};
use crate::types::enums::PermissionLevel;
use crate::types::structs::{DisplayInfo, Employee, Media};
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
        App {
            db_manager,
            employees,
            user: 0,
            media,
            rt,
        }
    }

    pub fn run(&mut self) {
        self.update_data();
        println!("Welcome to the library management system");
        self.request_login();
        self.main_loop();
    }

    fn update_data(&mut self) {
        self.employees =
            utils::loading::vec_to_hashmap(self.rt.block_on(self.db_manager.get_table()));
        self.media = utils::loading::vec_to_hashmap(self.rt.block_on(self.db_manager.get_table()));
    }

    fn request_login(&mut self) {
        loop {
            let employee_id = self.prompt_for_employee_id();
            if employee_id == 0 {
                return;
            }
            if let Some(employee) = self.employees.get(&employee_id) {
                if self.authenticate_employee(employee, employee_id) {
                    self.user = employee_id;
                    println!("Welcome, {}", employee.get_name());
                    return;
                }
            } else {
                println!("Employee ID of {} not found", employee_id);
            }
        }
    }

    fn prompt_for_employee_id(&self) -> u16 {
        loop {
            println!("Please enter your employee number. (0 to be guest)");
            let response = utils::user::get_input();
            match response.parse::<u16>() {
                Ok(value) => return value,
                Err(_) => println!("Invalid employee ID"),
            }
        }
    }

    fn authenticate_employee(&self, employee: &Employee, employee_id: u16) -> bool {
        loop {
            println!(
                "Employee ID {}. Enter password or 'exit' to exit",
                employee_id
            );
            let response = utils::user::get_input();
            if response.as_str().to_lowercase() == "exit" {
                return false;
            }
            match utils::security::verify_password(employee.password(), response.as_str()) {
                Ok(matches) => {
                    if matches {
                        return true;
                    } else {
                        println!("Incorrect password");
                    }
                }
                Err(_) => println!("Error checking password"),
            }
        }
    }

    // TODO - Make this more concise
    fn main_loop(&self) {
        let mut feature_permissions = HashMap::new();
        feature_permissions.insert("View Employees", PermissionLevel::User);
        feature_permissions.insert("View Media", PermissionLevel::Basic);
        feature_permissions.insert("Create Employee", PermissionLevel::Manager);
        feature_permissions.insert("Create Media", PermissionLevel::Admin);

        loop {
            println!(
                "Enter an option:\n\
                \t1: View Employees\n\
                \t2: View Media\n\
                \t3: Create Employee\n\
                \t4: Create Media"
            );

            let response = utils::user::get_input();
            let user_permission_level = self.employees.get(&self.user).unwrap().perm_level();

            match response.as_str() {
                "1" => {
                    if user_permission_level
                        >= feature_permissions
                            .get("View Employees")
                            .unwrap_or(&PermissionLevel::None)
                    {
                        _ = self.item_selection::<Employee>(&self.employees);
                    } else {
                        println!("Insufficient permission level.");
                    }
                }
                "2" => {
                    if user_permission_level
                        >= feature_permissions
                            .get("View Media")
                            .unwrap_or(&PermissionLevel::None)
                    {
                        _ = self.item_selection::<Media>(&self.media);
                    } else {
                        println!("Insufficient permission level.");
                    }
                }
                "3" => {
                    if user_permission_level
                        >= feature_permissions
                            .get("Create Employee")
                            .unwrap_or(&PermissionLevel::None)
                    {
                        match self.create_obj::<Employee>() {
                            Ok(_) => println!("Successfully created the employee!"),
                            Err(e) => println!("Failed to create the employee -> {}", e),
                        }
                    } else {
                        println!("Insufficient permission level.");
                    }
                }
                "4" => {
                    if user_permission_level
                        >= feature_permissions
                            .get("Create Media")
                            .unwrap_or(&PermissionLevel::None)
                    {
                        match self.create_obj::<Media>() {
                            Ok(_) => println!("Successfully created the media object!"),
                            Err(e) => println!("Failed to create the media object -> {}", e),
                        }
                    } else {
                        println!("Insufficient permission level.");
                    }
                }
                _ => {
                    println!("Invalid option.");
                }
            }
        }
    }

    fn item_selection<T: DisplayInfo + ToString>(&self, items: &HashMap<u16, T>) -> Option<()> {
        let obj = self.list_items::<T>(items)?;
        println!(
            "Choose an option for {}\
            \n1: Show Details\
            \n2: Change Information\
            \n3: Delete\
            \n4: Exit",
            obj.get_name()
        );

        let response = utils::user::get_input();
        match response.as_str() {
            "1" => println!("{}", obj.to_string()),
            "2" => {
                if let Err(err) = self.update_item(obj) {
                    println!("Failed to change information -> {}", err);
                }
            }
            "3" => {
                if let Err(err) = self.delete_item(obj) {
                    println!("Failed to delete the entry -> {}", err)
                }
            }
            "4" => (),
            _ => (),
        }
        Some(())
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
}
