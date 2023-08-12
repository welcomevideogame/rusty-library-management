pub mod structs {

    use crate::types::enums::MediaType;

    #[derive(serde::Deserialize, serde::Serialize, Debug)]
    pub struct Employee {
        pub id: u16,
        name: String,
        department: String,
        boss_id: u16,
        project: String,
        subject: String,
        alloc_budget: u16,
        perm_level: u16,
        password: String,
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

    pub struct Media {
        media_type: MediaType,
        name: String,
        borrowable: bool,
        vendor: String,
        renter: String,
    }

    impl Media {
        pub fn new(
            media_type: MediaType,
            name: String,
            borrowable: bool,
            vendor: String,
            renter: String,
        ) -> Media {
            Media {
                media_type,
                name,
                borrowable,
                vendor,
                renter,
            }
        }
    }
}

pub mod enums {
    pub enum MediaType {
        Book,
        VideoGame,
        Movie,
        Music,
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
