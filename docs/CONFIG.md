# Config Files

All files are placed in the `.mainframer` directory.

## `config`

General options.

Required. Should not be put to VCS.

Has a following format.

```properties
remote_machine={REMOTE_MACHINE}
local_compression_level={LEVEL}
remote_compression_level={LEVEL}
```

* `remote_machine` — remote machine hostname. Required.
* `local_compression_level` — compression level used to send files from local machine to remote one. Optional, `1` by default.
* `remote_compression_level` — compression level used to send files from remote machine to local one. Optional, `1` by default.

## `ignore`

Ignore rules used in process of transferring files from local machine to remote one and vice versa.

Optional. Should be put to VCS.

Has a following format.

```
.directory
*/directory/
directory/file
```

## `localignore`

Ignore rules used in process of transferring files from local machine to remote one.

Optional. Should be put to VCS.

Has a following format.

```
.directory
*/directory/
directory/file
```

## `remoteignore`

Ignore rules used in process of transferring files from remote machine to local one.

Optional. Should be put to VCS.

Has a following format.

```
.directory
*/directory/
directory/file
```
