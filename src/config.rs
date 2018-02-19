use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    remote_machine_name: String,
    local_compression_level: u8,
    remote_compression_level: u8
}

impl Config {
    pub fn from_file(file_path: &str) -> Result<Config, String> {
        let mut content = String::new();

        let mut file = match File::open(file_path) {
            Err(_) => return Err(format!("Could not open config file '{}'.", file_path)),
            Ok(value) => value,
        };

        file.read_to_string(&mut content)
            .expect(&format!("Could not read config file '{}'.", file_path));

        Ok(Config {
            remote_machine_name: String::from(""),
            local_compression_level: 1,
            remote_compression_level: 1
        })
    }
}

