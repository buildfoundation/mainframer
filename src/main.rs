mod args;
mod config;
mod ignore;
mod sync;
mod remote_command;

use args::Args;
use config::Config;
use ignore::*;
use std::env;
use std::process;
use sync::*;
use remote_command::*;

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
}

fn print_header() {
    println!(":: Mainframer v2.1.0\n");
}

fn exit_with_error(message: &str, code: i32) -> ! {
    eprintln!("Error: {}", message);
    process::exit(code);
}