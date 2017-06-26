#!/bin/bash
set -e

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Execute common pre-setup, include test functions.
source "$DIR/common.sh"

printTestStarted

# Add a rule to ignore two remote files.
echo "build/file2.txt" > "$REMOTE_IGNORE_FILE"
echo "build/file3.txt" >> "$REMOTE_IGNORE_FILE"

# Run mainframer that creates 4 files on remote machine.
bash "$REPO_DIR"/mainframer 'mkdir build && touch build/file1.txt && touch build/file2.txt && touch build/file3.txt && touch build/file4.txt'

# Make sure all files except ignored exist on local machine.
fileMustExistOnLocalMachine "build/file1.txt" "(sync problem)"
fileMustExistOnLocalMachine "build/file4.txt" "(sync problem)"

# Make sure ignored files do not exist on local machine.
fileMustNotExistOnLocalMachine "build/file2.txt" "(remote ignore problem)"
fileMustNotExistOnLocalMachine "build/file3.txt" "(remote ignore problem)"

printTestEnded
