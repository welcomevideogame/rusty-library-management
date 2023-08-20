pub mod loading {
    use crate::types::structs::DisplayInfo;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    pub fn load_db_settings() -> Vec<String> {
        let settings: [&str; 3] = ["endpoint", "api_key", "salt"];
        let mut res_settings: Vec<String> = Vec::new();

        let mut contents = String::new();
        BufReader::new(
            File::open("./resources/config.ini").expect("There was a problem loading the file!"),
        )
        .read_to_string(&mut contents)
        .expect("Failed to read the file");

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
