pub mod loading {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    pub fn load_db_settings() -> Vec<String> {
        let settings: [&str; 2] = ["endpoint", "password"];
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
}
