use crate::app::data_manager::manager::DbTool;
use crate::types::structs::Employee;

pub mod manager {
    use super::super::utils::network;
    use super::*;
    use postgrest::Postgrest;

    #[derive(Debug)]
    pub enum DbToolError {
        FailConnect,
        FailQuery,
        EntryExists,
    }

    impl std::fmt::Display for DbToolError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match *self {
                DbToolError::FailConnect => write!(f, "Invalid URL"),
                DbToolError::FailQuery => write!(f, "Failed to execute query"),
                DbToolError::EntryExists => write!(f, "Entry already exists"),
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
                return Err(DbToolError::FailConnect);
            }

            let client = Postgrest::new(&endpoint).insert_header("apikey", &api_key);

            client.from("Employee");

            println!("Connecting to {endpoint}");
            Ok(DbTool { salt, client })
        }

        pub async fn get_employee_table(&self) -> Vec<Employee> {
            let resp = self
                .client
                .from("Employee")
                .select("*")
                .execute()
                .await
                .expect("Failed to execute query");
            match resp.text().await {
                Ok(s) => match serde_json::from_str::<Vec<Employee>>(&s) {
                    Ok(emp) => emp,
                    Err(err) => panic!("Error Parsing Employee Data -> {}", err),
                },
                Err(_) => panic!("Invalid table settings. Unable to get data."),
            }
        }

        pub async fn add_employee(&self, employee: Employee) -> Result<(), DbToolError> {
            let _ = self
                .check_employee_exists(&employee)
                .await
                .map_err(|_| return DbToolError::EntryExists)?;

            let body = serde_json::to_value(employee).unwrap();
            let resp = self
                .client
                .from("Employee")
                .insert(body.to_string())
                .execute()
                .await;
            let body = resp.expect("").text().await;
            match body {
                Ok(_) => Ok(()),
                Err(_) => Err(DbToolError::FailConnect),
            }
        }

        async fn check_employee_exists(&self, employee: &Employee) -> Result<(), DbToolError> {
            let resp = self
                .client
                .from("Employee")
                .eq("id", employee.id.to_string())
                .execute()
                .await;
            match resp.expect("Unknown error").text().await {
                Ok(body) => {
                    if !body.contains("name") {
                        // Just any arbitrary column name
                        Ok(())
                    } else {
                        Err(DbToolError::EntryExists)
                    }
                }
                Err(_) => Err(DbToolError::FailQuery),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_employee() -> Employee {
        Employee::new(
            100,
            String::from("John Doe"),
            String::from("IT"),
            1,
            String::from("Ambitious Project"),
            String::from("Computer Science"),
            1_000,
            1,
            String::from("password"),
        )
        .unwrap()
    }

    #[test]
    fn serialize_employee() {
        let test_employee = create_test_employee();
        assert!(serde_json::to_value(test_employee).is_ok());
    }

    #[tokio::test]
    async fn add_employee() {
        let settings = super::super::utils::loading::load_db_settings();
        let tool = DbTool::new(&settings).await.unwrap();

        let test_employee = create_test_employee();
        assert!(tool.add_employee(test_employee).await.is_ok());
    }
}
