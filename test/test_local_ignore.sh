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
echo srcContent4 >"$BUILD_DIR/src/file4.txt"

# Add a rule to ignore two local files.
echo "src/file2.txt" > "$LOCAL_IGNORE_FILE"
echo "src/file3.txt" >> "$LOCAL_IGNORE_FILE"

# Run mainframer that noops.
"$MAINFRAMER_EXECUTABLE" 'echo noop'

# Make sure all files except ignored exist on remote machine.
remoteFileMustMatchLocal "src/file1.txt" "(sync problem)"
remoteFileMustMatchLocal "src/file4.txt" "(sync problem)"

# Make sure ignored files do not exist on remote machine.
fileMustNotExistOnRemoteMachine "src/file2.txt" "(local ignore problem)"
fileMustNotExistOnRemoteMachine "src/file3.txt" "(local ignore problem)"

printTestEnded
