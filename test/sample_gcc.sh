#!/bin/bash
set -e

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Execute common pre-setup, include test functions.
source "$DIR/common.sh"

printTestStarted

# Copy GCC sample to build dir.
cp -a "$DIR/../samples/gcc/." "$BUILD_DIR"

# Overwrite config to work with test remote machine.
setTestRemoteMachineInConfig

# Run mainframer that builds GCC project.
"$MAINFRAMER_EXECUTABLE" 'gcc -Wall sample.c -o sample'

# Run binary to ensure that it was built fine.
eval "$BUILD_DIR/sample"

printTestEnded
