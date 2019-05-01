use bus::{Bus, BusReader};
use config::Config;
use std::process::Command;
use std::process::Stdio;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, PartialEq, Clone)]
pub enum RemoteCommandResult {
    Ok(Duration),
    Err(Duration),
}

pub fn execute_remote_command(remote_command: String, config: Config, project_dir_on_remote_machine: String, number_of_readers: usize) -> Vec<BusReader<RemoteCommandResult>> {
    let mut bus: Bus<RemoteCommandResult> = Bus::new(1);
    let mut readers: Vec<BusReader<RemoteCommandResult>> = Vec::with_capacity(number_of_readers);

    for _ in 0..number_of_readers {
        readers.push(bus.add_rx())
    }

    thread::spawn(move || {
        bus.broadcast(_execute_remote_command(&remote_command, &config, &project_dir_on_remote_machine));
    });

    readers
}

fn _execute_remote_command(remote_command: &str, config: &Config, project_dir_on_remote_machine: &str) -> RemoteCommandResult {
    let start_time = Instant::now();

    let mut command = Command::new("ssh");

    command
        .arg(config.remote.host.clone())
        .arg(format!(
            "echo 'set -e && cd {project_dir_on_remote_machine} && echo \"{remote_command}\" && echo \"\" && {remote_command}' | bash",
            project_dir_on_remote_machine = project_dir_on_remote_machine,
            remote_command = remote_command)
        );

    let mut process = command
        // Interactively pipe ssh output to Mainframer output.
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    match process.wait() {
        Err(_) => RemoteCommandResult::Err(start_time.elapsed()), // No need to get error description as we've already piped command output to Mainframer output.
        Ok(exit_status) => if exit_status.success() {
            RemoteCommandResult::Ok(start_time.elapsed())
        } else {
            RemoteCommandResult::Err(start_time.elapsed())
        }
    }
}
