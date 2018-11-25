mod args;
mod config;
mod intermediate_config;
mod ignore;
mod remote_command;
mod sync;
mod time;

use args::Args;
use config::*;
use intermediate_config::IntermediateConfig;
use ignore::*;
use std::env;
use std::fs;
use std::path::Path;
use std::process;
use std::time::Instant;
use time::*;

fn main() {
    println!(":: Mainframer v{}\n", env!("CARGO_PKG_VERSION"));
    let raw_args: Vec<String> = env::args().skip(1).collect();

    let args = match Args::parse(raw_args.as_ref()) {
        Err(message) => exit_with_error(&message, 1),
        Ok(value) => value,
    };

    let local_dir_absolute_path = match env::current_dir() {
        Err(_) => exit_with_error(&"Could not resolve working directory, make sure it exists and user has enough permissions to work with it.", 1),
        Ok(value) => fs::canonicalize(value).unwrap()
    };

    let mut config_file = local_dir_absolute_path.to_owned();
    config_file.push(".mainframer/config.yml");

    let config = match merge_configs(&config_file) {
        Err(error) => exit_with_error(&error, 1),
        Ok(value) => value
    };

    let ignore = Ignore::from_working_dir(&local_dir_absolute_path);

    let start = Instant::now();

    if let Err(error) = sync_before_remote_command(&local_dir_absolute_path, &config, &ignore) {
        exit_with_error(&format!("Sync local → remote machine failed: {}.", error), 1)
    }

    let remote_command_result = execute_remote_command(&local_dir_absolute_path, &args, &config);

    if let Err(error) = sync_after_remote_command(&local_dir_absolute_path, &config, &ignore) {
        exit_with_error(&format!("Sync remote → local machine failed: {}.", error), 1)
    }

    let duration = start.elapsed();

    match remote_command_result {
        Err(_) => exit_with_error(&format!("\nFailure: took {}.", format_duration(duration)), 1),
        _ => println!("\nSuccess: took {}.", format_duration(duration))
    }
}

fn exit_with_error(message: &str, code: i32) -> ! {
    if !message.is_empty() {
        eprintln!("{}", message);
    }
    process::exit(code);
}

fn merge_configs(project_config_file: &Path) -> Result<Config, String> {
    let default_local_compression = 3;
    let default_remote_compression = 1;

    Ok(match IntermediateConfig::from_file(project_config_file) {
        Err(message) => return Err(message),
        Ok(intermediate_config) => {
            let remote_machine = match intermediate_config.remote_machine {
                None => return Err(String::from("Configuration must specify remoteMachine")),
                Some(value) => value
            };

            Config {
                remote_machine: RemoteMachine {
                    host: match remote_machine.host {
                        None => return Err(String::from("Configuration must specify remoteMachine.host")),
                        Some(value) => value
                    },
                    user: remote_machine.user,
                },
                compression: match intermediate_config.compression {
                    None => Compression {
                        local: default_local_compression,
                        remote: default_remote_compression,
                    },
                    Some(compression) => Compression {
                        local: compression.local.unwrap_or(default_local_compression),
                        remote: compression.remote.unwrap_or(default_remote_compression),
                    },
                },
            }
        }
    })
}

fn sync_before_remote_command(local_dir_absolute_path: &Path, config: &Config, ignore: &Ignore) -> Result<(), String> {
    println!("Sync local → remote machine...");

    let start = Instant::now();

    let result = sync::sync_local_to_remote(
        &local_dir_absolute_path,
        config,
        ignore,
    );

    let duration = start.elapsed();

    match result {
        Err(error) => Err(error),
        Ok(_) => {
            println!("Sync done: took {}.\n", format_duration(duration));
            Ok(())
        }
    }
}

fn execute_remote_command(local_dir_absolute_path: &Path, args: &Args, config: &Config) -> Result<(), ()> {
    println!("Executing command on remote machine...\n");

    let start = Instant::now();

    let result = remote_command::execute_remote_command(
        &args.command.clone(),
        config,
        sync::project_dir_on_remote_machine(local_dir_absolute_path).as_ref(),
    );

    let duration = start.elapsed();

    match result {
        Err(_) => eprintln!("\nExecution failed: took {}.\n", format_duration(duration)),
        Ok(_) => println!("\nExecution done: took {}.\n", format_duration(duration))
    }

    result
}

fn sync_after_remote_command(working_dir_name: &Path, config: &Config, ignore: &Ignore) -> Result<(), String> {
    println!("Sync remote → local machine...");

    let start = Instant::now();

    let result = sync::sync_remote_to_local(
        working_dir_name,
        config,
        ignore,
    );

    let duration = start.elapsed();

    match result {
        Err(error) => Err(error),
        Ok(_) => {
            println!("Sync done: took {}.", format_duration(duration));
            Ok(())
        }
    }
}
