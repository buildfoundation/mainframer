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
touch "$BUILD_DIR/src/file4.txt"

# Add a rule to ignore two local files.
echo "src/file2.txt" > "$LOCAL_IGNORE_FILE"
echo "src/file3.txt" >> "$LOCAL_IGNORE_FILE"

# Run mainframer.sh that noops.
bash "$BUILD_DIR"/mainframer.sh 'echo noop'

# Make sure all files except ignored exist on remote machine.
fileMustExistOnRemoteMachine "src/file1.txt" "(sync problem)"
fileMustExistOnRemoteMachine "src/file4.txt" "(sync problem)"

# Make sure ignored files do not exist on remote machine.
fileMustNotExistOnRemoteMachine "src/file2.txt" "(local ignore problem)"
fileMustNotExistOnRemoteMachine "src/file3.txt" "(local ignore problem)"

printTestEnded
