# mainframer

Blip-blop, it's `mainframer`, simple (at the moment lol) script that'll allow you move build process from your local machine to some remote machine and free up your local machine for better things like editing code in your IDE without lags and freezes, not running into swap, be able to actually use your computer while build is happening somewhere else.

For now `mainframer` designed to work with Gradle (and as a result: Android projects that use [Gradle](https://gradle.org/)), but it's pretty universal concept, so we're ~open for other things like `go` in particular, but please file an issue before contributing support for other build systems, thanks!

##Small video that shows remote build started from Android Studio

[![Video](video_preview.png)](https://youtu.be/xysQXMaPaGw "Remote Build — mainframer v1.0.0")

# Before

#### How to setup build machine

See [instruction](HOW_TO_SETUP_REMOTE_BUILD_MACHINE.md) (it's actually very easy: separate user for each real user, installed JDK and Android SDK, that's it!).

#### Info you'll need to start
To use this Remote Build script You will need to receive following information before proceeding.

* `BUILD_MACHINE_NAME` — build machine hostname.
* `BUILD_MACHINE_IP_OR_HOSTNAME` — build machine IP or hostname. Something like `172.42.42.42` or `woooot.remotebuild.mycompany.com`.
* `BUILD_MACHINE_USERNAME` — your build machine user. Something like `john_doe`.

# Authentication

### Configure ssh

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
  # On macOS you can copy content from commandline.
  $ pbcopy < ~/.ssh/sshkey.pub
  ```

* Once you've received confirmation that build machine is ready for you, test the connection.

  ```
  $ ssh {BUILD_MACHINE_HOSTNAME}
  ```

# Local Configuration

Download [`mainframer.sh`](mainframer.sh) and save it in your project (we recommend to put it under version control so you could sync changes across all team members).

Also:
>We recommend you subscribe to changes in this repo somehow (follow it on GitHub / watch for tweets of its maintainers / etc), this will allow you always apply best practises we found to make your Remote Build better, faster and safer.

Put the following content in your local `local.properties` file.

```properties
# Required.
remote_build.machine={BUILD_MACHINE_NAME}
```

That'll be passed to `ssh` as parameter, `user@machine` or `machine` or `ip` will be ok (depending on your ssh and remote machine config of course).

Now you can test the build.

  ```
  $ cd your_project
  $ bash mainframer.sh ./gradlew assembleDebug

  $ # Pro user will notice that actually we allow execute any command on remote machine during the "build".
  ```

# Android Studio Configuration to build and run APK

1. `Run` → `Edit Configuration` → `+`.
* Select "Android App".
  * Name: something meaningful, like `*-remote-build`.
* Remove `Gradle-aware Make` from `Before Launch` section (ha!).
* Create step in `Before Launch` section for `Run External Tool`.
  * Name: use something meaningful, like `remote assembleDebug`.
  * Program: `bash`.
  * Parameters: `mainframer.sh ./gradlew :app:assembleDebug` or any Gradle/etc command you want.
  * Working directory: `$ProjectFileDir$`.

Note: local Gradle sync is required sometimes because this is how Android Studio determines resulting `apk` name even though we'll build it on a remote machine.

# Android Studio Configuration for JUnit tests

1. `Run` → `Edit Configuration` → `Defaults` → `JUnit`.
* Remove default entry from `Before Launch` section.
(To return local test builds revert `Before launch` step to `Gradle-aware make`)
* Create step in `Before Launch` section for `Run External Tool`.
  * Name: use something meaningful, like `remote compileDebugUnitTestSources`.
  * Program: `bash`.
  * Parameters: `mainframer.sh ./gradlew compileDebugUnitTestSources mockableAndroidJar`
(NOTE: Turning incremental kotlin compilation ON can lead to tests running issues. Build Cache and Minimum SDK tuning are OK.).
  * Working directory: `$ProjectFileDir$`.
2. Run required JUnit tests as usual.
3. If tests are with configuration issues then run remote clean `mainframer.sh ./gradlew clean`, sync project locally and repeat step `2`.

### Pro: Any Android Studio / IntelliJ Configuration / Run from Terminal

Looks like you got it, right? We ❤️ IntelliJ because it allows you do things in ways you'd like them to be and then it'll do its part: launch & install apk, run tests from compiled classes and so on.

**Note to Android Developer Tools team and IntelliJ team**: please keep things as is so we could do crazy stuff like `mainframer` for Remote Builds and so on, thanks!

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

Note: it doesn't work very well with Unit tests & it's actually flaky, but speeds things up.

```
$ ./gradlew :app:assembleDebug -Pkotlin.incremental=true
```

## Change compression level

You can tune compression levels as you wish.

For example, if your network is very slow, you might consider increasing compression level up to `9` to download/upload less data.
If network is very fast (near to 1 Gb/s), you might consider disable compression at all by passing `0` as a value. Default values are `1`.

Configurable via `local.properties`:

```
# Optional.
remote_build.local_gzip_level=1

# Optional.
remote_build.remote_gzip_level=1
```

## Use best hardware available for remote machine

Performant CPU (more cores, higher frequency, more cache), fast SSD, fast RAM (~8 GB per user), fast network.

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
