use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc::TryRecvError::*;
use std::thread;
use std::time::{Duration, Instant};

use bus::BusReader;
use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;
use crossbeam_channel::unbounded;

use config::Config;
use ignore::Ignore;
use remote_command::RemoteCommandResult;

#[derive(Debug, PartialEq, Clone)]
pub enum PushResult {
    Ok(Duration),
    Err(Duration, String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum PullMode {
    /// Serial, after remote command execution.
    Serial,

    /// Parallel to remote command execution.
    /// First parameter is pause between pulls.
    Parallel(Duration),
}

#[derive(Debug, PartialEq, Clone)]
pub enum PullResult {
    Ok(Duration),
    Err(Duration, String),
}

pub fn push(local_dir_absolute_path: &Path, config: &Config, ignore: &Ignore) -> PushResult {
    let start_time = Instant::now();

    let mut command = Command::new("rsync");

    command
        .arg("--archive")
        .arg("--delete")
        // Create (if not exists) project dir on remote machine.
        .arg(format!("--rsync-path=mkdir -p {} && rsync", project_dir_on_remote_machine(local_dir_absolute_path)))
        .arg(format!("--compress-level={}", config.push.compression));

    apply_exclude_from(&mut command, &ignore.common_ignore_file);
    apply_exclude_from(&mut command, &ignore.local_ignore_file);

    command
        .arg("--rsh=ssh")
        .arg("./");

    command.arg(format!(
        "{remote_machine_name}:{project_dir_on_remote_machine}",
        remote_machine_name = config.remote.host,
        project_dir_on_remote_machine = project_dir_on_remote_machine(local_dir_absolute_path))
    );

    match execute_rsync(&mut command) {
        Err(reason) => PushResult::Err(start_time.elapsed(), reason),
        Ok(_) => PushResult::Ok(start_time.elapsed()),
    }
}

pub fn pull(local_dir_absolute_path: &Path, config: Config, ignore: Ignore, pull_mode: &PullMode, remote_command_finished_signal: BusReader<RemoteCommandResult>) -> Receiver<PullResult> {
    match pull_mode {
        PullMode::Serial => pull_serial(local_dir_absolute_path.to_path_buf(), config, ignore, remote_command_finished_signal),
        PullMode::Parallel(pause_between_pulls) => pull_parallel(local_dir_absolute_path.to_path_buf(), config, ignore, *pause_between_pulls, remote_command_finished_signal)
    }
}

fn pull_serial(local_dir_absolute_path: PathBuf, config: Config, ignore: Ignore, mut remote_command_finished_signal: BusReader<RemoteCommandResult>) -> Receiver<PullResult> {
    let (pull_finished_tx, pull_finished_rx): (Sender<PullResult>, Receiver<PullResult>) = unbounded();

    thread::spawn(move || {
        remote_command_finished_signal
            .recv()
            .expect("Could not receive remote_command_finished_signal");

        pull_finished_tx
            .send(_pull(local_dir_absolute_path.as_path(), &config, &ignore))
            .expect("Could not send pull_finished signal");
    });

    pull_finished_rx
}

fn pull_parallel(local_dir_absolute_path: PathBuf, config: Config, ignore: Ignore, pause_between_pulls: Duration, mut remote_command_finished_signal: BusReader<RemoteCommandResult>) -> Receiver<PullResult> {
    let start_time = Instant::now();

    let (pull_finished_tx, pull_finished_rx): (Sender<PullResult>, Receiver<PullResult>) = unbounded();

    thread::spawn(move || {
        let mut should_run = true;

        while should_run {
            match _pull(local_dir_absolute_path.as_path(), &config, &ignore) {
                PullResult::Err(_, reason) => {
                    should_run = false;
                    pull_finished_tx
                        .send(PullResult::Err(start_time.elapsed(), reason)) // TODO handle code 24.
                        .expect("Could not send pull_finished signal");
                },
                PullResult::Ok(_) => thread::sleep(pause_between_pulls),
            }

            match remote_command_finished_signal.try_recv() {
                Err(reason) => match reason {
                    Disconnected => should_run = false,
                    Empty => thread::sleep(pause_between_pulls)
                },
                Ok(_) => {
                    should_run = false;

                    // Final pull after remote command to ensure consistency of the files.
                    match _pull(local_dir_absolute_path.as_path(), &config, &ignore) {
                        PullResult::Err(_, reason) => pull_finished_tx
                            .send(PullResult::Err(start_time.elapsed(), reason))
                            .expect("Could not send pull finished signal (last iteration)"),
                        PullResult::Ok(_) => pull_finished_tx
                            .send(PullResult::Ok(start_time.elapsed()))
                            .expect("Could not send pull finished signal (last iteration)"),
                    }
                }
            }
        }
    });

    pull_finished_rx
}

fn _pull(local_dir_absolute_path: &Path, config: &Config, ignore: &Ignore) -> PullResult {
    let start_time = Instant::now();

    let mut command = Command::new("rsync");

    command
        .arg("--archive")
        .arg("--delete")
        .arg(format!("--compress-level={}", config.pull.compression));

    apply_exclude_from(&mut command, &ignore.common_ignore_file);
    apply_exclude_from(&mut command, &ignore.remote_ignore_file);

    command
        .arg("--rsh=ssh")
        .arg(format!(
            "{remote_machine_name}:{project_dir_on_remote_machine}/",
            remote_machine_name = config.remote.host,
            project_dir_on_remote_machine = project_dir_on_remote_machine(local_dir_absolute_path))
        )
        .arg("./");

    match execute_rsync(&mut command) {
        Err(reason) => PullResult::Err(start_time.elapsed(), reason),
        Ok(_) => PullResult::Ok(start_time.elapsed())
    }
}

pub fn project_dir_on_remote_machine(local_dir_absolute_path: &Path) -> String {
    format!("~/mainframer{}", local_dir_absolute_path.to_string_lossy())
}

fn apply_exclude_from(rsync_command: &mut Command, exclude_file: &Option<PathBuf>) {
    match exclude_file {
        Some(ref value) => {
            rsync_command.arg(format!("--exclude-from={}", value.to_string_lossy()));
        }
        None => ()
    };
}

fn execute_rsync(rsync: &mut Command) -> Result<(), String> {
    let result = rsync.output();

    match result {
        Err(_) => Err(String::from("Generic rsync error.")), // Rust doc doesn't really say when can an error occur.
        Ok(output) => match output.status.code() {
            None => Err(String::from("rsync was terminated.")),
            Some(status_code) => match status_code {
                0 => Ok(()),
                _ => Err(
                    format!(
                        "rsync exit code '{exit_code}',\nrsync stdout '{stdout}',\nrsync stderr '{stderr}'.",
                        exit_code = status_code,
                        stdout = String::from_utf8_lossy(&output.stdout),
                        stderr = String::from_utf8_lossy(&output.stderr)
                    )
                )
            }
        }
    }
}
