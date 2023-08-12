use crate::types::enums::MediaType;
use crate::types::structs::{DisplayInfo, Employee, Media};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::fmt::Display;

pub mod manager {
    use super::super::utils::network;
    use super::*;
    use postgrest::Postgrest;

    #[derive(Debug)]
    pub enum DbToolError {
        FailConnect,
        FailQuery,
        EntryExists,
        BadEntry,
    }

    impl Display for DbToolError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match *self {
                DbToolError::FailConnect => write!(f, "Invalid URL"),
                DbToolError::FailQuery => write!(f, "Failed to execute query"),
                DbToolError::EntryExists => write!(f, "Entry already exists"),
                DbToolError::BadEntry => write!(f, "Entry already exists"),
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

            println!("Connecting to {endpoint}");
            Ok(DbTool { salt, client })
        }

        pub async fn get_table<T: DisplayInfo + DeserializeOwned>(&self) -> Vec<T> {
            let resp = self
                .client
                .from(T::get_table_name())
                .select("*")
                .execute()
                .await
                .expect("Failed to execute query");
            match resp.text().await {
                Ok(s) => match serde_json::from_str(&s) {
                    Ok(vec_t) => vec_t,
                    Err(err) => panic!("Error parsing data -> {}", err),
                },
                Err(_) => panic!("Invalid table settings. Unable to get data."),
            }
        }

        pub async fn database_insert<T: DisplayInfo + serde::Serialize>(
            &self,
            obj: T,
        ) -> Result<(), DbToolError> {
            let _ = self
                .check_entry_exists::<T>(&obj)
                .await
                .map_err(|_| return DbToolError::EntryExists)?;

            let body = serde_json::to_value(obj).unwrap();
            let resp = self
                .client
                .from(T::get_table_name())
                .insert(body.to_string())
                .execute()
                .await;
            let body = resp.expect("").text().await;
            match body {
                Ok(_) => Ok(()),
                Err(_) => Err(DbToolError::FailConnect),
            }
        }

        async fn check_entry_exists<T: DisplayInfo>(&self, obj: &T) -> Result<(), DbToolError> {
            let resp = self
                .client
                .from(T::get_table_name())
                .eq("id", obj.get_id().to_string())
                .execute()
                .await;
            match resp.expect("Unknown error").text().await {
                Ok(body) => {
                    if !body.contains("id") {
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
            1005,
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

    fn create_test_media() -> Media {
        Media::new(
            1005,
            MediaType::VideoGame,
            String::from("Sonic Unleashed"),
            true,
            String::from("Sega"),
            String::from("Renter"),
        )
    }

    #[test]
    fn serialize_employee() {
        let test_employee = create_test_employee();
        assert!(serde_json::to_value::<Employee>(test_employee).is_ok());
    }

    #[test]
    fn serialize_media() {
        let test_media = create_test_media();
        assert!(serde_json::to_value::<Media>(test_media).is_ok())
    }

    #[tokio::test]
    async fn add_employee() {
        let settings = super::super::utils::loading::load_db_settings();
        let tool = manager::DbTool::new(&settings).await.unwrap();

        let test_employee = create_test_employee();
        assert!(tool
            .database_insert::<Employee>(test_employee)
            .await
            .is_ok());
    }
}
