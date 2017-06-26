#!/bin/bash
set -e

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Execute common pre-setup, include test functions.
source "$DIR/common.sh"

printTestStarted

# Copy Gradle sample to build dir.
cp -a "$DIR/../samples/gradle/." "$BUILD_DIR"

# Overwrite config to work with test remote machine.
setTestRemoteMachineInConfig

# Run mainframer that builds Gradle project.
bash "$REPO_DIR"/mainframer './gradlew build'

# Run jar to ensure that it was built fine.
java -jar "$BUILD_DIR/build/libs/sample.jar"

printTestEnded
