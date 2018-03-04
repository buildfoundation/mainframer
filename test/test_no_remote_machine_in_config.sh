#!/bin/bash
set -e

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Execute common pre-setup, include test functions.
# shellcheck disable=SC1090
source "$DIR/common.sh"

printTestStarted

# Make sure config is empty. 
echo "" > "$CONFIG_FILE"

# Run mainframer to make sure that it exits with error.
if "$MAINFRAMER_EXECUTABLE" 'echo noop'; then	set -e
	echo "Should have failed because config does not contain remote machine property."
	exit 1
fi

printTestEnded
