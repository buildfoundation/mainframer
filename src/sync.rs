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
pub enum PullMode {
    /// Serial, after remote command execution.
    Serial,

    /// Parallel to remote command execution.
    /// First parameter is pause between sync actions.
    Parallel(Duration),
}

#[derive(Debug, PartialEq, Clone)]
pub enum PullResult {
    Ok(Duration),
    Err(Duration, String),
}

// TODO add internal version of sync functions with closures as parameters to unit test properly.
pub fn sync_local_to_remote(local_dir_absolute_path: &Path, config: &Config, ignore: &Ignore) -> Result<(), String> {
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

    execute_rsync(&mut command)
}

pub fn sync_remote_to_local(local_dir_absolute_path: &Path, config: Config, ignore: Ignore, sync_mode: &PullMode, remote_command_finished_signal: BusReader<RemoteCommandResult>) -> Receiver<PullResult> {
    match sync_mode {
        PullMode::Serial => sync_remote_to_local_serial(local_dir_absolute_path.to_path_buf(), config, ignore, remote_command_finished_signal),
        PullMode::Parallel(pause_between_sync) => sync_remote_to_local_parallel(local_dir_absolute_path.to_path_buf(), config, ignore, *pause_between_sync, remote_command_finished_signal)
    }
}

fn sync_remote_to_local_serial(local_dir_absolute_path: PathBuf, config: Config, ignore: Ignore, mut remote_command_finished_signal: BusReader<RemoteCommandResult>) -> Receiver<PullResult> {
    let (sync_finished_tx, sync_finished_rx): (Sender<PullResult>, Receiver<PullResult>) = unbounded();

    thread::spawn(move || {
        remote_command_finished_signal
            .recv()
            .expect("Could not receive remote_command_finished_signal");

        sync_finished_tx
            .send(_sync_remote_to_local(local_dir_absolute_path.as_path(), &config, &ignore))
            .expect("Could not send sync_finished signal");
    });

    sync_finished_rx
}

fn sync_remote_to_local_parallel(local_dir_absolute_path: PathBuf, config: Config, ignore: Ignore, pause_between_sync: Duration, mut remote_command_finished_signal: BusReader<RemoteCommandResult>) -> Receiver<PullResult> {
    let start_time = Instant::now();

    let (sync_finished_tx, sync_finished_rx): (Sender<PullResult>, Receiver<PullResult>) = unbounded();

    thread::spawn(move || {
        let mut should_run = true;

        while should_run {
            match _sync_remote_to_local(local_dir_absolute_path.as_path(), &config, &ignore) {
                PullResult::Err(_, reason) => {
                    should_run = false;
                    sync_finished_tx
                        .send(PullResult::Err(start_time.elapsed(), reason)) // TODO handle code 24.
                        .expect("Could not send sync_finished signal");
                },
                PullResult::Ok(_) => thread::sleep(pause_between_sync),
            }

            match remote_command_finished_signal.try_recv() {
                Err(reason) => match reason {
                    Disconnected => should_run = false,
                    Empty => thread::sleep(pause_between_sync)
                },
                Ok(_) => {
                    should_run = false;

                    // Final sync after remote command to ensure consistency of the files.
                    match _sync_remote_to_local(local_dir_absolute_path.as_path(), &config, &ignore) {
                        PullResult::Err(_, reason) => sync_finished_tx
                            .send(PullResult::Err(start_time.elapsed(), reason))
                            .expect("Could not send sync finished signal (last iteration)"),
                        PullResult::Ok(_) => sync_finished_tx
                            .send(PullResult::Ok(start_time.elapsed()))
                            .expect("Could not send sync finished signal (last iteration)"),
                    }
                }
            }
        }
    });

    sync_finished_rx
}

fn _sync_remote_to_local(local_dir_absolute_path: &Path, config: &Config, ignore: &Ignore) -> PullResult {
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
        Err(_) => Err(String::from("Generic sync error.")), // Rust doc doesn't really say when can an error occur.
        Ok(output) => match output.status.code() {
            None => Err(String::from("Sync was terminated.")),
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
