use config::Config;
use ignore::Ignore;
use std::process::Command;

pub fn sync_local_to_remote(local_project_dir_name: &str, config: &Config, ignore: Ignore) -> Result<(), String> {
    let mut command = Command::new("rsync");

    command
        .arg("--archive")
        .arg("--delete")
        // Create (if not exists) project dir on remote machine.
        .arg(format!("--rsync-path=\"mkdir -p \"{}\" && rsync", project_dir_on_remote_machine(local_project_dir_name)))
        .arg(format!("--compress-level={}", config.local_compression_level));

    match ignore.common_ignore_file {
        Some(value) => {
            command.arg(format!("--exclude-from={}", value.to_string_lossy()));
        },
        None => ()
    }

    match ignore.local_ignore_file {
        Some(value) => {
            command.arg(format!("--exclude-from={}", value.to_string_lossy()));
        },
        None => ()
    }

    command.arg(format!("--rsh ssh ./ {remote_machine_name}:'{project_dir_on_remote_machine}'",
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

pub fn sync_remote_to_local() {

}

fn project_dir_on_remote_machine(local_project_dir_name: &str) -> String {
    format!("~/mainframer/{}", local_project_dir_name)
}

