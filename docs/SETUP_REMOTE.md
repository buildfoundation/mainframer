# Remote Machine Setup

## Dependencies

* SSH Server
* rsync

## Users

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

We recommend to create a separate user per person.
There are other options like a Docker container per person though.
  
## Environment

Install tools you need to perform remote processing.
