# Options

All files are placed in the `.mainframer` directory
on the same file tree level as `mainframer.sh`.

```
.
├── .mainframer
│   ├── config
│   ├── ignore
│   ├── localignore
│   └── remoteignore
└── mainframer.sh
```

## Configuration

* Format: [properties](https://en.wikipedia.org/wiki/.properties).
* Required: yes.
* Put in VCS: yes.

### `config`

```properties
remote_machine={REMOTE_MACHINE}
local_compression_level={LEVEL}
remote_compression_level={LEVEL}
```

* `remote_machine` — remote machine hostname.
 * Required: yes.
* `local_compression_level` — compression level used to send files from local machine to remote one.
 * Required: no.
 * Possible values: `0`—`9`.
 * Default value: `1`.
* `remote_compression_level` — compression level used to send files from remote machine to local one.
 * Required: no.
 * Possible values: `0`—`9`.
 * Default value: `1`.
 
## Ignore

* Format: `rsync` exclusion rules.
 * Be aware that it is similar but not the same as `.gitignore`.
* Required: no.
* Put in VCS: no.

### `ignore`

Used when transferring files from local machine to remote one and vice versa.

### `localignore`

Used only when transferring files from local machine to remote one.

### `remoteignore`

Used only when transferring files from remote machine to local one.
