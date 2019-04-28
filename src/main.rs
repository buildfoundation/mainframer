extern crate crossbeam_channel;

use std::env;
use std::fs;
use std::path::Path;
use std::process;
use std::time::Instant;

use args::Args;
use config::*;
use ignore::*;
use intermediate_config::IntermediateConfig;
use sync::PullMode;
use time::*;

mod args;
mod config;
mod intermediate_config;
mod ignore;
mod remote_command;
mod sync;
mod time;

// TODO use Reactive Streams instead of Channels.

fn main() {
    let total_start = Instant::now();

    println!(":: Mainframer v{}\n", env!("CARGO_PKG_VERSION"));
    let raw_args: Vec<String> = env::args().skip(1).collect();

    let args = match Args::parse(raw_args.as_ref()) {
        Err(message) => exit_with_error(&message, 1),
        Ok(value) => value,
    };

    let local_dir_absolute_path = match env::current_dir() {
        Err(_) => exit_with_error("Could not resolve working directory, make sure it exists and user has enough permissions to work with it.", 1),
        Ok(value) => fs::canonicalize(value).unwrap()
    };

    let mut config_file = local_dir_absolute_path.clone();
    config_file.push(".mainframer/config.yml");

    let config = match merge_configs(&config_file) {
        Err(error) => exit_with_error(&error, 1),
        Ok(value) => value
    };

    let ignore = Ignore::from_working_dir(&local_dir_absolute_path.clone());

    if let Err(error) = sync_before_remote_command(&local_dir_absolute_path, &config, &ignore) {
        exit_with_error(&format!("Sync local → remote machine failed: {}.", error), 1)
    }

    println!("Executing command on remote machine...\n");

    let remote_command_start_time = Instant::now();

    let remote_command_finished_rx = remote_command::execute_remote_command(
        args.command.clone(),
        config.clone(),
        sync::project_dir_on_remote_machine(&local_dir_absolute_path.clone()),
    );

    let remote_to_local_sync_finished_rx = sync::sync_remote_to_local(&local_dir_absolute_path, config.clone(), ignore, &config.pull.mode, remote_command_finished_rx.clone());

    let remote_command_result = remote_command_finished_rx.recv();
    let remote_command_duration = remote_command_start_time.elapsed(); // TODO: move duration to Result.

    match remote_command_result {
        Err(_) => eprintln!("\nExecution failed: took {}.\n", format_duration(remote_command_duration)),
        Ok(_) => println!("\nExecution done: took {}.\n", format_duration(remote_command_duration))
    }

    let remote_to_local_sync_result = remote_to_local_sync_finished_rx.recv();
    let total_duration = total_start.elapsed();

    if remote_command_result.is_ok() && remote_to_local_sync_result.is_ok() {
        println!("\nSuccess: took {}.", format_duration(total_duration));
    } else {
        exit_with_error(&format!("\nFailure: took {}.", format_duration(total_duration)), 1);
    }
}

fn exit_with_error(message: &str, code: i32) -> ! {
    if !message.is_empty() {
        eprintln!("{}", message);
    }
    process::exit(code);
}

fn merge_configs(project_config_file: &Path) -> Result<Config, String> {
    let default_push_compression = 3;
    let default_pull_compression = 1;
    let default_pull_mode = PullMode::Serial; // TODO: consider making Parallel the default mode.

    Ok(match IntermediateConfig::from_file(project_config_file) {
        Err(message) => return Err(message),
        Ok(intermediate_config) => {
            let remote = match intermediate_config.remote {
                None => return Err(String::from("Configuration must specify 'remote'")),
                Some(value) => value
            };

            Config {
                remote: Remote {
                    host: match remote.host {
                        None => return Err(String::from("Configuration must specify 'remote.host'")),
                        Some(value) => value
                    },
                },
                push: match intermediate_config.push {
                    None => Push {
                        compression: default_push_compression,
                    },
                    Some(push) => Push {
                        compression: push.compression.unwrap_or(default_push_compression),
                    }
                },
                pull: match intermediate_config.pull {
                    None => Pull {
                        compression: default_pull_compression,
                        mode: default_pull_mode,
                    },
                    Some(pull) => Pull {
                        compression: pull.compression.unwrap_or(default_pull_compression),
                        mode: pull.mode.unwrap_or(default_pull_mode)
                    }
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
