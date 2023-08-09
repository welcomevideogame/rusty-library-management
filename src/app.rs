use crate::app::data_manager::manager::DbTool;

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
        let settings = utils::loading::load_db_settings();
        App {
            db_manager: DbTool::new(&settings),
        }
    }

    pub fn run(&mut self) {
        print!(
            "Starting the library management system.\nConnecting to {}",
            self.db_manager.endpoint
        );
    }
}
