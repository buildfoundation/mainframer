mod args;
mod config;
mod ignore;
mod sync;
mod remote_command;

use args::Args;
use config::Config;
use ignore::*;
use remote_command::execute_remote_command as execute_remote_command_impl;
use std::env;
use std::path::PathBuf;
use std::process;
use sync::*;

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

    sync_before_remote_command(&working_dir, &config, &ignore);
    execute_remote_command(&working_dir, &args, &config);
    sync_after_remote_command(&working_dir, &config, &ignore);
}

fn print_header() {
    println!(":: Mainframer v2.1.0\n");
}

fn exit_with_error(message: &str, code: i32) -> ! {
    eprintln!("Error: {}", message);
    process::exit(code);
}

fn sync_before_remote_command(working_dir: &PathBuf, config: &Config, ignore: &Ignore) {
    sync_local_to_remote(&working_dir.file_name().unwrap().to_string_lossy().clone(), config, ignore);
}

fn execute_remote_command(working_dir: &PathBuf, args: &Args, config: &Config) {
    execute_remote_command_impl(
        &args.command.clone(),
        config,
        &format!("~/mainframer/{}", working_dir.file_name().unwrap().to_string_lossy().clone())
    );
}

fn sync_after_remote_command(working_dir: &PathBuf, config: &Config, ignore: &Ignore) {
    sync_remote_to_local(&working_dir.file_name().unwrap().to_string_lossy().clone(), config, ignore);
}