pub mod structs {

    use crate::types::enums::MediaType;
    use std::fmt;

    // Struct Definitions
    // ---------------------------------------------------------------

    #[derive(serde::Deserialize, serde::Serialize, Debug)]
    pub struct Employee {
        id: u16,
        name: String,
        department: String,
        boss_id: u16,
        project: String,
        subject: String,
        alloc_budget: u16,
        perm_level: u16,
        password: String,
    }

    #[derive(serde::Deserialize, serde::Serialize, Debug)]
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

    pub trait DisplayInfo {
        fn get_id(&self) -> u16;
        fn get_name(&self) -> &str;
        fn get_table_name() -> &'static str;
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
    impl Employee {
        pub fn new(
            id: u16,
            name: String,
            department: String,
            boss_id: u16,
            project: String,
            subject: String,
            alloc_budget: u16,
            perm_level: u16,
            password: String,
        ) -> Result<Self, &'static str> {
            if password.len() < 8 {
                return Err("Password should be at least 8 characters long.");
            }
            Ok(Employee {
                id,
                name,
                department,
                boss_id,
                project,
                subject,
                alloc_budget,
                perm_level,
                password,
            })
        }
        pub fn id(&self) -> u16 {
            self.id
        }
        pub fn name(&self) -> &str {
            &self.name
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
        pub fn perm_level(&self) -> u16 {
            self.perm_level
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
        pub fn set_perm_level(&mut self, perm_level: u16) {
            self.perm_level = perm_level;
        }
        pub fn set_password(&mut self, password: String) {
            self.password = password;
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
        pub fn id(&self) -> u16 {
            self.id
        }
        pub fn media_type(&self) -> &MediaType {
            &self.media_type
        }
        pub fn name(&self) -> &str {
            &self.name
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
    use std::fmt;

    #[derive(serde::Deserialize, serde::Serialize, Debug)]
    pub enum MediaType {
        Book,
        VideoGame,
        Movie,
        Music,
    }

    impl fmt::Display for MediaType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                MediaType::Book => write!(f, "Book"),
                MediaType::VideoGame => write!(f, "Video Game"),
                MediaType::Movie => write!(f, "Movie"),
                MediaType::Music => write!(f, "Music"),
            }
        }
    }
}

// Tests
// ---------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

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
            1,
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
            1,
            String::from("pass"),
        );
        assert!(test_employee.is_err())
    }
}
