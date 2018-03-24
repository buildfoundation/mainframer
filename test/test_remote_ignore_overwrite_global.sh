#!/bin/bash
set -e

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Execute common pre-setup, include test functions.
source "$DIR/common.sh"

printTestStarted

# This configs should be overwriten by the folder ignore
echo "src/file1.txt" > "$GLOBAL_REMOTE_IGNORE_FILE"
echo "build/buildfile1.txt" >> "$GLOBAL_REMOTE_IGNORE_FILE"

bash "$DIR/test_remote_ignore.sh"

printTestEnded
