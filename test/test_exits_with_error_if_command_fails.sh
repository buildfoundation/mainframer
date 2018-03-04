#!/bin/bash
set -e

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Execute common pre-setup, include test functions.
# shellcheck disable=SC1090
source "$DIR/common.sh"

printTestStarted

# Make sure mainframer also exits with error code.
if "$MAINFRAMER_EXECUTABLE" 'exit 1'; then
	echo "Should have exited with error code."
	exit 1
fi

printTestEnded
