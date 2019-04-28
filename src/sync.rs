use config::Config;
use ignore::Ignore;
use std::error::Error;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;
use crossbeam_channel::TryRecvError::*;
use crossbeam_channel::unbounded;
use std::time::Duration;
use std::thread;

#[derive(Debug, PartialEq, Clone)]
pub enum PullMode {
    /// Serial, after remote command execution.
    Serial,

    /// Parallel to remote command execution.
    /// First parameter is pause between sync actions.
    Parallel(Duration),
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

pub fn sync_remote_to_local(local_dir_absolute_path: &Path, config: Config, ignore: Ignore, sync_mode: PullMode, remote_command_finished_signal: Receiver<Result<(), ()>>) -> Receiver<Result<(), String>> {
    match sync_mode {
        PullMode::Serial => sync_remote_to_local_serial(local_dir_absolute_path.to_path_buf(), config, ignore, remote_command_finished_signal),
        PullMode::Parallel(pause_between_sync) => sync_remote_to_local_parallel(local_dir_absolute_path.to_path_buf(), config, ignore, pause_between_sync, remote_command_finished_signal)
    }
}

fn sync_remote_to_local_serial(local_dir_absolute_path: PathBuf, config: Config, ignore: Ignore, remote_command_finished_signal: Receiver<Result<(), ()>>) -> Receiver<Result<(), String>> {
    let (sync_finished_tx, sync_finished_rx): (Sender<Result<(), String>>, Receiver<Result<(), String>>) = unbounded(); // TODO remove me mpsc::channel();

    thread::spawn(move || {
        match remote_command_finished_signal.recv() {
            Err(error) => sync_finished_tx.send(Err(error.description().to_string())),
            Ok(remote_command_result) => match remote_command_result {
                Ok(_) | Err(_) => {
                    match _sync_remote_to_local(local_dir_absolute_path.as_path(), config, ignore) {
                        Err(message) => sync_finished_tx.send(Err(message)),
                        Ok(_) => sync_finished_tx.send(Ok(()))
                    }
                }
            }
        }
    });

    return sync_finished_rx;
}

fn sync_remote_to_local_parallel(local_dir_absolute_path: PathBuf, config: Config, ignore: Ignore, pause_between_sync: Duration, remote_command_finished_signal: Receiver<Result<(), ()>>) -> Receiver<Result<(), String>> {
    let (sync_finished_tx, sync_finished_rx): (Sender<Result<(), String>>, Receiver<Result<(), String>>) = unbounded(); // TODO remove me mpsc::channel();

    thread::spawn(move || {
        let mut should_run = true;

        while should_run {
            match _sync_remote_to_local(local_dir_absolute_path.as_path(), config.clone(), ignore.clone()) {
                Err(reason) => {
                    should_run = false;
                    sync_finished_tx
                        .send(Err(reason)) // TODO handle code 24.
                        .expect("Could not send sync_finished signal.");
                },
                Ok(_) => {
                    thread::sleep(pause_between_sync)
                }
            }

            match remote_command_finished_signal.try_recv() {
                Err(reason) => match reason {
                    Disconnected => should_run = false,
                    Empty => thread::sleep(pause_between_sync)
                },
                Ok(remote_command_result) => match remote_command_result {
                    Ok(_) | Err(_) => {
                        should_run = false;

                        // Final sync after remote command to ensure consistency of the files.
                        sync_finished_tx
                            .send(_sync_remote_to_local(local_dir_absolute_path.as_path(), config.clone(), ignore.clone()))
                            .expect("Could not send sync finished signal (last iteration).");
                    },
                }
            }
        }
    });

    return sync_finished_rx;
}

fn _sync_remote_to_local(local_dir_absolute_path: &Path, config: Config, ignore: Ignore) -> Result<(), String> {
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

    execute_rsync(&mut command)
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
