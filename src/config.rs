use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    pub remote_machine_name: String,
    pub local_compression_level: u8,
    pub remote_compression_level: u8
}

impl Config {
    pub fn from_file(file_path: &Path) -> Result<Config, String> {
        let mut content = String::new();

        let mut file = match File::open(file_path) {
            Err(_) => return Err(format!("Could not open config file '{}'.", file_path.to_string_lossy())),
            Ok(value) => value,
        };

        file.read_to_string(&mut content)
            .expect(&format!("Could not read config file '{}'.", file_path.to_string_lossy()));

        match parse_config_from_str(&content) {
            Err(message) => Err(format!("Error during parsing config file '{}'\n{}", file_path.to_string_lossy(), message)),
            Ok(config) => Ok(config)
        }
    }
}

// Parses config content as https://en.wikipedia.org/wiki/.properties
fn find_value(config_content: &str, key: &str) -> Option<String> {
    match config_content.find(&format!("{}=", key)) {
        None => None,
        Some(startIndex) => {
            let content_starting_with_key = &config_content[startIndex..config_content.len()];
            let value_start_index = match content_starting_with_key.find("=") {
                None => return None,
                Some(index) => index
            };

            let line_end_index = match content_starting_with_key.find("\n") {
                None => content_starting_with_key.len(),
                Some(index) => index
            };

            let value = (&content_starting_with_key[value_start_index + 1..line_end_index]).trim();

            match value.len() {
                0 => None,
                _ => Some(String::from(value))
            }
        }
    }
}

fn parse_config_from_str(config_content: &str) -> Result<Config, String> {
    Ok(Config {
        remote_machine_name: match find_value(&config_content, &"remote_machine") {
            None => return Err(format!("please specify 'remote_machine'.")),
            Some(value) => value
        },
        local_compression_level: match find_value(&config_content, &"local_compression_level") {
            None => 1,
            Some(value) => match value.parse() {
                Err(_) => return Err(format!("'local_compression_level' must be a positive number, found '{}'.", value)),
                Ok(value) => value
            }
        },
        remote_compression_level: match find_value(&config_content, &"remote_compression_level") {
            None => 1,
            Some(value) => match value.parse() {
                Err(_) => return Err(format!("'remote_compression_level' must be a positive number, found '{}'.", value)),
                Ok(value) => value
            }
        }
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn find_value_no_new_line_at_the_end() {
        let content = "key=value";
        assert_eq!(find_value(content, "key"), Some(String::from("value")));
    }

    #[test]
    fn find_value_new_line_at_the_end() {
        let content = "key=value\n";
        assert_eq!(find_value(content, "key"), Some(String::from("value")));
    }

    #[test]
    fn find_value_trims_value() {
        let content = "key=value  ";
        assert_eq!(find_value(content, "key"), Some(String::from("value")));
    }

    #[test]
    fn find_value_no_value_after_equals_sign() {
        let content = "key=";
        assert_eq!(find_value(content, "key"), None);
    }

    #[test]
    fn find_value_whitespace_instead_of_value() {
        let content = "key= ";
        assert_eq!(find_value(content, "key"), None);
    }

    #[test]
    fn parse_config_from_str_all_fields() {
        let content = "remote_machine=test@machine\nlocal_compression_level=2\nremote_compression_level=3";
        assert_eq!(parse_config_from_str(content), Ok(Config {
            remote_machine_name: String::from("test@machine"),
            local_compression_level: 2,
            remote_compression_level: 3
        }));
    }

    #[test]
    fn parse_config_from_str_all_fields_unordered() {
        let content = "remote_compression_level=3\nremote_machine=test@machine\nlocal_compression_level=2";
        assert_eq!(parse_config_from_str(content), Ok(Config {
            remote_machine_name: String::from("test@machine"),
            local_compression_level: 2,
            remote_compression_level: 3
        }));
    }

    #[test]
    fn parse_config_from_str_only_remote_machine_name() {
        let content = "remote_machine=test@machine";
        assert_eq!(parse_config_from_str(content), Ok(Config {
            remote_machine_name: String::from("test@machine"),
            local_compression_level: 1, // Default value.
            remote_compression_level: 1, // Default value.
        }));
    }

    #[test]
    fn parse_config_from_str_no_remote_machine_name() {
        let content = "local_compression_level=2\nremote_compression_level=3";
        assert_eq!(parse_config_from_str(content), Err(String::from("please specify 'remote_machine'.")));
    }

    #[test]
    fn parse_config_from_str_local_compression_level_not_a_number() {
        let content = "remote_machine=test@machine\nlocal_compression_level=yooo";
        assert_eq!(parse_config_from_str(content), Err(String::from("'local_compression_level' must be a positive number, found 'yooo'.")));
    }

    #[test]
    fn parse_config_from_str_remote_compression_level_not_a_number() {
        let content = "remote_machine=test@machine\nremote_compression_level=wut";
        assert_eq!(parse_config_from_str(content), Err(String::from("'remote_compression_level' must be a positive number, found 'wut'.")));
    }
}