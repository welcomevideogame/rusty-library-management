pub mod manager {
    pub struct DbTool {
        pub endpoint: String,
        api_key: String,
    }
    impl DbTool {
        pub fn new(args: &[String]) -> DbTool {
            DbTool {
                endpoint: args[0].clone(),
                api_key: args[1].clone(),
            }
        }
        pub fn connect(&mut self) {}
    }
}
