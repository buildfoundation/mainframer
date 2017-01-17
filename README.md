# mainframer

Blip-blop, it's `mainframer`, a simple script that allows you to move build process
from your local machine to some remote one. This way you can free up your local machine
for better things — like editing source code in your IDE without lags,
freezes and running into swap, being able to actually _use_ your computer
when the build is happening somewhere else.

## Demo

[![Video](video_preview.png)](https://youtu.be/xysQXMaPaGw "Remote Build — mainframer v1.0.0")

## Application

`mainframer` supports basically anything you can execute as a command.
The idea is quite simple. The script will sync files to remote machine,
execute a command and sync files back.

We have quite a bunch of samples showing off some practical applications.

* [Clang](samples/clang)
* [GCC](samples/gcc)
* [Go](samples/go)
* [Gradle](samples/gradle)
* [Maven](samples/mvn)
* [Rust](samples/rust)

## Setup

* [Remote machine](docs/SETUP_REMOTE.md)
* [Local machine](docs/SETUP_LOCAL.md)
* [Configuration files](docs/CONFIG.md)

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
