#!/bin/bash
set -e

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Execute common pre-setup, include test functions.
export OVERRIDDEN_BUILD_DIR_NAME="folder name"

# shellcheck disable=SC1090
source "$DIR/common.sh"

printTestStarted

# Create several files that should be synced to remote machine.
mkdir "$BUILD_DIR/src"
echo srcContent1 > "$BUILD_DIR/src/file1.txt"
echo srcContent2 > "$BUILD_DIR/src/file2.txt"

# Run mainframer that creates 3 build files.
"$MAINFRAMER_EXECUTABLE" 'mkdir build && echo buildContent1 > build/buildfile1.txt && echo buildContent2 > build/buildfile2.txt'

# Make sure all src files exist on remote machine.
remoteFileMustMatchLocal "src/file1.txt" "(sync problem)"
remoteFileMustMatchLocal "src/file2.txt" "(sync problem)"

# Make sure all build files except ignored exist on local machine.
localFileMustMatchRemote "build/buildfile1.txt" "(sync problem)"
localFileMustMatchRemote "build/buildfile2.txt" "(sync problem)"

printTestEnded
