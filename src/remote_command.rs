use config::Config;
use std::process::Command;
use std::process::Stdio;

pub fn execute_remote_command(remote_command: &str, config: &Config, project_dir_on_remote_machine: &str) -> Result<(), ()> {
    let mut command = Command::new("ssh");

    command
        .arg(config.remote_machine_name.clone())
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
        Err(_) => Err(()), // No need to get error description as we've already piped command output to Mainframer output.
        Ok(exit_status) => match exit_status.success() {
            false => Err(()),
            true => Ok(())
        }
    }
}