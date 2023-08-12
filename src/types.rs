pub mod structs {

    use crate::types::enums::MediaType;

    #[derive(serde::Deserialize, Debug)]
    pub struct Employee {
        id: u8,
        name: String,
        department: String,
        boss_id: String,
        project: String,
        subject: String,
        alloc_budget: u16,
        perm_level: u8,
        password: String,
    }

    impl Employee {
        pub fn new(
            id: u8,
            name: String,
            department: String,
            boss_id: String,
            project: String,
            subject: String,
            alloc_budget: u16,
            perm_level: u8,
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
