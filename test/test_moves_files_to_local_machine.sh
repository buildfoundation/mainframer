#!/bin/bash
set -e

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Execute common pre-setup, include test functions.
# shellcheck disable=SC1090
source "$DIR/common.sh"

printTestStarted

# Create several files that should be synced to remote machine.
mkdir "$BUILD_DIR/src"
echo srcContent1 >"$BUILD_DIR/src/file1.txt"
echo srcContent2 >"$BUILD_DIR/src/file2.txt"
echo srcContent3 >"$BUILD_DIR/src/file3.txt"

# Run mainframer that creates "build" result file that should be synced back to local machine.
"$MAINFRAMER_EXECUTABLE" 'mkdir build && echo buildContent1 > build/buildresult.txt'

# Make sure files exist on local machine after sync.
localFileMustMatchRemote "build/buildresult.txt" "(sync problem)"

printTestEnded
