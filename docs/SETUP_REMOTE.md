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

If some tool require shell initialization scripts or special environment variables you should add them to your ~/.bashrc or ~/.bash_profile file depending on your remote operating system.

Be sure to add all your configurations at the beginning of the file before this line :

  ```
  # If not running interactively, don't do anything
  ```

## Recipe

We've built a [recipe to ease the setup of remote machine](../recipes/SETUP_REMOTE_MACHINE.md) for you.
