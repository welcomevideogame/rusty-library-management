use crate::app::data_manager::manager::DbTool;
use crate::types::structs::{DisplayInfo, Employee, Media};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::error::Error;

use tokio::runtime::Runtime;

mod utils {
    include!("utils.rs");
}
mod data_manager {
    include!("data_manager.rs");
}

pub struct App {
    db_manager: DbTool,
    employees: HashMap<u16, Employee>,
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
            media,
            rt,
        }
    }

    pub fn run(&mut self) {
        self.update_data();
        println!("Welcome to the library management system");
        loop {
            println!(
                "Enter an option:\n\
            \t1: View Employees\n\
            \t2: View Media\n"
            );
            let response = utils::user::get_input();
            match response.as_str() {
                "1" => _ = self.item_selection::<Employee>(&self.employees),
                "2" => _ = self.item_selection::<Media>(&self.media),
                _ => (),
            }
        }
    }

    pub fn update_data(&mut self) {
        self.employees =
            utils::loading::vec_to_hashmap(self.rt.block_on(self.db_manager.get_table()));
        self.media = utils::loading::vec_to_hashmap(self.rt.block_on(self.db_manager.get_table()));
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
                    println!("{}", err);
                }
            }
            "3" => todo!("deleting entry"),
            "4" | _ => (),
        }
        Some(())
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
        let mut json_obj = serde_json::to_value(&obj).unwrap();
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

        let updated_obj: T =
            serde_json::from_value(json_obj).map_err(|_| "Failed to update object")?;

        self.rt
            .block_on(self.db_manager.database_update(&updated_obj))
            .map_err(|_| "Failed to update on database")?;
        Ok(())
    }
}
