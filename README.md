# Mainframer

Tool that allows you to move build process from a local machine to a remote one.

Remote machine ought to be much faster than a laptop. 
With `mainframer` you can free up your local machine for better things —
like editing source code in your IDE without lags and
freezes, being able to actually _use_ your computer when the build is happening.

## Table of contents

- [State of Project Development](#state-of-project-development)
- [Supported Workflows](#supported-workflows)
- [Setup](#setup)
- [Demo](#demo)
- [3rd-party IntelliJ Mainframer Plugin](#3rd-party-intellij-mainframer-plugin)
- [How to Run Tests (for contributors)](#how-to-run-tests-for-contributors)
- [License](#license)

## State of Project Development

Version `3.x` is in active development, the stable version is `2.x` and recommended for production use.

You can view `2.x` version sources and docs using [this Git tag](https://github.com/gojuno/mainframer/tree/v2.1.0).

There are lots of changes between `2.x` and `3.x`, but key subset is the following:

- Mainframer will work as system-wide CLI tool #[185](https://github.com/gojuno/mainframer/issues/185)
- Mainframer is now written in Rust #[191](https://github.com/gojuno/mainframer/issues/191)
- Mainframer will speed up most of existing workflows by syncing during remote command execution #[188](https://github.com/gojuno/mainframer/issues/188)

You can track `3.x` development progress using the [3.0.0 milestone](https://github.com/gojuno/mainframer/milestone/6).

## Supported Workflows

`mainframer` supports basically anything you can execute as a command.
It will sync files to remote machine, execute a command and sync files back.

We have quite a bunch of samples showing off some practical applications.

* [Gradle](samples/gradle)
* [Gradle Android](samples/gradle-android)
* [Rust](samples/rust)
* [Clang](samples/clang)
* [GCC](samples/gcc)
* [Maven](samples/mvn)
* [Buck](samples/buck)
* [Go](samples/go)

## Setup

* [Remote machine](docs/SETUP_REMOTE.md)
* [Local machine](docs/SETUP_LOCAL.md)
* [Configuration](docs/CONFIGURATION.md)

## Demo

[![asciicast demo](samples/demo.png)](https://asciinema.org/a/101327)

## 3rd-party IntelliJ Mainframer Plugin

Guys from [@elpassion](https://github.com/elpassion) maintain [IntelliJ Plugin for Mainframer](https://github.com/elpassion/mainframer-intellij-plugin) that makes integration of Mainframer with any IntelliJ project better. 

We highly recommend the plugin if you use IntelliJ with Mainframer!

### How to Run Tests (for contributors)

Dependencies: Bash, Docker.

```console
$ ci/build.sh
```

## License

```
Copyright 2017 Juno, Inc.

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
