# Change Log

## Version 1.1.0

_2016_12_29_

This release is focused on migration from `tar` + `scp` to `rsync` which increases files transferring speed between machines, especially noticeable on connections slower than 1 Gbit/s. Please make sure you have `rsync` installed on both local and remote machines.

* [PR 35](https://github.com/gojuno/mainframer/pull/35): Replace tar+scp with rsync.

## Version 1.0.2

_2016_12_22_

* [PR 26](https://github.com/gojuno/mainframer/pull/26): Fix zero compression.

## Version 1.0.1

_2016_12_21_

* [PR 24](https://github.com/gojuno/mainframer/pull/24): Optional support for parallel `gzip` implementation `pigz`, both on local and remote machines.

## Version 1.0.0

_2016_12_20_

* 🚀 Basic support for remote Gradle builds (including Android Gradle projects).
