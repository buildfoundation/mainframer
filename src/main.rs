mod args;
mod config;
mod ignore;
mod remote_command;
mod sync;
mod time;

use args::Args;
use config::Config;
use ignore::*;
use std::env;
use std::process;
use std::time::Instant;
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError::*;
use time::*;

fn main() {
    println!(":: Mainframer v{}\n", env!("CARGO_PKG_VERSION"));

    let args = match Args::parse(env::args().skip(1).collect()) {
        Err(message) => exit_with_error(&message, 1),
        Ok(value) => value,
    };

    let working_dir = match env::current_dir() {
        Err(_) => exit_with_error(&"Could not resolve working directory, make sure it exists and user has enough permissions to work with it.", 1),
        Ok(value) => value
    };

    let working_dir_name = working_dir.file_name().unwrap().to_string_lossy().to_string();

    let mut config_file = working_dir.clone();
    config_file.push(".mainframer/config");

    let config = match Config::from_file(config_file.as_path()) {
        Err(message) => exit_with_error(&message, 1),
        Ok(value) => value
    };

    let ignore = Ignore::from_working_dir(&working_dir);

    let start = Instant::now();

    match sync_local_to_remote(&working_dir_name, &config, &ignore) {
        Err(error) => exit_with_error(&format!("Sync local → remote machine failed: {}.", error), 1),
        Ok(_) => ()
    }

    let (remote_command_tx, remote_command_rx) = mpsc::channel();

    let rc_working_dir_name = working_dir_name.clone();
    let rc_config = config.clone();

    thread::spawn(move || {
        let remote_command_result = execute_remote_command(&rc_working_dir_name, &args, &rc_config);
        remote_command_tx.send(remote_command_result).unwrap();
    });

    // Signal to the channel means "stop sync".
    let (sync_remote_to_local_tx, sync_remote_to_local_rx) = mpsc::channel();

    let srtl_working_dir_name = working_dir_name.clone();
    let srtl_config = config.clone();
    let srtl_ignore = ignore.clone();

    let sync_remote_to_local_handle = thread::spawn(move || {
        let mut should_run = true;
        let mut received_stop_signal = false;

        while should_run {
            match sync_remote_to_local(&srtl_working_dir_name, &srtl_config, &srtl_ignore) {
                Err(error) => exit_with_error(&format!("Sync remote → local machine failed: {}.", error), 1),
                Ok(_) => ()
            }

            if received_stop_signal {
                // Do one final sync after stop signal.
                should_run = false
            }

            let stop_signal = sync_remote_to_local_rx.try_recv();

            match stop_signal {
                Err(e) => match e {
                    Disconnected => should_run = false,
                    Empty => should_run = true,
                },
                Ok(_) => received_stop_signal = true
            }
        }
    });

    // Block until remote command finishes.
    let remote_command_result = remote_command_rx.recv().unwrap();

    // Stop continuous sync.
    sync_remote_to_local_tx.send(()).unwrap();

    // Wait for sync thread to finish.
    sync_remote_to_local_handle.join().unwrap();

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

fn sync_local_to_remote(working_dir_name: &str, config: &Config, ignore: &Ignore) -> Result<(), String> {
    println!("Sync local → remote machine...");

    let start = Instant::now();

    let result = sync::sync_local_to_remote(
        &working_dir_name,
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

fn execute_remote_command(working_dir_name: &str, args: &Args, config: &Config) -> Result<(), ()> {
    println!("Executing command on remote machine...\n");

    let start = Instant::now();

    let result = remote_command::execute_remote_command(
        &args.command.clone(),
        config,
        &format!("~/mainframer/{}", working_dir_name),
    );

    let duration = start.elapsed();

    match result {
        Err(_) => eprintln!("\nExecution failed: took {}.\n", format_duration(duration)),
        Ok(_) => println!("\nExecution done: took {}.\n", format_duration(duration))
    }

    result
}

fn sync_remote_to_local(working_dir_name: &str, config: &Config, ignore: &Ignore) -> Result<(), String> {
    println!("Sync remote → local machine...");

    let start = Instant::now();

    let result = sync::sync_remote_to_local(
        &working_dir_name,
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
