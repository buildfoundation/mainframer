#!/bin/bash
set -e

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Execute common pre-setup, include test functions.
source "$DIR/common.sh"

printTestStarted

# Create several files that should be synced to remote machine.
mkdir "$BUILD_DIR/src"
touch "$BUILD_DIR/src/file1.txt"
touch "$BUILD_DIR/src/file2.txt"
touch "$BUILD_DIR/src/file3.txt"

# Run mainframer that basically noop except syncing.
"$MAINFRAMER_EXECUTABLE" 'echo noop'

# Make sure files exist on remote machine after sync.
fileMustExistOnRemoteMachine "src/file1.txt" "(sync problem)"
fileMustExistOnRemoteMachine "src/file2.txt" "(sync problem)"
fileMustExistOnRemoteMachine "src/file3.txt" "(sync problem)"

printTestEnded
