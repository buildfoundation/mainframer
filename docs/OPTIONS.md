# Options

All files are placed in the `.mainframer` directory
in the root directory of your project.

```
.
├── .mainframer
│   ├── config
│   ├── ignore
│   ├── localignore
│   └── remoteignore
└── mainframer.sh
└── your_project_files
```

## Configuration Files

### `config`

* Format: [properties](https://en.wikipedia.org/wiki/.properties).
* Required: yes.
* Commit into VCS: no.

```properties
remote_machine={REMOTE_MACHINE}
local_compression_level={LEVEL}
remote_compression_level={LEVEL}
```

* `remote_machine` — remote machine name from SSH config.
 * Required: yes.
* `local_compression_level` — compression level used to send files from local machine to remote one.
 * Required: no.
 * Possible values: `0`—`9`.
 * Default value: `1`.
* `remote_compression_level` — compression level used to send files from remote machine to local one.
 * Required: no.
 * Possible values: `0`—`9`.
 * Default value: `1`.
 
## Ignore Rule Files

To speedup execution you might want to **ignore large directories and files** not required for remote command such as `/.git` and so on. 

Please see our samples:

* [Gradle](samples/gradle)
* [Gradle Android](samples/gradle-android)
* [Rust](samples/rust)
* [Clang](samples/clang)
* [GCC](samples/gcc)
* [Maven](samples/mvn)
* [Go](samples/go)

#### `ignore`

Used both when transferring files from local machine to remote one and vice versa.

* Format: `rsync` exclusion rules.
 * Be aware that it is similar but not the same as `.gitignore`.
 * Refer to [rsync `Include/exclude pattern rules`](https://download.samba.org/pub/rsync/rsync.html).
* Required: no.
* Commit into VCS: yes.

#### `localignore`

Used only when transferring files from local machine to remote one.

* Format: `rsync` exclusion rules.
 * Be aware that it is similar but not the same as `.gitignore`.
 * Refer to [rsync `Include/exclude pattern rules`](https://download.samba.org/pub/rsync/rsync.html).
* Required: no.
* Commit into VCS: yes.

#### `remoteignore`

Used only when transferring files from remote machine to local one.

* Format: `rsync` exclusion rules.
 * Be aware that it is similar but not the same as `.gitignore`.
 * Refer to [rsync `Include/exclude pattern rules`](https://download.samba.org/pub/rsync/rsync.html).
* Required: no.
* Commit into VCS: yes.
