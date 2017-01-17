## Sample Buck project

Most interesting for you here is not the source code, but configs in [`.mainframer`](.mainframer) folder.

This is a very common setup for basically any Buck project (including multimodule), but of course you might need to tune some ignore configs for your project.

### How to build

```bash
$ bash mainframer.sh buck build sample
```

Or any other Buck task you want, btw it integrates with IntelliJ pretty easily! TODO add link to docs.

### Requirements

* Buck installed on remote machine.
* JDK 6+ installed on remote machine. (Because sample is in Java)
