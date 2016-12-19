# Terms

To use this Remote Build script You will need to receive following information before proceeding.

* `BUILD_MACHINE_NAME` — build machine hostname.
* `BUILD_MACHINE_IP_OR_HOSTNAME` — build machine IP or hostname. Something like `172.42.42.42` or `woooot.remotebuild.mycompany.com`.
* `BUILD_MACHINE_USERNAME` — your build machine user. Something like `john_doe`.

# Authentication

  * Configure ssh.

  * Generate new ssh key or use existing one.
  ```
  $ ssh-keygen -t rsa -b 4096 -C "{BUILD_MACHINE_USERNAME}"
  ```
  * Run the command
  * Enter file location for new key, ie: `~/.ssh/remote-build`.
  

  * Append following content to `~/.ssh/config`.

  ```config
  Host {BUILD_MACHINE_NAME}
    User {BUILD_MACHINE_USERNAME}
    HostName {BUILD_MACHINE_IP_OR_HOSTNAME}
    IdentityFile ~/.ssh/remote-build
    PreferredAuthentications publickey
    ControlMaster auto
    ControlPath /tmp/%r@%h:%p
    ControlPersist 1h
  ```

  * Specifying `PreferredAuthentications` speeds up connection phase.
  * `ControlMaster` allows reusing existing ssh connections.
  * `ControlPersist` specifies for how long ssh should keep connection open.

* Copy and send public key to the person responsible for maintenance of remote build infrastructure.

  ```
  # On Mac OS you can copy content from commandline.
  $ pbcopy < ~/.ssh/sshkey.pub
  ```

* Once you receive confirmation, test the connection.

  ```
  $ ssh {BUILD_MACHINE_HOSTNAME}
  ```

# Local Configuration

Download [`remote_build.sh`](remote_build.sh) and save it in your project (we recommend to put it under version control so you could sync changes across all team members).

Put the following content in your local `local.properties` file.

```properties
# Required.
remote_build.machine={BUILD_MACHINE_NAME}
```

Now you can test the build.

  ```
  $ bash remote_build.sh ./gradlew assembleDebug
  ```

# Android Studio Configuration

1. `Run` → `Edit Configuration` → `+`.
* Create the desired configuration as you would for local execution.
  * Name: use something meaningful, like `*-remote-build`.
* Remove `Gradle-aware Make` from `Before Launch` section.
* Create step in `Before Launch` section for `Run External Tool`.
  * Name: use something meaningful, like `remote assembleDebug`.
  * Program: `bash`.
  * Parameters: `remote_build.sh ./gradlew :app:assembleDebug` or any Gradle/etc command you want.
  * Working directory: `$ProjectFileDir$`.

# Android Studio Configuration for JUnit tests

1. `Run` → `Edit Configuration` → `Defaults` → `JUnit`.
* Remove default entry from `Before Launch` section.
(To return local test builds revert `Before launch` step to `Gradle-aware make`)
* Create step in `Before Launch` section for `Run External Tool`.
  * Name: use something meaningful, like `remote compileDebugUnitTestSources`.
  * Program: `bash`.
  * Parameters: `remote_build.sh ./gradlew compileDebugUnitTestSources mockableAndroidJar`
(NOTE: Turning incremental kotlin compilation ON can lead to tests running issues. Build Cache and Minimum SDK tuning are OK.).
  * Working directory: `$ProjectFileDir$`.
2. Run required JUnit tests as usual.
3. If tests are failing to start then run remote clean `remote_build.sh ./gradlew clean`, sync project locally and repeat step `2`.

# Performance Optimizations

## Specify more precise task


For example `assembleDebug` for specific module.

```
$ ./gradlew :app:assembleDebug
```

instead of 

```
$ ./gradlew assembleDebug
```

## Change Minimum SDK

```
$ ./gradlew :app:assembleDebug -Pmin.sdk.version=21
```

See https://artemzin.com/blog/minsdk-without-flavors/

## Enable Build Cache of Android Gradle Plugin

```
$ ./gradlew :app:assembleDebug -Pandroid.enableBuildCache=true
```

## Enable Kotlin Incremental Compilation (if you use Kotlin of course)

Note: it doesn't work very well with Unit tests.

```
$ ./gradlew :app:assembleDebug -Pkotlin.incremental=true
```

## Change compression level

You can tune compression levels as you wish.

For example, if your network is very slow, you might consider increasing compression level up to `9` to download/upload less data.
If network is very fast, you might consider disable compression at all by passing `0` as a value. Default values are `1`.

Configurable via properties in `local.properties`:

```
# Optional.
remote_build.local_gzip_level=1

# Optional.
remote_build.remote_gzip_level=1
```


License
=======

    Copyright 2016 Juno, Inc.

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.