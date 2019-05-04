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
echo srcContent1 > "$BUILD_DIR/src/file1.txt"
echo srcContent2 > "$BUILD_DIR/src/file2.txt"
echo srcContent3 > "$BUILD_DIR/src/file3.txt"

# Add a rule to ignore one local file and one remote file.
echo "src/file2.txt" > "$COMMON_IGNORE_FILE"
echo "build/buildfile2.txt" >> "$COMMON_IGNORE_FILE"

# Run mainframer that creates 3 build files.
"$MAINFRAMER_EXECUTABLE" 'mkdir build && echo content1 > build/buildfile1.txt && echo content2 > build/buildfile2.txt && echo content3 > build/buildfile3.txt'

# Make sure all src files except ignored exist on remote machine.
remoteFileMustMatchLocal "src/file1.txt" "(sync problem)"
remoteFileMustMatchLocal "src/file3.txt" "(sync problem)"

# Make sure ignored src file does not exist on remote machine.
fileMustNotExistOnRemoteMachine "src/file2.txt" "(common ignore problem)"

# Make sure all build files except ignored exist on local machine.
localFileMustMatchRemote "build/buildfile1.txt" "(sync problem)"
localFileMustMatchRemote "build/buildfile3.txt" "(sync problem)"

# Make sure ignored build file does not exist on local machine.
fileMustNotExistOnLocalMachine "build/buildfile2.txt" "(common ignore problem)"

printTestEnded
