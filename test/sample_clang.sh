#!/bin/bash
set -e

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Execute common pre-setup, include test functions.
source "$DIR/common.sh"

printTestStarted

# Copy Clang sample to build dir.
cp -a "$DIR/../samples/clang/." "$BUILD_DIR"

# Overwrite config to work with test remote machine.
setTestRemoteMachineInConfig

# Run mainframer.sh that builds Clang project.
bash "$REPO_DIR"/mainframer.sh 'clang sample.c -o sample'

# Run binary to ensure that it was built fine.
eval "$BUILD_DIR/sample"

printTestEnded
