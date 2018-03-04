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

# Add a rule to ignore one local file and one remote file.
IGNORE_FILE="$1"
echo "src/file2.txt" > "$IGNORE_FILE"
echo "build/buildfile2.txt" >> "$IGNORE_FILE"

# Run mainframer that creates 3 build files.
"$REPO_DIR"/mainframer 'mkdir build && touch build/buildfile1.txt && touch build/buildfile2.txt && touch build/buildfile3.txt'

# Make sure all src files except ignored exist on remote machine.
fileMustExistOnRemoteMachine "src/file1.txt" "(sync problem)"
fileMustExistOnRemoteMachine "src/file3.txt" "(sync problem)"

# Make sure ignored src file does not exist on remote machine.
fileMustNotExistOnRemoteMachine "src/file2.txt" "(common ignore problem)"

# Make sure all build files except ignored exist on local machine.
fileMustExistOnLocalMachine "build/buildfile1.txt" "(sync problem)"
fileMustExistOnLocalMachine "build/buildfile3.txt" "(sync problem)"

# Make sure ignored build file does not exist on local machine.
fileMustNotExistOnLocalMachine "build/buildfile2.txt" "(common ignore problem)"

printTestEnded
