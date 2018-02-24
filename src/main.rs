mod args;
mod config;
mod ignore;
mod remote_command;
mod sync;
mod time;

use args::Args;
use config::Config;
use ignore::*;
use remote_command::execute_remote_command as execute_remote_command_impl;
use std::env;
use std::path::PathBuf;
use std::process;
use std::time::Instant;
use sync::*;
use time::*;

fn main() {
    print_header();

    let args = match Args::parse(env::args().collect()) {
        Err(message) => exit_with_error(&message, 1),
        Ok(value) => value,
    };

    let working_dir = match env::current_dir() {
        Err(_) => exit_with_error(&"Could not resolve working directory, make sure it exists and user has enough permissions to work with it.", 1),
        Ok(value) => value
    };

    let mut config_file = working_dir.clone();
    config_file.push(".mainframer/config");

    let config = match Config::from_file(config_file.as_path()) {
        Err(message) => exit_with_error(&message, 1),
        Ok(value) => value
    };

    let ignore = Ignore::from_working_dir(&working_dir);

    let start = Instant::now();

    match sync_before_remote_command(&working_dir, &config, &ignore) {
        Err(error) => exit_with_error(&format!("Sync local → remote machine failed: {}", error), 1),
        Ok(_) => ()
    }

    let remote_command_result = execute_remote_command(&working_dir, &args, &config);

    match sync_after_remote_command(&working_dir, &config, &ignore) {
        Err(error) => exit_with_error(&format!("Sync remote → local machine failed: {}", error), 1),
        Ok(_) => ()
    }

    let duration = start.elapsed();

    match remote_command_result {
        Err(_) => exit_with_error(&format!("\nFailure: took {}", format_duration(&duration)), 1),
        _ => println!("\nSuccess: took {}", format_duration(&duration))
    }
}

fn print_header() {
    println!(":: Mainframer v2.1.0\n");
}

fn exit_with_error(message: &str, code: i32) -> ! {
    if !message.is_empty() {
        eprintln!("{}", message);
    }
    process::exit(code);
}

fn sync_before_remote_command(working_dir: &PathBuf, config: &Config, ignore: &Ignore) -> Result<(), String> {
    println!("Sync local → remote machine...");

    let start = Instant::now();

    let result = sync_local_to_remote(
        &working_dir.file_name().unwrap().to_string_lossy().clone(),
        config,
        ignore
    );

    let duration = start.elapsed();

    match result {
        Err(error) => Err(error),
        Ok(_) => {
            println!("Sync done: took {}\n", format_duration(&duration));
            Ok(())
        }
    }
}

fn execute_remote_command(working_dir: &PathBuf, args: &Args, config: &Config) -> Result<(), ()> {
    println!("Executing command on remote machine...\n");

    let start = Instant::now();

    let result = execute_remote_command_impl(
        &args.command.clone(),
        config,
        &format!("~/mainframer/{}", working_dir.file_name().unwrap().to_string_lossy().clone())
    );

    let duration = start.elapsed();

    match result {
        Err(_) => eprintln!("\nExecution failed: took {}\n", format_duration(&duration)),
        Ok(_) => println!("\nExecution done: took {}\n", format_duration(&duration))
    }

    result
}

fn sync_after_remote_command(working_dir: &PathBuf, config: &Config, ignore: &Ignore) -> Result<(), String> {
    println!("Sync remote → local machine...");

    let start = Instant::now();

    let result = sync_remote_to_local(
        &working_dir.file_name().unwrap().to_string_lossy().clone(),
        config,
        ignore
    );

    let duration = start.elapsed();

    match result {
        Err(error) => Err(error),
        Ok(_) => {
            println!("Sync done: took {}", format_duration(&duration));
            Ok(())
        }
    }
}