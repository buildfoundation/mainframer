# Getting Started on Local Machine

## Soft Skill Dependencies

Find the person responsible for maintaining the remote machine and ask for the following.

* `REMOTE_MACHINE_IP_OR_HOSTNAME` — IP or hostname. Examples: `24.24.24.24`, `remote.domain.com`.
* `REMOTE_MACHINE_SSH_PORT` — SSH port. Example: `42`.
* `REMOTE_MACHINE_USERNAME` — username, most likely a personalized, not shared one. Example: `john_doe`.

Think about an alias for the remote machine.

* `REMOTE_MACHINE_SSH_ALIAS` — SSH alias. Example: `mainframe`.

## Authentication

1. Generate SSH key and remember the resulting file name as `{SSH_KEY_FILE_NAME}`.

    ```console
    $ ssh-keygen -t rsa -b 4096 -C "{REMOTE_MACHINE_USERNAME}"
    ```

2. Append the following content to `~/.ssh/config`.

    ```sshconfig
    Host {REMOTE_MACHINE_SSH_ALIAS}
      User {REMOTE_MACHINE_USERNAME}
      HostName {REMOTE_MACHINE_IP_OR_HOSTNAME}
      Port {REMOTE_MACHINE_SSH_PORT}
      IdentityFile ~/.ssh/{SSH_KEY_FILE_NAME}
      PreferredAuthentications publickey
      ControlMaster auto
      ControlPath /tmp/%r@%h:%p
      ControlPersist 1h
    ```

    * `ControlMaster` enables reusing SSH connection.
    * `ControlPersist` specifies SSH connection timeout.

3. Send SSH public key to a person responsible for the remote machine maintenance.
4. Once you’ve received a confirmation that the remote machine is ready for you, try the connection.

    ```console
    $ ssh {REMOTE_MACHINE_SSH_ALIAS}
    ```

## Installation

### macOS

```console
$ brew tap buildfoundation/homebrew-tap
$ brew install mainframer
```

## Configuration

* Is the Mainframer already used in the project? You’ll need to create the config file.
* Is the Mainframer not used in the project? You’ll need to create the config file and ignore rules.

Please refer to [the documentation](../configuration/description.md).

## Running

```console
$ mainframer 'echo "It works" > success.txt'
$ cat success.txt
```

Congratulations! You’ve created a `success.txt` file on the remote machine
and viewed the resulting content on the local machine.

