#!/bin/bash
set -e

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Execute common pre-setup, include test functions.
source "$DIR/common.sh"

printTestStarted

# Create several files that should be synced to remote machine AND KEEPT after execution.
mkdir "$BUILD_DIR/src"
touch "$BUILD_DIR/src/file1.txt"
touch "$BUILD_DIR/src/file2.txt"
touch "$BUILD_DIR/src/file3.txt"

# Run mainframer that noops.
"$MAINFRAMER_EXECUTABLE" 'echo noop'

# Make sure files STILL exist on local machine after execution.
fileMustExistOnLocalMachine "src/file1.txt" "(sync problem)"
fileMustExistOnLocalMachine "src/file2.txt" "(sync problem)"
fileMustExistOnLocalMachine "src/file3.txt" "(sync problem)"

printTestEnded
