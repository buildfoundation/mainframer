#!/bin/bash
set -xe

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
DIR_NAME="$( basename "$DIR")"
BUILD_DIR="$DIR/run"

function cleanBuildDirOnLocalMachine {
	rm -rf "$BUILD_DIR"
}

function cleanMainfamerDirOnRemoteMachine {
	ssh localhost "rm -rf ~/$DIR_NAME"
}

function fileMustExist {
	if [ ! -f "$1" ]; then
		echo "$1 does not exist $2"
		exit 1
	fi
}

# Clean build directory after run.
if [ "$CLEAN_BUILD_DIR_AFTER_RUN" == "true" ]; then
	trap cleanBuildDirOnLocalMachine EXIT
fi

# Clean build directories.
cleanBuildDirOnLocalMachine
cleanMainfamerDirOnRemoteMachine

# Create build directory.
mkdir "$BUILD_DIR"

# Copy mainframer.sh into build directory.
cp "$DIR/../mainframer.sh" "$BUILD_DIR/"

# Create local.properties that sets localhost as a "remote" build machine 
# (this is how we test, localhost should have sshd running on port 22 and ssh key of current user allowed).
echo "remote_build.machine=localhost" > "$BUILD_DIR/local.properties"
