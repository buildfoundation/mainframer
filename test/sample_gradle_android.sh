#!/bin/bash
set -e

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Execute common pre-setup, include test functions.
# shellcheck disable=SC1090
source "$DIR/common.sh"

printTestStarted

# Copy Gradle Android sample to build dir.
cp -a "$DIR/../samples/gradle-android/." "$BUILD_DIR"

# Overwrite config to work with test remote machine.
setTestRemoteMachineInConfig

# Run mainframer that builds Gradle Android project.
"$MAINFRAMER_EXECUTABLE" './gradlew clean build'

# Check that apk exists to ensure that it build was fine.
fileMustExistOnLocalMachine 'app/build/outputs/apk/debug/app-debug.apk'

printTestEnded
