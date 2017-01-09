#!/bin/bash
set -e

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Execute common pre-setup, include test functions.
source "$DIR/common.sh"

printTestStarted

# Copy Gradle sample to build dir.
cp -r "$DIR/../samples/gradle/" "$BUILD_DIR"

# Overwrite config to work with test remote machine.
setTestRemoteMachineInPersonalConfig

# Run mainframer.sh that builds Gradle project.
bash "$BUILD_DIR"/mainframer.sh 'ls -la && ./gradlew clean build'

# Run jar to ensure that it was built fine.
java -jar "$BUILD_DIR/build/libs/sample.jar"

printTestEnded
