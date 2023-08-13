use crate::app::data_manager::manager::DbTool;
use crate::types::structs::{Employee, Media};
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
        App { db_manager }
    }

    pub fn run(&mut self) {
        let rt = Runtime::new().unwrap();
        println!("Welcome to the library management system");
        loop {
            println!(
                "Enter an option:\n\
            \t1: View Employees\n\
            \t2: View Media\n"
            );
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            match buffer.trim() {
                "1" => {
                    self.print_employees(&rt);
                }
                "2" => {
                    self.print_media(&rt);
                }
                _ => {
                    println!("Not a valid option");
                }
            }
        }
    }

    fn print_employees(&mut self, rt: &Runtime) {
        let employees: Vec<Employee> = rt.block_on(self.db_manager.get_table());
        for employee in employees {
            println!("{}", employee.to_string())
        }
    }

    fn print_media(&mut self, rt: &Runtime) {
        let media: Vec<Media> = rt.block_on(self.db_manager.get_table());
        for med in media {
            println!("{}", med.to_string())
        }
    }
}
