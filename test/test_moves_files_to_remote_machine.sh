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

# Run mainframer that basically noop except syncing.
"$MAINFRAMER_EXECUTABLE" 'echo noop'

# Make sure files exist on remote machine after sync.
remoteFileMustMatchLocal "src/file1.txt" "(sync problem)"
remoteFileMustMatchLocal "src/file2.txt" "(sync problem)"
remoteFileMustMatchLocal "src/file3.txt" "(sync problem)"

printTestEnded
