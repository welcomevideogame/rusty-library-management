pub mod loading {
    use crate::types::structs::DisplayInfo;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    pub fn load_db_settings() -> Vec<String> {
        let settings: [&str; 3] = ["endpoint", "api_key", "salt"];
        load_settings(&settings)
    }

    pub fn load_hashing_salt() -> Vec<String> {
        let settings: [&str; 1] = ["hash_salt"];
        load_settings(&settings)
    }

    fn load_config_contents() -> String {
        let mut contents = String::new();
        BufReader::new(File::open("./resources/config.ini").expect("Config file does not exist"))
            .read_to_string(&mut contents)
            .expect("Config file is corrupt");
        contents
    }

    pub fn load_setting(section: &str, setting: &str) -> Option<String> {
        let contents = load_config_contents();
        let mut config = configparser::ini::Ini::new();
        config.read(contents).expect("Invalid config file!");
        config.get(section, setting)
    }

    fn load_settings(settings: &[&str]) -> Vec<String> {
        let mut res_settings = Vec::new();
        let contents = load_config_contents();
        let mut config = configparser::ini::Ini::new();
        config.read(contents).expect("Invalid config file!");

        for setting in settings {
            res_settings.push(config.get("DBSettings", setting).unwrap());
        }
        res_settings
    }

    pub fn vec_to_hashmap<T: DisplayInfo>(obj_vec: Vec<T>) -> HashMap<u16, T> {
        obj_vec.into_iter().map(|obj| (obj.get_id(), obj)).collect()
    }
}

pub mod network {
    pub async fn test_network(url: &str) -> bool {
        reqwest::get(url).await.is_ok()
    }
}

pub mod user {
    use std::io;

    pub fn get_input() -> String {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .ok()
            .unwrap_or_else(|| String::new().parse().unwrap());
        buffer.trim().to_string()
    }
}

pub mod security {
    use sha2::{Digest, Sha256};
    use std::borrow::Cow;

    pub fn hash_str(s: &str, salt: Option<&str>) -> String {
        let mut hasher = Sha256::new();
        let s: Cow<str> = match salt {
            Some(chars) => format!("{}{}", chars, s).into(),
            None => s.into(),
        };
        hasher.update(s.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

// Tests
// ---------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::utils::security;

    #[test]
    fn test_hashing() {
        let test_string = "hello world";
        let hash_string = security::hash_str(test_string, None);
        assert_eq!(
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9",
            hash_string
        )
    }

    #[test]
    fn test_hashing_salt() {
        let salt_string = "salt";
        let test_string = "hello world";
        let hash_string = security::hash_str(test_string, Some(salt_string));
        assert_ne!(
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9",
            hash_string
        )
    }
}
