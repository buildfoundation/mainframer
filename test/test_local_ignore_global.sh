#!/bin/bash
set -e

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Execute common pre-setup, include test functions.
source "$DIR/common.sh"

printTestStarted

bash "$DIR/common_test_local_ignore.sh" "$GLOBAL_LOCAL_IGNORE_FILE"

printTestEnded
