# Getting Started on Local Machine

## Soft Skill Dependencies

Find the person responsible for maintaining the remote machine and ask for the following.

* `REMOTE_MACHINE_IP_OR_HOSTNAME` — remote machine IP or hostname. Examples: `42.42.42.42`, `remote.domain.com`.
* `REMOTE_MACHINE_PORT` — remote machine port. Example: `42`.
* `REMOTE_MACHINE_USERNAME` — remote machine user, most likely a personalized, not shared one. Example: `john.doe`.

Think about an alias for the remote machine.

* `REMOTE_MACHINE_ALIAS` — SSH alias for remote machine. Something like `mainframe` or `build-machine`.

## Authentication

1. Generate SSH key and remember the resulting file name as `{SSH_KEY_NAME}`.

    ```
    $ ssh-keygen -t rsa -b 4096 -C "{REMOTE_MACHINE_USERNAME}"
    ```

2. Append the following content to `~/.ssh/config`.

    ```sshconfig
    Host {REMOTE_MACHINE_ALIAS}
      User {REMOTE_MACHINE_USERNAME}
      HostName {REMOTE_MACHINE_IP_OR_HOSTNAME}
      Port {REMOTE_MACHINE_PORT}
      IdentityFile ~/.ssh/{SSH_KEY_NAME}
      PreferredAuthentications publickey
      ControlMaster auto
      ControlPath /tmp/%r@%h:%p
      ControlPersist 1h
    ```

    * `ControlMaster` enables reusing SSH connection.
    * `ControlPersist` specifies for how long SSH should keep connection open.

3. Send SSH public key to a person responsible for remote machine maintenance.

    ```shell
    # macOS-specific. Linux users — godspeed.
    $ pbcopy < ~/.ssh/{SSH_KEY_NAME}.pub
    ```

4. Once you’ve received a confirmation that the remote machine is ready for you, try the connection.

    ```
    $ ssh {REMOTE_MACHINE_ALIAS}
    ```

## Installation

### macOS

```
$ brew tap buildfoundation/homebrew-tap
$ brew install mainframer
```

## Configuration

* Is the Mainframer already used on the project? You’ll need to create the config file.
* Is the Mainframer not used on the project? You’ll need to create the config file and ignore rules.

Please refer to the [documentation](../configuration.md).

## Running

```
$ mainframer echo "It works!" > success.txt
$ cat success.txt
```

Congratulations! You’ve created a `success.txt` file on the remote machine
and viewed the resulting content on the local machine.

