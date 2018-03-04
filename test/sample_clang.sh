#!/bin/bash
set -e

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Execute common pre-setup, include test functions.
# shellcheck disable=SC1090
source "$DIR/common.sh"

printTestStarted

# Copy Clang sample to build dir.
cp -a "$DIR/../samples/clang/." "$BUILD_DIR"

# Overwrite config to work with test remote machine.
setTestRemoteMachineInConfig

# Run mainframer that builds Clang project.
"$MAINFRAMER_EXECUTABLE" 'clang sample.c -o sample'

# Run binary to ensure that it was built fine.
eval "$BUILD_DIR/sample"

printTestEnded
