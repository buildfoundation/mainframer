use config::Config;
use ignore::Ignore;
use std::path::PathBuf;
use std::process::Command;

// TODO add internal version of sync functions with closures as parameters to unit test properly.
pub fn sync_local_to_remote(local_project_dir_name: &str, config: &Config, ignore: &Ignore) -> Result<(), String> {
    let mut command = Command::new("rsync");

    command
        .arg("--archive")
        .arg("--delete")
        // Create (if not exists) project dir on remote machine.
        .arg(format!("--rsync-path=\"mkdir -p \"{}\" && rsync", project_dir_on_remote_machine(local_project_dir_name)))
        .arg(format!("--compress-level={}", config.local_compression_level));

    apply_exclude_from(&mut command, &ignore.common_ignore_file);
    apply_exclude_from(&mut command, &ignore.local_ignore_file);

    command.arg(format!(
        "--rsh ssh ./ {remote_machine_name}:'{project_dir_on_remote_machine}'",
        remote_machine_name = config.remote_machine_name,
        project_dir_on_remote_machine = project_dir_on_remote_machine(local_project_dir_name))
    );

    let result = command.output();

    match result {
        Err(_) => Err(String::from("Failed to sync files from local machine to remote.")),
        Ok(output) => match output.status.code() {
            None => Err(String::from("Sync of files from local machine to remote was terminated.")),
            Some(status_code) => match status_code {
                0 => Ok(()),
                _ => Err(format!("Failed to sync files from local machine to remote, rsync exit code '{}'.", status_code)) // TODO append rsync output for more info.
            }
        }
    }
}

pub fn sync_remote_to_local(local_project_dir_name: &str, config: &Config, ignore: &Ignore) -> Result<(), String> {
    let mut command = Command::new("rsync");

    command
        .arg("--archive")
        .arg("--delete")
        .arg(format!("--compress-level={}", config.remote_compression_level));

    apply_exclude_from(&mut command, &ignore.common_ignore_file);
    apply_exclude_from(&mut command, &ignore.remote_ignore_file);

    // 	COMMAND+="--rsh ssh $REMOTE_MACHINE:'$PROJECT_DIR_ON_REMOTE_MACHINE'/ ./"

    command.arg(format!(
        "--rsh ssh {remote_machine_name}:'{project_dir_on_remote_machine}'/ ./",
        remote_machine_name = config.remote_machine_name,
        project_dir_on_remote_machine = project_dir_on_remote_machine(local_project_dir_name))
    );

    let result = command.output();

    match result {
        Err(_) => Err(String::from("Failed to sync files from remote machine to local.")),
        Ok(output) => match output.status.code() {
            None => Err(String::from("Sync of files from remote machine to local was terminated.")),
            Some(status_code) => match status_code {
                0 => Ok(()),
                _ => Err(format!("Failed to sync files from remote machine to local, rsync exit code '{}'.", status_code)) // TODO append rsync output for more info.
            }
        }
    }
}

fn project_dir_on_remote_machine(local_project_dir_name: &str) -> String {
    format!("~/mainframer/{}", local_project_dir_name)
}

fn apply_exclude_from(rsync_command: &mut Command, exclude_file: &Option<PathBuf>) {
    match exclude_file.clone() {
        Some(value) => {
            rsync_command.arg(format!("--exclude-from={}", value.to_string_lossy()));
        }
        None => ()
    };
}
