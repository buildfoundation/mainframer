#!/bin/bash
set -xe

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
DIR_NAME="$( basename "$DIR")"

# This is how we test, localhost should have sshd running on port 22 and ssh key of current user allowed.
PRIVATE_TEST_REMOTE_MACHINE="localhost"

PRIVATE_BUILD_DIR_NAME="run"
PRIVATE_REMOTE_BUILD_DIR="~/$PRIVATE_BUILD_DIR_NAME"

# Should be used by tests.
BUILD_DIR="$DIR/$PRIVATE_BUILD_DIR_NAME"

function cleanBuildDirOnLocalMachine {
	rm -rf "$BUILD_DIR"
}

function cleanMainfamerDirOnRemoteMachine {
	ssh "$PRIVATE_TEST_REMOTE_MACHINE" "rm -rf $PRIVATE_REMOTE_BUILD_DIR"
}

function fileMustExistOnLocalMachine {
	if [ ! -f "$1" ]; then
		echo "$1 does not exist $2"
		exit 1
	fi
}

function fileMustExistOnRemoteMachine {
	if [ ! ssh "$PRIVATE_TEST_REMOTE_MACHINE" "test -f $PRIVATE_REMOTE_BUILD_DIR/$1" ]; then
		echo "$PRIVATE_REMOTE_BUILD_DIR/$1 does not exist on remote machine $2"
	fi
}

# Clean build directory after run.
if [ ! "$CLEAN_BUILD_DIRS_AFTER_RUN" == "false" ]; then
	trap "cleanBuildDirOnLocalMachine ; cleanMainfamerDirOnRemoteMachine" EXIT
fi

# Clean build directories.
cleanBuildDirOnLocalMachine
cleanMainfamerDirOnRemoteMachine

# Create build directory.
mkdir "$BUILD_DIR"

# Copy mainframer.sh into build directory.
cp "$DIR/../mainframer.sh" "$BUILD_DIR/"

# Create local.properties that sets remote build machine for the test.
echo "remote_build.machine=$PRIVATE_TEST_REMOTE_MACHINE" > "$BUILD_DIR/local.properties"

# TODO: Remove these files once they'll become non-required and create separate tests for exclude strategies.
touch "$BUILD_DIR/.mainframerignorelocal"
touch "$BUILD_DIR/.mainframerignoreremote"