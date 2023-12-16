use crate::types::structs::DisplayInfo;
use serde::de::DeserializeOwned;
use std::fmt;

use serde::Serialize;
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
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                DbToolError::FailConnect => write!(f, "Invalid URL"),
                DbToolError::FailQuery => write!(f, "Failed to execute query"),
                DbToolError::EntryExists => write!(f, "Entry already exists"),
                DbToolError::BadEntry => write!(f, "Entry does not exist"),
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
            let table_name = format!("{}{}", self.salt, T::get_table_name());
            let resp = self
                .client
                .from(table_name)
                .select("*")
                .execute()
                .await
                .expect("Failed to execute query");
            match resp.text().await {
                Ok(s) => match serde_json::from_str(&s) {
                    Ok(vec_t) => vec_t,
                    Err(err) => panic!(
                        "Error parsing data -> {}, most likely an invalid API url",
                        err
                    ),
                },
                Err(_) => panic!("Invalid table settings. Unable to get data."),
            }
        }

        pub async fn database_insert<T: DisplayInfo + Serialize>(
            &self,
            obj: &T,
        ) -> Result<(), DbToolError> {
            self.check_entry_exists::<T>(obj)
                .await
                .map_err(|_| DbToolError::EntryExists)?;
            let table_name = format!("{}{}", self.salt, T::get_table_name());
            let body = serde_json::to_value(obj).unwrap();
            let resp = self
                .client
                .from(table_name)
                .insert(body.to_string())
                .execute()
                .await;
            match resp.expect("Unknown Error").text().await {
                Ok(_) => Ok(()),
                Err(_) => Err(DbToolError::FailConnect),
            }
        }

        pub async fn database_update<T: DisplayInfo + Serialize>(
            &self,
            obj: &T,
        ) -> Result<(), DbToolError> {
            if let Ok(()) = self.check_entry_exists::<T>(obj).await {
                return Err(DbToolError::BadEntry);
            }

            let table_name = format!("{}{}", self.salt, T::get_table_name());
            let body = serde_json::to_value(obj).unwrap();
            self.client
                .from(table_name)
                .eq("id", obj.get_id().to_string())
                .update(body.to_string())
                .execute()
                .await
                .expect("Unknown Error")
                .text()
                .await
                .map_err(|_| DbToolError::EntryExists)?;
            Ok(())
        }

        pub async fn database_delete<T: DisplayInfo + Serialize>(
            &self,
            obj: &T,
        ) -> Result<(), DbToolError> {
            if let Ok(()) = self.check_entry_exists::<T>(obj).await {
                return Err(DbToolError::BadEntry);
            }
            let table_name = format!("{}{}", self.salt, T::get_table_name());
            self.client
                .from(table_name)
                .eq("id", obj.get_id().to_string())
                .delete()
                .execute()
                .await
                .expect("Unknown Error")
                .text()
                .await
                .map_err(|_| DbToolError::FailQuery)?;
            Ok(())
        }

        async fn check_entry_exists<T: DisplayInfo>(&self, obj: &T) -> Result<(), DbToolError> {
            let table_name = format!("{}{}", self.salt, T::get_table_name());
            let resp = self
                .client
                .from(table_name)
                .eq("id", obj.get_id().to_string())
                .execute()
                .await;
            match resp.expect("Unknown error").text().await {
                Ok(body) => {
                    if !body.contains("id") {
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

// Tests
// ---------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::enums::{MediaType, PermissionLevel};
    use crate::types::structs::{Employee, Media};

    fn create_test_employee() -> Employee {
        Employee::new(
            1005,
            String::from("John Doe"),
            String::from("IT"),
            1,
            String::from("Ambitious Project"),
            String::from("Computer Science"),
            1_000,
            PermissionLevel::Admin,
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
        let settings = super::super::utils::loading::load_db_settings().unwrap();
        let tool = manager::DbTool::new(&settings).await.unwrap();

        let test_employee = create_test_employee();
        assert!(tool
            .database_insert::<Employee>(&test_employee)
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn update_employee() {
        let settings = super::super::utils::loading::load_db_settings().unwrap();
        let tool = manager::DbTool::new(&settings).await.unwrap();
        let mut test_employee = create_test_employee();
        test_employee.set_name("Jane Doe".to_owned());
        assert!(tool
            .database_update::<Employee>(&test_employee)
            .await
            .is_ok());
    }
}
