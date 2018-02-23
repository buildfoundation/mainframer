#!/bin/bash
set -e

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Execute common pre-setup, include test functions.
source "$DIR/common.sh"

printTestStarted

# Make sure local ignore rules do not exist. 
rm -f "$LOCAL_IGNORE_FILE"

# Run mainframer that noops to make sure that it does not exit with error.
"$MAINFRAMER_EXECUTABLE" 'echo noop'

printTestEnded
