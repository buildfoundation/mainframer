# Remote Machine Setup

## Dependencies

* `ssh`
* `rsync`

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
  
## Environment

### Android

We need JDK and Android SDK installed. Each user has its own Android SDK copy.

```
$ wget https://dl.google.com/android/repository/tools_r25.2.3-linux.zip
$ unzip tools_r25.2.3-linux.zip -d android-sdk-linux
```

Append the following line to the `~/.bashrc`.

```
export ANDROID_HOME=~/android-sdk-linux
```
