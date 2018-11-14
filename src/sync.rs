use config::Config;
use ignore::Ignore;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

// TODO add internal version of sync functions with closures as parameters to unit test properly.
pub fn sync_local_to_remote(local_dir_absolute_path: &Path, config: &Config, ignore: &Ignore) -> Result<(), String> {
    let mut command = Command::new("rsync");

    command
        .arg("--archive")
        .arg("--delete")
        // Create (if not exists) project dir on remote machine.
        .arg(format!("--rsync-path=mkdir -p {} && rsync", project_dir_on_remote_machine(local_dir_absolute_path)))
        .arg(format!("--compress-level={}", config.local_compression_level));

    apply_exclude_from(&mut command, &ignore.common_ignore_file);
    apply_exclude_from(&mut command, &ignore.local_ignore_file);

    command
        .arg("--rsh=ssh")
        .arg("./");

    command.arg(format!(
        "{remote_machine_name}:{project_dir_on_remote_machine}",
        remote_machine_name = config.remote_machine_name,
        project_dir_on_remote_machine = project_dir_on_remote_machine(local_dir_absolute_path))
    );

    execute_rsync(&mut command)
}

pub fn sync_remote_to_local(local_dir_absolute_path: &Path, config: &Config, ignore: &Ignore) -> Result<(), String> {
    let mut command = Command::new("rsync");

    command
        .arg("--archive")
        .arg("--delete")
        .arg(format!("--compress-level={}", config.remote_compression_level));

    apply_exclude_from(&mut command, &ignore.common_ignore_file);
    apply_exclude_from(&mut command, &ignore.remote_ignore_file);

    command
        .arg("--rsh=ssh")
        .arg(format!(
            "{remote_machine_name}:{project_dir_on_remote_machine}/",
            remote_machine_name = config.remote_machine_name,
            project_dir_on_remote_machine = project_dir_on_remote_machine(local_dir_absolute_path))
        )
        .arg("./");

    execute_rsync(&mut command)
}

pub fn project_dir_on_remote_machine(local_dir_absolute_path: &Path) -> String {
    format!("~/mainframer/{}", local_dir_absolute_path.to_string_lossy())
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
