pub mod loading {
    use crate::types::structs::DisplayInfo;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    use super::super::types::structs::Trie;

    pub fn load_db_settings() -> Result<Vec<String>, String> {
        let settings: [&str; 3] = ["endpoint", "api_key", "salt"];
        load_settings(&settings)
    }

    fn load_config_contents() -> String {
        let mut contents = String::new();
        BufReader::new(File::open("./resources/config.ini").expect("Config file does not exist"))
            .read_to_string(&mut contents)
            .expect("Config file is corrupt");
        contents
    }

    fn load_settings(settings: &[&str]) -> Result<Vec<String>, String> {
        let contents = load_config_contents(); // Ensure this function returns a Result or handles errors internally
        let mut config = configparser::ini::Ini::new();
        config.read(contents).map_err(|e| e.to_string())?;
    
        settings.iter()
            .map(|&setting| config.get("DBSettings", setting)
            .ok_or_else(|| format!("Setting not found: {}", setting))).collect()
    }

    pub fn vec_to_hashmap<T: DisplayInfo>(obj_vec: Vec<T>) -> HashMap<u16, T> {
        obj_vec.into_iter().map(|obj| (obj.get_id(), obj)).collect()
    }

    pub fn hashmap_to_vec<T: Clone>(map: &HashMap<u16, T>) -> Vec<T> {
        map.values().cloned().collect()
    }

    pub fn hashmap_to_trie<T: DisplayInfo>(obj_map: &HashMap<u16, T>) -> Trie {
        let mut trie = Trie::new();
        for (_, obj) in obj_map {
            trie.insert(obj.get_name().to_lowercase().to_string());
        }
        trie
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
    use argon2::{self, Config, ThreadMode, Variant, Version};
    use rand::RngCore;

    pub fn hash_str(s: &str) -> Result<String, argon2::Error> {
        let config = Config {
            variant: Variant::Argon2i,
            version: Version::Version13,
            mem_cost: 65536,
            time_cost: 10,
            lanes: 4,
            thread_mode: ThreadMode::Parallel,
            secret: &[],
            ad: &[],
            hash_length: 32,
        };

        // Generate a new random salt
        let mut salt = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut salt);

        let hash = argon2::hash_encoded(s.as_bytes(), &salt, &config)?;
        Ok(hash)
    }

    pub fn verify_password(hashed: &str, password: &str) -> Result<bool, argon2::Error> {
        argon2::verify_encoded(hashed, password.as_bytes())
    }
}

// Tests
// ---------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::utils::security::{hash_str, verify_password};

    #[test]
    fn test_hashing() {
        let test_string = "hello world";
        match hash_str(test_string) {
            Ok(hash) => {
                assert!(verify_password(hash.as_str(), test_string).unwrap());
            }
            Err(_) => panic!("Hashing failed"),
        }
    }

    #[test]
    fn test_hashing_fail() {
        let test_string = "hello world";
        match hash_str(test_string) {
            Ok(hash) => {
                assert!(
                    !verify_password(hash.as_str(), &test_string[..test_string.len() - 1]).unwrap()
                );
            }
            Err(_) => panic!("Hashing failed"),
        }
    }
}