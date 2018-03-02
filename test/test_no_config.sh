#!/bin/bash
set -e

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Execute common pre-setup, include test functions.
# shellcheck disable=SC1090
source "$DIR/common.sh"

printTestStarted

# Make sure config does not exist. 
rm -f "$CONFIG_FILE"

set +e

# Run mainframer that noops to make sure that it exits with error.
"$MAINFRAMER_EXECUTABLE" 'echo noop'

if [ "$?" == "0" ]; then
	set -e
	echo "Should have failed because config does not exist."
	exit 1
fi

set -e

printTestEnded
