#!/bin/bash 
set -e

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Execute common pre-setup, include test functions.
source "$DIR/common.sh"

printTestStarted

# Mainframer will take config from global config
echo "$REMOTE_MACHINE_PROPERTY=invalidIpAddress" > "$GLOBAL_CONFIG_FILE"

# Run mainframer that noops to make sure that it does not exit with error.
"$REPO_DIR"/mainframer 'echo noop'

printTestEnded
