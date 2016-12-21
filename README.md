# mainframer

Blip-blop, it's `mainframer`, simple (at the moment lol) script that allows you to move build process from your local machine to some remote one. This way you can free up your local machine for better things — like editing source code in your IDE without lags, freezes and running into swap, being able to actually _use_ your computer when the build is happening somewhere else.

**For now** `mainframer` implemented to work with [Gradle][Gradle] (mostly Android-focused) projects, but its design is pretty universal, so we’re open for other things like [`go`][Go] in particular, but please file an issue before contributing support for other build systems, thanks! 

**In v2.0.0** we're [planning](https://github.com/gojuno/mainframer/issues/19) to make `mainframer` more universal and customizable so you could use it with other build systems.

## Demo

[![Video](video_preview.png)](https://youtu.be/xysQXMaPaGw "Remote Build — mainframer v1.0.0")

# Before

#### Build Machine Setup

See [instruction](HOW_TO_SETUP_REMOTE_BUILD_MACHINE.md). It’s actually very easy: create a Linux user for each real user, install JDK and Android SDK, that's it!

#### Info You’ll Need to Start

* `BUILD_MACHINE_NAME` — build machine hostname.
* `BUILD_MACHINE_IP_OR_HOSTNAME` — build machine IP or hostname. Something like `172.42.42.42` or `woooot.remotebuild.company.com`.
* `BUILD_MACHINE_USERNAME` — your build machine user. Something like `john_doe`.

# Authentication

### Configure SSH

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

# Local Configuration

1. Download [`mainframer.sh`](mainframer.sh) and save it in your project, most likely put it to VCS so you could sync changes across all team members.

  We recommend you to subscribe to changes in this repo somehow (follow it on GitHub / watch for tweets of its maintainers / etc). This will allow you to apply best practises we found to make your Remote Build better, faster and safer.

2. Update your `local.properties` file.

  ```
  $ echo "remote_build.machine={BUILD_MACHINE_NAME}" >> local.properties
  ```

3. And finally you can test the build.

  ```
  $ bash mainframer.sh ./gradlew assembleDebug
  ```

# Android Studio Configuration to Run APK

1. `Run` → `Edit Configuration` → `+`.
* Select `Android App`.
  * Name: something meaningful, like `*-remote-build`.
* Remove `Gradle-aware Make` from `Before Launch` section (ha!).
* Create a step in `Before Launch` section for `Run External Tool`.
  * Name: use something meaningful, like `remote assembleDebug`.
  * Program: `bash`.
  * Parameters: `mainframer.sh ./gradlew :app:assembleDebug` or any command you want.
  * Working directory: `$ProjectFileDir$`.

Note: local Gradle sync is required sometimes because this is how Android Studio determines resulting APK name even though we’ll build it on a remote machine.

# Android Studio Configuration to Run JUnit tests

1. `Run` → `Edit Configuration` → `Defaults` → `JUnit`.
* Remove default entry from `Before Launch` section.
* Create a step in `Before Launch` section for `Run External Tool`.
  * Name: use something meaningful, like `remote compileDebugUnitTestSources`.
  * Program: `bash`.
  * Parameters: `mainframer.sh ./gradlew compileDebugUnitTestSources mockableAndroidJar`
(NOTE: Turning incremental kotlin compilation ON can lead to test run issues. Build Cache and Minimum SDK tunings are OK.).
  * Working directory: `$ProjectFileDir$`.
2. Run required JUnit tests as usual.
3. If tests are failing with configuration issues, run remote clean: `mainframer.sh ./gradlew clean`, then run Gradle sync locally and repeat step `2`.

### Pro: Any Android Studio / IntelliJ Configuration / Run from Terminal

Looks like you got it, right? You can run any command on remote machine and grab its results to local one.

We ❤️ IntelliJ because it allows you do things in ways you’d like them to be and then it’ll do its part: launch and install APK, run tests from compiled classes and so on.

**Note to Android Developer Tools team and IntelliJ team**: please keep things as is so we can do crazy stuff like `mainframer`!

# Performance Optimizations

## Specify Precise Task

For example, `assembleDebug` for specific module.

```
$ ./gradlew :app:assembleDebug
```

Instead of.

```
$ ./gradlew assembleDebug
```

## Change Minimum SDK

```
$ ./gradlew :app:assembleDebug -Pmin.sdk.version=21
```

See [detailed explanation](https://artemzin.com/blog/minsdk-without-flavors/).

## Enable Build Cache of Android Gradle Plugin

```
$ ./gradlew :app:assembleDebug -Pandroid.enableBuildCache=true
```

## Enable Kotlin Incremental Compilation

Note: it doesn’t work very well with Unit tests and it’s actually flaky at this point of time, but speeds things up.

```
$ ./gradlew :app:assembleDebug -Pkotlin.incremental=true
```

## Change Compression Level

You can tune compression levels as you wish. Default compression levels are `1`.

* If network is slow, you might consider increasing compression levels up to `9` to exchange less data.
* If network is fast (near to 1 Gb/s), you might consider disabling compression at all by passing `0` as a value.

Configurable via `local.properties`:

```
remote_build.local_gzip_level=1
remote_build.remote_gzip_level=1
```

## Use The Best Hardware Available for Remote Machine

Performant CPU (more cores, higher frequency, more cache), fast SSD, fast RAM (~8 GB per user), fast network will help a lot.

We’ve created this with a simple motivation in mind — it is always easier to beef up a single headless Linux-based PC hardware unit instead of paying a ridiculous amount of money to some companies (cough-cough) still using mobile CPUs in their laptops. Use and abuse this concept as you wish.

# License

```
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
```

[Gradle]: https://gradle.org/
[Go]: https://golang.org
