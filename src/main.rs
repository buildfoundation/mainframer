extern crate bus;
extern crate crossbeam_channel;

use std::env;
use std::fs;
use std::process;
use std::time::Instant;

use args::Args;
use config::*;
use ignore::*;
use sync::PullMode;
use time::*;

mod args;
mod config;
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

    let config = match Config::from_path(&config_file) {
        Err(error) => exit_with_error(&error, 1),
        Ok(value) => value,
    };

    let ignore = Ignore::from_working_dir(&local_dir_absolute_path);

    println!("Pushing...");

    match sync::push(&local_dir_absolute_path, &config, &ignore) {
        Err(err) => exit_with_error(
            &format!(
                "Push failed: {}, took {}",
                err.message,
                format_duration(err.duration)
            ),
            1,
        ),
        Ok(ok) => println!("Push done: took {}.\n", format_duration(ok.duration)),
    }

    match config.pull.mode {
        PullMode::Serial => println!("Executing command on remote machine...\n"),
        PullMode::Parallel => {
            println!("Executing command on remote machine (pulling in parallel)...\n")
        }
    }

    let mut remote_command_readers = remote_command::execute_remote_command(
        args.command,
        config.clone(),
        sync::project_dir_on_remote_machine(&local_dir_absolute_path),
        2,
    );

    let pull_finished_rx = sync::pull(
        &local_dir_absolute_path,
        config.clone(),
        ignore,
        &config.pull.mode,
        remote_command_readers.pop().unwrap(),
    );

    let remote_command_result = remote_command_readers.pop().unwrap().recv().unwrap();

    match remote_command_result {
        Err(ref err) => eprintln!(
            "\nExecution failed: took {}.\nPulling...",
            format_duration(err.duration)
        ),
        Ok(ref ok) => println!(
            "\nExecution done: took {}.\nPulling...",
            format_duration(ok.duration)
        ),
    }

    let pull_result = pull_finished_rx
        .recv()
        .expect("Could not receive remote_to_local_sync_result");

    let total_duration = total_start.elapsed();

    match pull_result {
        Err(ref err) => eprintln!(
            "Pull failed: {}, took {}.",
            err.message,
            format_duration(err.duration)
        ),
        Ok(ref ok) => println!("Pull done: took {}", format_duration(ok.duration)),
    }

    if remote_command_result.is_err() || pull_result.is_err() {
        exit_with_error(
            &format!("\nFailure: took {}.", format_duration(total_duration)),
            1,
        );
    } else {
        println!("\nSuccess: took {}.", format_duration(total_duration));
    }
}

fn exit_with_error(message: &str, code: i32) -> ! {
    if !message.is_empty() {
        eprintln!("{}", message);
    }
    process::exit(code);
}
