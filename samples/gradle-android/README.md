## Sample Gradle Android project

## Demo

[![Video](demo.png)](https://youtu.be/xysQXMaPaGw "mainframer Android Sample")

Most interesting for you here is not the source code, but configs in [`.mainframer`](.mainframer) folder.

This is a very common setup for basically any Gradle Android project (including multimodule), but of course you might need to tune some ignore configs for your project.

### How to build

```bash
$ bash mainframer.sh ./gradlew build
```

Or any other Gradle task you want, btw it integrates with Android Studio and IntelliJ pretty easily!

### Requirements

* JDK 8 installed on remote machine.
* Android SDK with required components installed on remote machine.

# Android Studio Configuration to Run APK

1. `Run` → `Edit Configuration` → `+`.
* Select `Android App`.
  * Name: something meaningful, like `*-remote-build`.
* Remove `Gradle-aware Make` from `Before Launch` section (ha!).
* Create a step in `Before Launch` section for `Run External Tool`.
  * Name: use something meaningful, like `remote assembleDebug`.
  * Program: `bash`.
  * Parameters: `mainframer.sh ./gradlew :app:assembleDebug` or any command you want\*.
  * Working directory: `$ProjectFileDir$`.

Note: local Gradle sync is required sometimes because this is how Android Studio determines resulting APK name even though we’ll build it on a remote machine.

\* Please check to target your build as precise as you can. If you do `assembleDebug` it will compile all `debug` builds. If you i.e. have 2 flavors (i.e. Dev/Prod from the google multidex tutorial) it will compile both. In this case it's better to target only one - `:app:assembleDevDebug` or `:app:assembleProdDebug`.

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
