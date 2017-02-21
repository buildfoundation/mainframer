# Remote Machine Setup

## Dependencies

* SSH Server
* rsync

## Users

We recommend to create a separate user per person.
There are other options like a Docker container per person though.

1. Create user.

  ```
  $ adduser {FIRST_NAME}_{LAST_NAME}
  ```

2. Place user SSH key.

  ```
  $ mkdir -p ~/.ssh
  $ chmod u+rwx,go= ~/.ssh
  $ echo {SSH_KEY} >> ~/.ssh/authorized_keys
  $ chmod u+rw,go= ~/.ssh
  ```

## Environment

Install tools you need to perform remote processing.

If a build system requires Shell initialization scripts or special environment variables,
you should place them to your `~/.bashrc` or `~/.bash_profile` file depending on your remote OS.
Be careful, some Bash configurations do block config evaluation when detecting non-interactive mode.
In such case, you should place your declarations before the blocking instruction such as the following one.

```bash
# If not running interactively, don't do anything
```

## Recipe

We've built a [recipe to ease the setup of remote machine](../recipes/SETUP_REMOTE_MACHINE.md) for you.
