## Sample Go project

Most interesting for you here is not the source code, but configs in [`.mainframer`](.mainframer) folder.

This is a very common setup for Go project, but of course you might need to tune some ignore configs for your project.

### How to build

```bash
$ bash mainframer.sh eval "export GOPATH=\`pwd\` && go install gojuno.com/mainframer/sample"
```

Or any other Go build command you want.

To properly export `GOPATH` env variable on remote machine we use `eval`.

### Requirements

Go tools installed on remote machine.
