# How to setup Remote Build Machine

### OS

We recommend any Linux based OS, Mac OS should work fine too and it seems that Windows can also be possible variant either with [Cygwin](https://www.cygwin.com) or [Bash on Ubuntu on Windows](https://msdn.microsoft.com/en-us/commandline/wsl/install_guide).

### Users

We recommend create separate user for each real user of remote build.

```bash
$ adduser artem_zinnatullin
```

Then add ssh key of that user to `~/.ssh`:

```bash
$ mkdir -p ~/.ssh
$ chmod 700 ~/.ssh
$ echo required_ssh_key >> ~/.ssh/authorized_keys
$ chmod 600 ~/.ssh/authorized_keys
```

### Required software

`rsync`, `ssh` running as daemon.

### Environment for the build

In our case we need JDK and Android SDK installed on the remote machine (each user has its own Android SDK copy).

```bash
$ wget https://dl.google.com/android/repository/tools_r25.2.3-linux.zip
$ unzip tools_r25.2.3-linux.zip -d android-sdk-linux
```

Environment variables:

To build Android project you will need `ANDROID_HOME` env var available, on most Linux distros you can add `export ANDROID_HOME=~/android-sdk-linux` to the `~/.bashrc` to achieve that.

