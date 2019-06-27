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

# Tilde expands on remote machine during ssh.
# shellcheck disable=SC2088
PRIVATE_REMOTE_BUILD_ROOT_DIR="~/mainframer"
PRIVATE_REMOTE_BUILD_DIR="${PRIVATE_REMOTE_BUILD_ROOT_DIR}${DIR}/$PRIVATE_BUILD_DIR_NAME"

### Used by tests (shellcheck raises SC2034)
BUILD_DIR="$DIR/$PRIVATE_BUILD_DIR_NAME"

CONFIG_FILE="$BUILD_DIR/.mainframer/config.yml"

# shellcheck disable=SC2034
LOCAL_IGNORE_FILE="$BUILD_DIR/.mainframer/localignore"

# shellcheck disable=SC2034
REMOTE_IGNORE_FILE="$BUILD_DIR/.mainframer/remoteignore"

# shellcheck disable=SC2034
COMMON_IGNORE_FILE="$BUILD_DIR/.mainframer/ignore"

# TODO test both debug and release builds.
# shellcheck disable=SC2034
MAINFRAMER_EXECUTABLE="$DIR/../target/debug/mainframer"
###

function buildMainframer {
    echo "Building Mainframer..."
    pushd "$DIR/.." > /dev/null
    cargo build
    popd > /dev/null
}

function printTestStarted {
	echo ""
	test_name=$(basename "$0")
	echo "-------- TEST STARTED (pull mode = '$TEST_PULL_MODE') $test_name -------- "
}

function printTestEnded {
	echo ""
	test_name=$(basename "$0")
	echo "-------- TEST ENDED (pull mode = '$TEST_PULL_MODE') $test_name -------- "
}

function cleanBuildDirOnLocalMachine {
	rm -rf "$BUILD_DIR"
}

function cleanMainfamerDirOnRemoteMachine {
    # $PRIVATE_REMOTE_BUILD_ROOT_DIR should expand locally.
    # shellcheck disable=SC2029
    ssh "$TEST_REMOTE_MACHINE" "rm -rf $PRIVATE_REMOTE_BUILD_ROOT_DIR"
}

function localFileMustMatchRemote {
    local -r file_name="$1"
    local -r error_message="$2"

    local -r local_file="$BUILD_DIR/$file_name"
    # shellcheck disable=SC2088
    local -r remote_file="~/mainframer/$BUILD_DIR/$file_name"

    if [[ ! -f "$local_file" ]]; then
        echo "$local_file does not exist on local machine $error_message" >&2
        exit 1
    else
        local -r tmp_file="$(mktemp)"
        scp "$TEST_REMOTE_MACHINE:$remote_file" "$tmp_file"
        local -r actual_shasum=$("$DIR/calculate_shasum" "$tmp_file")
        rm -f "$tmp_file"
        "$DIR/verify_shasum" "$actual_shasum" "$local_file"
    fi
}

function fileMustNotExistOnLocalMachine {
	local_file="$BUILD_DIR/$1"
	if [ -f "$local_file" ]; then
		echo "$local_file exists on local machine $2"
		exit 1
	fi
}

function remoteFileMustMatchLocal {
    local -r file_name="$1"
    local -r error_message="$2"

    local -r local_file="$BUILD_DIR/$file_name"
    # shellcheck disable=SC2088
    local -r remote_file="~/mainframer/$BUILD_DIR/$file_name"

    if [[ ! -f "$local_file" ]]; then
        echo "$local_file does not exist on local machine $error_message" >&2
        exit 1
    else
        local -r tmp_file="$(mktemp)"
        scp "$TEST_REMOTE_MACHINE:$remote_file" "$tmp_file"
        local -r actual_shasum=$("$DIR/calculate_shasum" "$local_file")
        "$DIR/verify_shasum" "$actual_shasum" "$tmp_file"
        rm -f "$tmp_file"
    fi
}

function fileMustNotExistOnRemoteMachine {
    # $PRIVATE_REMOTE_BUILD_ROOT_DIR should expand locally.
    # shellcheck disable=SC2029
    if ssh "$TEST_REMOTE_MACHINE" "test -f $PRIVATE_REMOTE_BUILD_DIR/$1"; then
        echo "$PRIVATE_REMOTE_BUILD_DIR/$1 exists on remote machine $2"
        exit 1
    fi
}

function createConfig {
    {
        echo -e "remote:\\n"
        echo -e "  host: \"$TEST_REMOTE_MACHINE\""
        echo -e "pull:\\n"
        echo -e "  mode: \"$TEST_PULL_MODE\""
    } > "$CONFIG_FILE"
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
createConfig

# Set build directory as "working dir".
pushd "$BUILD_DIR"
