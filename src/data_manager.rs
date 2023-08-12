use crate::types::structs::Employee;
use std::error::Error;
use std::fmt;

pub mod manager {
    use super::super::utils::network;
    use crate::types;
    use postgrest::Postgrest;

    #[derive(Debug)]
    pub enum DbToolError {
        Timeout,
    }

    impl std::fmt::Display for DbToolError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match *self {
                DbToolError::Timeout => write!(f, "Invalid URL"),
            }
        }
    }

    pub struct DbTool {
        salt: String,
        client: Postgrest,
    }
    impl DbTool {
        pub async fn new(args: &[String]) -> Result<DbTool, DbToolError> {
            let endpoint = args[0].clone();
            let api_key = args[1].clone();
            let salt = args[2].clone();

            let res = network::test_network(&endpoint).await;
            if !res {
                return Err(DbToolError::Timeout);
            }

            let client = Postgrest::new(&endpoint).insert_header("apikey", &api_key);

            client.from("Employee");

            println!("Connecting to {endpoint}");
            Ok(DbTool { salt, client })
        }

        pub async fn get_employee_table(&self) -> Vec<types::structs::Employee> {
            let resp = self
                .client
                .from("Employee")
                .select("*")
                .execute()
                .await
                .expect("Failed to execute query");
            match resp.text().await {
                Ok(s) => match serde_json::from_str::<Vec<types::structs::Employee>>(&s) {
                    Ok(emp) => emp,
                    Err(err) => panic!("Error Parsing Employee Data -> {}", err),
                },
                Err(_) => panic!("Invalid table settings. Unable to get data."),
            }
        }
    }
}
