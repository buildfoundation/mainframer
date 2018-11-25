extern crate yaml_rust;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use self::yaml_rust::Yaml;
use self::yaml_rust::YamlLoader;

#[derive(Debug, PartialEq, Eq)]
pub struct IntermediateConfig {
    pub remote_machine: Option<IntermediateRemoteMachine>,
    pub compression: Option<IntermediateCompression>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct IntermediateRemoteMachine {
    pub host: Option<String>,
    pub user: Option<String>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct IntermediateCompression {
    pub local: Option<i64>,
    pub remote: Option<i64>,
}

impl IntermediateConfig {
    pub fn from_file(file_path: &Path) -> Result<IntermediateConfig, String> {
        let mut content = String::new();

        let mut file = match File::open(file_path) {
            Err(_) => return Err(format!("Could not open config file '{}'.", file_path.to_string_lossy())),
            Ok(value) => value,
        };

        file.read_to_string(&mut content)
            .unwrap_or_else(|_| panic!("Could not read config file '{}'.", file_path.to_string_lossy()));

        match parse_config_from_str(&content) {
            Err(message) => Err(format!("Error during parsing config file '{}'\n{}", file_path.to_string_lossy(), message)),
            Ok(config) => Ok(config)
        }
    }
}

fn parse_config_from_str(config_content: &str) -> Result<IntermediateConfig, String> {
    let yaml = match YamlLoader::load_from_str(config_content) {
        Err(error) => return Err(format!("Error during parsing of config.yml: {:#?}", error)),
        Ok(content) => content[0].to_owned()
    };

    println!("Yaml: {:#?}", yaml);

    let remote_machine = match &yaml["remoteMachine"] {
        Yaml::Hash(remote_machine) => {
            let host = match &remote_machine.get(&Yaml::String(String::from("host"))) {
                Some(host) => match host {
                    Yaml::String(host) => Some(host.to_string()),
                    Yaml::Null => None,
                    _ => return Err(String::from("Error during parsing of config.yml, remoteMachine.host must be a string."))
                },
                None => None
            };

            let user = match &remote_machine.get(&Yaml::String(String::from("user"))) {
                Some(user) => match user {
                    Yaml::String(user) => Some(user.to_string()),
                    Yaml::Null => None,
                    _ => return Err(String::from("Error during parsing of config.yml, remoteMachine.user must be a string."))
                },
                None => None
            };

            Some(IntermediateRemoteMachine {
                host,
                user,
            })
        }
        Yaml::Null | Yaml::BadValue => None,
        ref something_else => return Err(format!("'remoteMachine' must be an object, but was {:#?}", something_else))
    };


    let compression = match &yaml["compression"] {
        Yaml::Hash(compression) => {
            let local = match compression.get(&Yaml::String(String::from("local"))) {
                Some(local) => match local {
                    Yaml::Integer(local) => if local >= &1 && local <= &9 {
                        Some(local.to_owned())
                    } else {
                        return Err(format!("'compression.local' must be a positive integer from 1 to 9, but was {:#?}", local));
                    },
                    Yaml::Null => None,
                    ref something_else => return Err(format!("'compression.local' must be a positive integer from 1 to 9, but was {:#?}", something_else))
                },
                None => None
            };

            let remote = match compression.get(&Yaml::String(String::from("remote"))) {
                Some(remote) => match remote {
                    Yaml::Integer(remote) => if remote >= &1 && remote <= &9 {
                        Some(remote.to_owned())
                    } else {
                        return Err(format!("'compression.remote' must be a positive integer from 1 to 9, but was {:#?}", remote));
                    },
                    Yaml::Null => None,
                    ref something_else => return Err(format!("'compression.remote' must be a positive integer from 1 to 9, but was {:#?}", something_else))
                },
                None => None
            };

            Some(IntermediateCompression {
                local,
                remote,
            })
        }
        Yaml::Null | Yaml::BadValue => None,
        _ => return Err(String::from("'compression' must be an object."))
    };

    Ok(IntermediateConfig {
        remote_machine,
        compression,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_config_from_str_all_fields() {
        let content = "
remoteMachine:
  host: computer1
  user: user1
compression:
  local: 5
  remote: 2"
        ;
        assert_eq!(parse_config_from_str(content), Ok(IntermediateConfig {
            remote_machine: Some(IntermediateRemoteMachine {
                host: Some(String::from("computer1")),
                user: Some(String::from("user1")),
            }),
            compression: Some(IntermediateCompression {
                local: Some(5),
                remote: Some(2),
            }),
        }));
    }

    #[test]
    fn parse_config_from_str_only_remote_machine_host() {
        let content = "
remoteMachine:
  host: computer1
";
        assert_eq!(parse_config_from_str(content), Ok(IntermediateConfig {
            remote_machine: Some(IntermediateRemoteMachine {
                host: Some(String::from("computer1")),
                user: None,
            }),
            compression: None,
        }));
    }

    #[test]
    fn parse_config_from_str_only_remote_machine_user() {
        let content = "
remoteMachine:
  user: user1
";
        assert_eq!(parse_config_from_str(content), Ok(IntermediateConfig {
            remote_machine: Some(IntermediateRemoteMachine {
                host: None,
                user: Some(String::from("user1")),
            }),
            compression: None,
        }));
    }

    #[test]
    fn parse_config_from_str_only_compression_local() {
        let content = "
compression:
  local: 5
";
        assert_eq!(parse_config_from_str(content), Ok(IntermediateConfig {
            remote_machine: None,
            compression: Some(IntermediateCompression {
                local: Some(5),
                remote: None,
            }),
        }));
    }

    #[test]
    fn parse_config_from_str_only_compression_remote() {
        let content = "
compression:
  remote: 2
";
        assert_eq!(parse_config_from_str(content), Ok(IntermediateConfig {
            remote_machine: None,
            compression: Some(IntermediateCompression {
                local: None,
                remote: Some(2),
            }),
        }));
    }

    #[test]
    fn parse_config_from_str_compression_local_not_an_integer() {
        let content = "
compression:
  local: yooo
";
        assert_eq!(parse_config_from_str(content), Err(String::from("'compression.local\' must be a positive integer from 1 to 9, but was String(\n    \"yooo\"\n)")));
    }

    #[test]
    fn parse_config_from_str_compression_remote_not_an_integer() {
        let content = "
compression:
  remote: yooo
";
        assert_eq!(parse_config_from_str(content), Err(String::from("'compression.remote\' must be a positive integer from 1 to 9, but was String(\n    \"yooo\"\n)")));
    }
}
