pub mod structs {

    use super::super::utils;
    use crate::types::enums::{MediaType, PermissionLevel};
    use serde::{Deserialize, Serialize};
    use std::fmt;

    // Struct Definitions
    // ---------------------------------------------------------------

    #[derive(serde::Deserialize, serde::Serialize, Debug, Default)]
    pub struct Employee {
        id: u16,
        name: String,
        department: String,
        boss_id: u16,
        project: String,
        subject: String,
        alloc_budget: u16,
        perm_level: PermissionLevel,
        password: String,
    }

    #[derive(serde::Deserialize, serde::Serialize, Debug, Default)]
    pub struct Media {
        id: u16,
        media_type: MediaType,
        name: String,
        borrowable: bool,
        vendor: String,
        renter: String,
    }

    // Trait Implementation
    // ---------------------------------------------------------------

    pub trait DisplayInfo: Serialize + for<'de> Deserialize<'de> {
        fn get_id(&self) -> u16;
        fn get_name(&self) -> &str;
        fn get_table_name() -> &'static str;
        fn additional_setup(&mut self);
    }

    impl DisplayInfo for Employee {
        fn get_id(&self) -> u16 {
            self.id
        }
        fn get_name(&self) -> &str {
            self.name.as_str()
        }
        fn get_table_name() -> &'static str {
            "Employee"
        }
        fn additional_setup(&mut self) {
            match utils::security::hash_str(self.password.as_str()) {
                Ok(hash) => self.password = hash,
                Err(e) => eprintln!("{}", e),
            }
        }
    }

    impl fmt::Display for Employee {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "Employee Information:\n\
                 ID: {}\n\
                 Name: {}\n\
                 Department: {}\n\
                 Boss ID: {}\n\
                 Project: {}\n\
                 Subject: {}\n\
                 Allocated Budget: {}\n\
                 Permission Level: {}\n\
                 Password: {}",
                self.id,
                self.name,
                self.department,
                self.boss_id,
                self.project,
                self.subject,
                self.alloc_budget,
                self.perm_level,
                self.password
            )
        }
    }

    impl DisplayInfo for Media {
        fn get_id(&self) -> u16 {
            self.id
        }
        fn get_name(&self) -> &str {
            self.name.as_str()
        }
        fn get_table_name() -> &'static str {
            "Media"
        }
        fn additional_setup(&mut self) {
            // TODO - Nothing yet
        }
    }

    impl fmt::Display for Media {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "Media Information:\n\
                 ID: {}\n\
                 Media Type: {}\n\
                 Name: {}\n\
                 Borrowable: {}\n\
                 Vendor: {}\n\
                 Renter: {}",
                self.id,
                self.media_type,
                self.name,
                if self.borrowable { "Yes" } else { "No" },
                self.vendor,
                self.renter
            )
        }
    }

    // Constructors and Getters/Setters
    // ---------------------------------------------------------------
    #[allow(clippy::too_many_arguments)]
    impl Employee {
        pub fn new(
            id: u16,
            name: String,
            department: String,
            boss_id: u16,
            project: String,
            subject: String,
            alloc_budget: u16,
            perm_level: PermissionLevel,
            password: String,
        ) -> Result<Self, &'static str> {
            if password.len() < 8 {
                return Err("Password should be at least 8 characters long.");
            }
            match utils::security::hash_str(password.as_str()) {
                Ok(hash) => Ok(Employee {
                    id,
                    name,
                    department,
                    boss_id,
                    project,
                    subject,
                    alloc_budget,
                    perm_level,
                    password: hash,
                }),
                Err(_) => Err("Error occurred while hashing password"),
            }
        }
        pub fn department(&self) -> &str {
            &self.department
        }
        pub fn boss_id(&self) -> u16 {
            self.boss_id
        }
        pub fn project(&self) -> &str {
            &self.project
        }
        pub fn subject(&self) -> &str {
            &self.subject
        }
        pub fn alloc_budget(&self) -> u16 {
            self.alloc_budget
        }
        pub fn perm_level(&self) -> &PermissionLevel {
            &self.perm_level
        }
        pub fn password(&self) -> &str {
            &self.password
        }
        pub fn set_id(&mut self, id: u16) {
            self.id = id;
        }
        pub fn set_name(&mut self, name: String) {
            self.name = name;
        }
        pub fn set_department(&mut self, department: String) {
            self.department = department;
        }
        pub fn set_boss_id(&mut self, boss_id: u16) {
            self.boss_id = boss_id;
        }
        pub fn set_project(&mut self, project: String) {
            self.project = project;
        }
        pub fn set_subject(&mut self, subject: String) {
            self.subject = subject;
        }
        pub fn set_alloc_budget(&mut self, alloc_budget: u16) {
            self.alloc_budget = alloc_budget;
        }
        pub fn set_perm_level(&mut self, perm_level: PermissionLevel) {
            self.perm_level = perm_level;
        }
        pub fn set_password(&mut self, password: String) {
            match utils::security::hash_str(password.as_str()) {
                Ok(hash) => self.password = hash,
                Err(e) => eprintln!("{}", e),
            }
        }
    }

    impl Media {
        pub fn new(
            id: u16,
            media_type: MediaType,
            name: String,
            borrowable: bool,
            vendor: String,
            renter: String,
        ) -> Media {
            Media {
                id,
                media_type,
                name,
                borrowable,
                vendor,
                renter,
            }
        }
        pub fn media_type(&self) -> &MediaType {
            &self.media_type
        }
        pub fn borrowable(&self) -> bool {
            self.borrowable
        }
        pub fn vendor(&self) -> &str {
            &self.vendor
        }
        pub fn renter(&self) -> &str {
            &self.renter
        }
        pub fn set_id(&mut self, id: u16) {
            self.id = id;
        }
        pub fn set_media_type(&mut self, media_type: MediaType) {
            self.media_type = media_type;
        }
        pub fn set_name(&mut self, name: String) {
            self.name = name;
        }
        pub fn set_borrowable(&mut self, borrowable: bool) {
            self.borrowable = borrowable;
        }
        pub fn set_vendor(&mut self, vendor: String) {
            self.vendor = vendor;
        }
        pub fn set_renter(&mut self, renter: String) {
            self.renter = renter;
        }
    }
}

pub mod enums {
    use std::cmp::Ordering;
    use std::fmt;

    #[derive(serde::Deserialize, serde::Serialize, Debug, Default)]
    pub enum PermissionLevel {
        Basic,
        User,
        Manager,
        Admin,
        Dev,
        #[default]
        None,
    }

    #[derive(serde::Deserialize, serde::Serialize, Debug, Default)]
    pub enum MediaType {
        Book,
        VideoGame,
        Movie,
        Music,
        #[default]
        None,
    }

    impl PartialEq for PermissionLevel {
        fn eq(&self, other: &Self) -> bool {
            self.to_ordinal() == other.to_ordinal()
        }
    }

    impl PartialOrd for PermissionLevel {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.to_ordinal().cmp(&other.to_ordinal()))
        }
    }

    impl PermissionLevel {
        fn to_ordinal(&self) -> u8 {
            match self {
                PermissionLevel::None => 0,
                PermissionLevel::Basic => 1,
                PermissionLevel::User => 2,
                PermissionLevel::Manager => 3,
                PermissionLevel::Admin => 4,
                PermissionLevel::Dev => 5,
            }
        }
    }

    impl fmt::Display for PermissionLevel {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                PermissionLevel::Basic => write!(f, "Basic"),
                PermissionLevel::User => write!(f, "User"),
                PermissionLevel::Manager => write!(f, "Manager"),
                PermissionLevel::Admin => write!(f, "Admin"),
                PermissionLevel::Dev => write!(f, "Dev"),
                PermissionLevel::None => write!(f, "None"),
            }
        }
    }

    impl fmt::Display for MediaType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                MediaType::Book => write!(f, "Book"),
                MediaType::VideoGame => write!(f, "Video Game"),
                MediaType::Movie => write!(f, "Movie"),
                MediaType::Music => write!(f, "Music"),
                MediaType::None => write!(f, "None"),
            }
        }
    }
}

// Tests
// ---------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::enums::PermissionLevel;

    #[test]
    fn create_employee() {
        let test_employee = structs::Employee::new(
            10,
            String::from("John Doe"),
            String::from("IT"),
            1,
            String::from("Ambitious Project"),
            String::from("Computer Science"),
            1_000,
            PermissionLevel::Manager,
            String::from("password"),
        );
        assert!(test_employee.is_ok())
    }

    #[test]
    fn create_bad_employee() {
        let test_employee = structs::Employee::new(
            10,
            String::from("John Doe"),
            String::from("IT"),
            1,
            String::from("Ambitious Project"),
            String::from("Computer Science"),
            1_000,
            PermissionLevel::Manager,
            String::from("pass"),
        );
        assert!(test_employee.is_err())
    }
}
