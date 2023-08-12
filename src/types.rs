pub mod structs {

    use crate::types::enums::MediaType;
    use std::fmt;

    pub trait DisplayInfo {
        fn get_id(&self) -> u16;
        fn get_table_name() -> &'static str;
    }

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

    impl DisplayInfo for Employee {
        fn get_id(&self) -> u16 {
            self.id
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
    }

    impl DisplayInfo for Media {
        fn get_id(&self) -> u16 {
            self.id
        }

        fn get_table_name() -> &'static str {
            "Media"
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
