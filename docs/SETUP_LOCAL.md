# Local Machine Setup

## Dependencies

* `ssh`
* `rsync`

## Info You’ll Need to Start

* `REMOTE_MACHINE_ALIAS` — ssh alias for remote machine. Something like `blood-dragon` or `sonic`.
* `REMOTE_MACHINE_IP_OR_HOSTNAME` — remote machine IP or hostname. Something like `42.42.42.42` or `remote.domain.com`.
* `REMOTE_MACHINE_USERNAME` — your remote machine user. Something like `john_doe`.

## Authentication

1. Generate SSH key.

  ```
  $ ssh-keygen -t rsa -b 4096 -C "{REMOTE_MACHINE_USERNAME}"
  ```

2. Append the following content to `~/.ssh/config`.

  ```config
  Host {REMOTE_MACHINE_ALIAS}
    User {REMOTE_MACHINE_USERNAME}
    HostName {REMOTE_MACHINE_IP_OR_HOSTNAME}
    Port 22
    IdentityFile ~/.ssh/{SSH_KEY_NAME}
    PreferredAuthentications publickey
    ControlMaster auto
    ControlPath /tmp/%r@%h:%p
    ControlPersist 1h
  ```

  * `ControlMaster` enables SSH connection reusage.
  * `ControlPersist` specifies for how long SSH should keep connection open.

3. Copy and send public key to a person responsible for remote machine maintenance.

  ```shell
  # macOS-specific. Linux users, you know what to do.
  $ pbcopy < ~/.ssh/{SSH_KEY_NAME}.pub
  ```

4. Once you’ve received confirmation that remote machine is ready for you, try the connection.

  ```
  $ ssh {REMOTE_MACHINE_ALIAS}
  ```

## Configuration

1. Download [latest release version of `mainframer.sh`](https://github.com/gojuno/mainframer/releases/latest) and save it in your project. Most likely commit it into version control system so you could sync changes across all team members.

  We recommend you to subscribe to changes in this repo (follow it on GitHub / watch for tweets of its maintainers / etc). This will allow you to apply best practises we found to make `mainframer` faster and safer.

2. Carefully read [Options documentation](OPTIONS.md) and configure mainframer for your kind of project.

3. Finally you can test the build.

  ```
  $ bash mainframer.sh echo "It works!" > success.txt
  ```
