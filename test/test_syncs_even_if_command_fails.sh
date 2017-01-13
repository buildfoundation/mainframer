#!/bin/bash
set -e

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Execute common pre-setup, include test functions.
source "$DIR/common.sh"

printTestStarted

# Run mainframer.sh that creates "build" result file that should be synced back to local machine even after error exit code.
bash "$REPO_DIR"/mainframer.sh 'mkdir build && touch build/buildresult.txt && exit 1'

# Make sure files exist on local machine after sync.
fileMustExistOnLocalMachine "build/buildresult.txt" "(sync after error code problem)"

printTestEnded
