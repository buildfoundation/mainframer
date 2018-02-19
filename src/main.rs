mod args;
mod config;
mod sync;
mod remote_command;

use args::Args;
use config::Config;
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

    let config = match Config::from_file("") {
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