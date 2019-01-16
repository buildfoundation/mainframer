# Mainframer

A tool that executes a command on a remote machine while syncing files back and forth.
The process is known as remote execution (in general) and remote build (in particular cases).

Mainframer helps to move heavy operations (like compiling the source code)
from a personal computer to a dedicated machine. This way you, as a developer,
can use your machine for changing the source code or browsing the documentation
without constant freezes and hearing jet engine-like sounds caused by the build process.
The execution itself is not limited and can be applied to actions
like encoding audio and video, batch processing and more.

It works via pushing files to the remote machine, executing the command there
and pulling results to the local machine.

```console
$ mainframer ./gradlew build
Sync local → remote machine...
:build
BUILD SUCCESSFUL
Sync remote → local machine...

$ java -jar build/libs/sample.jar
This program was built on a remote machine!
```

## State of the Project

* [`2.x`](https://github.com/gojuno/mainframer/tree/v2.1.0) — the stable version, recommended for production use.
* `3.x` — the future version, in active development at this point.
    * Mainframer is going to be [a system-wide tool](https://github.com/gojuno/mainframer/issues/185).
    * Mainframer is [rewritten in Rust](https://github.com/gojuno/mainframer/issues/191).
    * Mainframer will speed up most of existing workflows by [syncing during remote command execution](https://github.com/gojuno/mainframer/issues/188).

## Documentation

### Getting Started

* [Remote Machine](docs/getting-started/remote-machine.md)
* [Local Machine](docs/getting-started/local-machine.md)

### Configuration

* [Description](docs/configuration/description.md)
* [Examples](docs/configuration/examples.md)

### Integrations

* [IntelliJ IDEA](docs/integration/intellij-idea.md)

## Transfer Notice

In October 2018, Mainframer was transferred from
[github/gojuno](https://github.com/gojuno) to [github/buildfoundation](https://github.com/buildfoundation).
It was a friendly agreement between Juno Inc. and Build Foundation after
the core developer of the project [@artem-zinnatullin](https://github.com/artem-zinnatullin) left the company.

The motivation for the transfer is to have a neutral space for the future work on the project.

## License

```
Copyright 2018 Mainframer Maintainers.

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
