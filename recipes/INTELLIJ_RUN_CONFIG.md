## IntelliJ Run Configuration Recipe

### IntelliJ Run Configuration Setup

1. `Run` → `Edit Configuration` → `+`.
* Select configuration type your typical run task is.
  * Name: something meaningful, like `*-remote-build`.
* Remove standard `Make`/etc step from `Before Launch` section (ha!).
* Create a step in `Before Launch` section for `Run External Tool`.
  * Name: use something meaningful, like `remote assembleDebug`.
  * Program: `bash`.
  * Parameters: `mainframer somebuildcommand with parameters` or any command you want\*.
  * Working directory: `$ProjectFileDir$`.

This also works for tests, you can compile tests on remote machine and then IntelliJ will simply run compiled code on your local machine.

Tip: you can override default run configuration options to to run tasks through mainframer by default.

**Note to IntelliJ team**: please keep things as is so we can do crazy stuff like `mainframer`!

---

Follow [mainframer IntelliJ Plugin development](https://github.com/gojuno/mainframer/issues/125).

### Performance Optimizations

#### Specify as Precise Build Task as Possible

For example, `assembleDebug` for specific module.

```
$ ./gradlew :app:assembleDebug
```

Instead of.

```
$ ./gradlew assembleDebug
```

#### Enable Build Daemon on Remote Machine

If your build system supports some kind of daemoning like [Gradle](https://docs.gradle.org/current/userguide/gradle_daemon.html) does or [Buck](https://facebook.github.io/watchman/docs/install.html), please enable it on remote machine to achieve maximum performance.
