use config::Config;
use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;
use crossbeam_channel::unbounded;
use std::process::Command;
use std::process::Stdio;
use std::thread;

pub fn execute_remote_command(remote_command: String, config: Config, project_dir_on_remote_machine: String) -> Receiver<Result<(), ()>> {
    let (remote_command_finished_tx, remote_command_finished_rx): (Sender<Result<(), ()>>, Receiver<Result<(), ()>>) = unbounded();

    thread::spawn(move || {
        remote_command_finished_tx
            .send(_execute_remote_command(remote_command, config, project_dir_on_remote_machine))
            .expect("Could not send remote_command_finished signal.");
    });

    return remote_command_finished_rx;
}

fn _execute_remote_command(remote_command: String, config: Config, project_dir_on_remote_machine: String) -> Result<(), ()> {
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
        Err(_) => Err(()), // No need to get error description as we've already piped command output to Mainframer output.
        Ok(exit_status) => if exit_status.success() {
            Ok(())
        } else {
            Err(())
        }
    }
}
