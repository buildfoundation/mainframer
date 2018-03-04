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
touch "$BUILD_DIR/src/file1.txt"
touch "$BUILD_DIR/src/file2.txt"

# Run mainframer that creates 3 build files.
"$MAINFRAMER_EXECUTABLE" 'mkdir build && touch build/buildfile1.txt && touch build/buildfile2.txt'

# Make sure all src files exist on remote machine.
fileMustExistOnRemoteMachine "src/file1.txt" "(sync problem)"
fileMustExistOnRemoteMachine "src/file2.txt" "(sync problem)"

# Make sure all build files except ignored exist on local machine.
fileMustExistOnLocalMachine "build/buildfile1.txt" "(sync problem)"
fileMustExistOnLocalMachine "build/buildfile2.txt" "(sync problem)"

printTestEnded
