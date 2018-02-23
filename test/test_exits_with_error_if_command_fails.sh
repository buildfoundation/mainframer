#!/bin/bash
set -e

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Execute common pre-setup, include test functions.
source "$DIR/common.sh"

printTestStarted

# Run mainframer that exits with error code on remote machine.
set +e
"$MAINFRAMER_EXECUTABLE" 'exit 1'

# Make sure mainframer also exits with error code.
if [ "$?" == "0" ]; then
	echo "Should have exited with error code."
	set -e
	exit 1
fi
set -e

printTestEnded
