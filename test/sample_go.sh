#!/bin/bash
set -e

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Execute common pre-setup, include test functions.
# shellcheck disable=SC1090
source "$DIR/common.sh"

printTestStarted

# Copy Go sample to build dir.
cp -a "$DIR/../samples/go/." "$BUILD_DIR"

# Overwrite config to work with test remote machine.
setTestRemoteMachineInConfig

# Run mainframer that builds Go project.
"$MAINFRAMER_EXECUTABLE" eval "export GOPATH=\`pwd\` && go install gojuno.com/mainframer/sample"

# Run binary to ensure that it was built fine.
eval "$BUILD_DIR/bin/sample"

printTestEnded
