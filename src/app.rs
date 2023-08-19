use crate::app::data_manager::manager::DbTool;
use crate::types::structs::{DisplayInfo, Employee, Media};
use std::collections::HashMap;
use std::io;
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
        }
    }

    pub fn run(&mut self) {
        let rt = Runtime::new().unwrap();
        self.update_data(&rt);
        println!("Welcome to the library management system");
        loop {
            println!(
                "Enter an option:\n\
            \t1: View Employees\n\
            \t2: View Media\n"
            );
            let response = utils::user::get_input();
            match response.as_str() {
                "1" => self.print_items(&self.employees),
                "2" => self.print_items(&self.media),
                _ => {
                    println!("Not a valid option")
                }
            }
        }
    }

    pub fn update_data(&mut self, rt: &Runtime) {
        self.employees = utils::loading::vec_to_hashmap(rt.block_on(self.db_manager.get_table()));

        self.media = utils::loading::vec_to_hashmap(rt.block_on(self.db_manager.get_table()));
    }

    fn print_items<T: DisplayInfo + ToString>(&self, items: &HashMap<u16, T>) {
        let _ = items
            .iter()
            .for_each(|(key, val)| println!("{} -> {}", key, val.get_name()));

        let response: u16 = match utils::user::get_input().parse() {
            Ok(n) => n,
            Err(_) => return,
        };
        if let Some(obj) = items.get(&response) {
            println!("{}", obj.to_string());
        } else {
            println!("No entry with ID {} found", response);
        }
    }
}
