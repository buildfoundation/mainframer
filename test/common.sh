#!/bin/bash
set -e

if [ "$DEBUG_MODE_FOR_ALL_TESTS" == "true" ]; then
	set -x
fi

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# This is how we test, localhost should have sshd running on port 22 and ssh key of current user allowed.
TEST_REMOTE_MACHINE="localhost"

if [ -z "$OVERRIDDEN_BUILD_DIR_NAME" ]; then
    PRIVATE_BUILD_DIR_NAME=$(printf '%q' "run")
else
    echo "Overriding folder name for the test to '$OVERRIDDEN_BUILD_DIR_NAME'"
    PRIVATE_BUILD_DIR_NAME=$(printf '%q' "$OVERRIDDEN_BUILD_DIR_NAME")
fi

# shellcheck disable=SC2088, tilde will expand during ssh.
PRIVATE_REMOTE_BUILD_ROOT_DIR="~/mainframer"
PRIVATE_REMOTE_BUILD_DIR="$PRIVATE_REMOTE_BUILD_ROOT_DIR/$PRIVATE_BUILD_DIR_NAME"

# Should be used by tests.
BUILD_DIR="$DIR/$PRIVATE_BUILD_DIR_NAME"
CONFIG_FILE="$BUILD_DIR/.mainframer/config"
# shellcheck disable=SC2034, used by tests.
LOCAL_IGNORE_FILE="$BUILD_DIR/.mainframer/localignore"
# shellcheck disable=SC2034, used by tests.
REMOTE_IGNORE_FILE="$BUILD_DIR/.mainframer/remoteignore"
# shellcheck disable=SC2034, used by tests.
COMMON_IGNORE_FILE="$BUILD_DIR/.mainframer/ignore"
REMOTE_MACHINE_PROPERTY="remote_machine"

# TODO test both debug and release builds.
# shellcheck disable=SC2034, used by tests.
MAINFRAMER_EXECUTABLE="$DIR/../target/debug/mainframer"

function buildMainframer {
    echo "Building Mainframer..."
    pushd "$DIR/.." > /dev/null
    cargo build
    popd > /dev/null
}

function printTestStarted {
	echo ""
	test_name=$(basename "$0")
	echo "-------- TEST STARTED $test_name -------- "
}

function printTestEnded {
	echo ""
	test_name=$(basename "$0")
	echo "-------- TEST ENDED $test_name -------- "	
}

function cleanBuildDirOnLocalMachine {
	rm -rf "$BUILD_DIR"
}

function cleanMainfamerDirOnRemoteMachine {
        # shellcheck disable=SC2029, $PRIVATE_REMOTE_BUILD_ROOT_DIR should expand locally.
	ssh "$TEST_REMOTE_MACHINE" "rm -rf $PRIVATE_REMOTE_BUILD_ROOT_DIR"
}

function fileMustExistOnLocalMachine {
	local_file="$BUILD_DIR/$1"
	if [ ! -f "$local_file" ]; then
		echo "$local_file does not exist on local machine $2"
		exit 1
	fi
}

function fileMustNotExistOnLocalMachine {
	local_file="$BUILD_DIR/$1"
	if [ -f "$local_file" ]; then
		echo "$local_file exists on local machine $2"
		exit 1
	fi
}

function fileMustExistOnRemoteMachine {
        # shellcheck disable=SC2029, $PRIVATE_REMOTE_BUILD_ROOT_DIR should expand locally.
	if ssh "$TEST_REMOTE_MACHINE" "test -f $PRIVATE_REMOTE_BUILD_DIR/$1"; then
		echo "$PRIVATE_REMOTE_BUILD_DIR/$1 does not exist on remote machine $2"
		exit 1
	fi
}

function fileMustNotExistOnRemoteMachine {
	# shellcheck disable=SC2029, $PRIVATE_REMOTE_BUILD_ROOT_DIR should expand locally.
	if ssh "$TEST_REMOTE_MACHINE" "test -f $PRIVATE_REMOTE_BUILD_DIR/$1"; then
		echo "$PRIVATE_REMOTE_BUILD_DIR/$1 exists on remote machine $2"
		exit 1
	fi
}

function setTestRemoteMachineInConfig {
	echo "$REMOTE_MACHINE_PROPERTY=$TEST_REMOTE_MACHINE" > "$CONFIG_FILE"
}

# Clean build directory after run.
if [ ! "$CLEAN_BUILD_DIRS_AFTER_RUN" == "false" ]; then
	trap "cleanBuildDirOnLocalMachine ; cleanMainfamerDirOnRemoteMachine" EXIT
fi

buildMainframer

# Clean build directories.
cleanBuildDirOnLocalMachine
cleanMainfamerDirOnRemoteMachine

# Create build directory.
mkdir -p "$BUILD_DIR/.mainframer"

# Create config that sets remote build machine for the test.
setTestRemoteMachineInConfig

# Set build directory as "working dir".
pushd "$BUILD_DIR"
