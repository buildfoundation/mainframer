# Local Machine Setup

## Info You’ll Need to Start

* `BUILD_MACHINE_NAME` — build machine hostname.
* `BUILD_MACHINE_IP_OR_HOSTNAME` — build machine IP or hostname. Something like `172.42.42.42` or `woooot.remotebuild.company.com`.
* `BUILD_MACHINE_USERNAME` — your build machine user. Something like `john_doe`.

## Authentication

1. Generate SSH key.

  ```
  $ ssh-keygen -t rsa -b 4096 -C "{BUILD_MACHINE_USERNAME}"
  ```

2. Append the following content to `~/.ssh/config`.

  ```config
  Host {BUILD_MACHINE_NAME}
    User {BUILD_MACHINE_USERNAME}
    HostName {BUILD_MACHINE_IP_OR_HOSTNAME}
    IdentityFile ~/.ssh/{SSH_KEY_NAME}
    PreferredAuthentications publickey
    ControlMaster auto
    ControlPath /tmp/%r@%h:%p
    ControlPersist 1h
  ```

  * `ControlMaster` enables SSH connection reusage.
  * `ControlPersist` specifies for how long SSH should keep connection open.

3. Copy and send public key to the person responsible for maintenance of remote build infrastructure.

  ```shell
  # macOS-specific. Linux users, you know what to do.
  $ pbcopy < ~/.ssh/sshkey.pub
  ```

4. Once you’ve received confirmation that build machine is ready for you, test the connection.

  ```
  $ ssh {BUILD_MACHINE_HOSTNAME}
  ```

## Configuration

1. **Download** [latest release version of `mainframer.sh`](https://github.com/gojuno/mainframer/releases/latest) and save it in your project, most likely put it to VCS so you could sync changes across all team members.

  We recommend you to subscribe to changes in this repo somehow (follow it on GitHub / watch for tweets of its maintainers / etc). This will allow you to apply best practises we found to make your Remote Build better, faster and safer.

2. Add the following content to `local.properties`.

  ```properties
  remote_build.machine={BUILD_MACHINE_NAME}
  ```

3. And finally you can test the build.

  ```
  $ bash mainframer.sh ./gradlew assembleDebug
  ```
