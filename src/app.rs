use crate::app::data_manager::manager::DbTool;
use crate::types::structs::{DisplayInfo, Employee};
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
        let employees: Vec<Employee> = rt.block_on(self.db_manager.get_table());
        for emp in employees {
            println!("{}", emp.get_id().to_string());
        }
    }
}
