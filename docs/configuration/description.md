Files are placed in the `.mainframer` directory.
The final configuration is the combination of files placed in:

* `${HOME}/.mainframer` — the global configuration;
* `.mainframer` — the project configuration.

It is posssible to share the configuration using the global one
and fine-tune it using the project one.

## `.mainframer`

The directory contains following files.

Name         | Optional | Keep in VCS | Description
-------------|----------|-------------|------------
`config.yml` | No       | No          | Configuration options.
`ignore.yml` | Yes      | Yes         | Ignore rules for copying files.

## `.mainframer/config.yml`

```yaml
remote:
  name: "{SSH machine name}"
push:
  compression: {level}
pull:
  compression: {level}
```

Name               | Optional | Value   | Default | Description
-------------------|----------|---------|---------|------------------
`remote.name`      | No       | `string`| —       | Remote machine name from SSH config.
`push.compression` | Yes      | `0..9`  | `0`     | Compression level used to copy files from local machine to remote one.
`pull.compression` | Yes      | `0..9`  | `0`     | Compression level used to copy files from remote machine to local one.

Compression level is inherited from underlying `rsync`
which uses [`zlib` values](https://www.zlib.net/manual.html):

> The compression level must be between `0` and `9`:
> `1` gives best speed, `9` gives best compression, `0` gives no compression at all
> (the input data is simply copied a block at a time).

## `.mainframer/ignore.yml`

```yaml
push:
  - "pattern"
  - "..."
pull:
  - "pattern"
  - "..."
both:
  - "pattern"
  - "..."
```

Name   | Description | Suggestions
-------|-------------|------------
`push` | Patterns used to copy files from local machine to remote one. | Build results.
`pull` | Patterns used to copy files from remote machine to local one. | Source code.
`both` | Patterns used to copy files both ways. | VCS, IDE-related directories.

Ignore patterns are inherited from underlying `rsync`.
Please refer to [`rsync` documentation](https://download.samba.org/pub/rsync/rsync.html) —
see the _Include/Exclude Pattern Rules_ section.
The format is very similar to `.gitignore`.

