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
use remote_command::{RemoteCommandOk, RemoteCommandErr};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PushOk {
    pub duration: Duration,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PushErr {
    pub duration: Duration,
    pub message: String,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum PullMode {
    /// Serial, after remote command execution.
    Serial,

    /// Parallel to remote command execution.
    /// First parameter is pause between pulls.
    Parallel(Duration),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PullOk {
    pub duration: Duration,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PullErr {
    pub duration: Duration,
    pub message: String,
}

pub fn push(local_dir_absolute_path: &Path, config: &Config, ignore: &Ignore) -> Result<PushOk, PushErr> {
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
        Err(reason) => Err(PushErr {
            duration: start_time.elapsed(),
            message: reason,
        }),
        Ok(_) => Ok(PushOk {
            duration: start_time.elapsed()
        }),
    }
}

pub fn pull(local_dir_absolute_path: &Path, config: Config, ignore: Ignore, pull_mode: &PullMode, remote_command_finished_signal: BusReader<Result<RemoteCommandOk, RemoteCommandErr>>) -> Receiver<Result<PullOk, PullErr>> {
    match pull_mode {
        PullMode::Serial => pull_serial(local_dir_absolute_path.to_path_buf(), config, ignore, remote_command_finished_signal),
        PullMode::Parallel(pause_between_pulls) => pull_parallel(local_dir_absolute_path.to_path_buf(), config, ignore, *pause_between_pulls, remote_command_finished_signal)
    }
}

fn pull_serial(local_dir_absolute_path: PathBuf, config: Config, ignore: Ignore, mut remote_command_finished_rx: BusReader<Result<RemoteCommandOk, RemoteCommandErr>>) -> Receiver<Result<PullOk, PullErr>> {
    let (pull_finished_tx, pull_finished_rx): (Sender<Result<PullOk, PullErr>>, Receiver<Result<PullOk, PullErr>>) = unbounded();

    #[allow(unused_must_use)] // We don't handle remote_command_result, in any case we need to pull after it.
    thread::spawn(move || {
        remote_command_finished_rx
            .recv()
            .expect("Could not receive remote_command_finished_rx");

        pull_finished_tx
            .send(_pull(local_dir_absolute_path.as_path(), &config, &ignore))
            .expect("Could not send pull_finished signal");
    });

    pull_finished_rx
}

fn pull_parallel(local_dir_absolute_path: PathBuf, config: Config, ignore: Ignore, pause_between_pulls: Duration, mut remote_command_finished_signal: BusReader<Result<RemoteCommandOk, RemoteCommandErr>>) -> Receiver<Result<PullOk, PullErr>> {
    let (pull_finished_tx, pull_finished_rx): (Sender<Result<PullOk, PullErr>>, Receiver<Result<PullOk, PullErr>>) = unbounded();
    let start_time = Instant::now();

    thread::spawn(move || {
        loop {
            if let Err(pull_err) = _pull(local_dir_absolute_path.as_path(), &config, &ignore) {
                pull_finished_tx
                    .send(Err(pull_err)) // TODO handle code 24.
                    .expect("Could not send pull_finished signal");
                break;
            }

            match remote_command_finished_signal.try_recv() {
                Err(reason) => match reason {
                    Disconnected => break,
                    Empty => thread::sleep(pause_between_pulls)
                },
                Ok(remote_command_result) => {
                    let remote_command_duration = match remote_command_result {
                        Err(err) => err.duration,
                        Ok(ok) => ok.duration
                    };

                    // Final pull after remote command to ensure consistency of the files.
                    match _pull(local_dir_absolute_path.as_path(), &config, &ignore) {
                        Err(err) => pull_finished_tx
                            .send(Err(PullErr {
                                duration: calculate_perceived_pull_duration(start_time.elapsed(), remote_command_duration),
                                message: err.message
                            }))
                            .expect("Could not send pull finished signal (last iteration)"),

                        Ok(_) => pull_finished_tx
                            .send(Ok(PullOk {
                                duration: calculate_perceived_pull_duration(start_time.elapsed(), remote_command_duration)
                            }))
                            .expect("Could not send pull finished signal (last iteration)"),
                    }

                    break;
                }
            }
        }
    });

    pull_finished_rx
}

fn _pull(local_dir_absolute_path: &Path, config: &Config, ignore: &Ignore) -> Result<PullOk, PullErr> {
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
        Err(reason) => Err(PullErr {
            duration: start_time.elapsed(),
            message: reason
        }),
        Ok(_) => Ok(PullOk {
            duration: start_time.elapsed(),
        })
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

fn calculate_perceived_pull_duration(total_pull_duration: Duration, remote_command_duration: Duration) -> Duration {
    match total_pull_duration.checked_sub(remote_command_duration) {
        None => Duration::from_millis(0),
        Some(duration) => duration,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_perceived_pull_duration_equals() {
        assert_eq!(
            calculate_perceived_pull_duration(Duration::from_millis(10), Duration::from_millis(10)),
            Duration::from_millis(0)
        );
    }

    #[test]
    fn calculate_perceived_pull_duration_pull_longer_than_execution() {
        assert_eq!(
            calculate_perceived_pull_duration(Duration::from_secs(10), Duration::from_secs(8)),
            Duration::from_secs(2)
        );
    }

    #[test]
    fn calculate_perceived_pull_duration_pull_less_than_execution() {
        assert_eq!(
            calculate_perceived_pull_duration(Duration::from_secs(7), Duration::from_secs(9)),
            Duration::from_secs(0)
        );
    }
}
