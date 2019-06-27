# Configuration

Files can be put in two places.

* `${HOME}/.config/mainframer` or
  [`${XDG_CONFIG_HOME}`](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html)
  if available — the global configuration;
* `.mainframer` — the project configuration.

It is posssible to declare common configuration using the global one
and fine-tune it using the project one.

## `.mainframer`

The directory contains following files.

Name         | Required | Keep in VCS | Description
-------------|----------|-------------|------------
`config.yml` | Yes      | No          | Configuration options.
`ignore.yml` | No       | Yes         | Ignore rules for copying files.

## `.mainframer/config.yml`

```yaml
remote:
  host: "{SSH machine host}"
push:
  compression: {level}
pull:
  compression: {level}
```

Name               | Required | Value                  | Default | Description
-------------------|----------|------------------------|---------|------------------
`remote.host`      | Yes      | `string`               | —       | Remote machine name from SSH config or hostname / IP address.
`push.compression` | No       | `0..9`                 | `0`     | Compression level used to copy files from local machine to remote one.
`pull.compression` | No       | `0..9`                 | `0`     | Compression level used to copy files from remote machine to local one.
`pull.mode`        | No       | `serial` \| `parallel` | `serial`| Pull mode. `serial` pulls once remote command has finished, `parallel` pulls in parallel to remote command execution.

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

Name   | Description                                                   | Suggestions
-------|---------------------------------------------------------------|------------
`push` | Patterns used to copy files from local machine to remote one. | Build results.
`pull` | Patterns used to copy files from remote machine to local one. | Source code.
`both` | Patterns used to copy files both ways.                        | VCS, IDE-related directories.

Ignore patterns are inherited from underlying `rsync`.
Please refer to [`rsync` documentation](https://download.samba.org/pub/rsync/rsync.html) —
see the _Include/Exclude Pattern Rules_ section.
The format is very similar to `.gitignore`.

