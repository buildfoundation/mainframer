#!/bin/bash
set -e

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Execute common pre-setup, include test functions.
source "$DIR/common.sh"

printTestStarted

# Copy Buck sample to build dir.
cp -a "$DIR/../samples/buck/." "$BUILD_DIR"

# Overwrite config to work with test remote machine.
setTestRemoteMachineInConfig

# Run mainframer that builds Buck project.
"$REPO_DIR"/mainframer 'buck clean && buck build sample'

# Run jar to ensure that it was built fine.
java -jar "$BUILD_DIR/buck-out/gen/sample/sample.jar"

printTestEnded
