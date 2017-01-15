## Sample GCC project

Most interesting for you here is not the source code, but configs in [`.mainframer`](.mainframer) folder.

This is a very common setup for GCC project, but of course you might need to tune some ignore configs for your project.

### How to build

```bash
$ bash mainframer.sh gcc -Wall sample.c -o sample
```

Or any other GCC build task you want.

### Requirements

GCC installed on remote machine.
