#!/bin/bash
set -e

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Execute common pre-setup, include test functions.
# shellcheck disable=SC1090
source "$DIR/common.sh"

printTestStarted

# Create several files that should be synced to remote machine.
mkdir "$BUILD_DIR/src"
touch "$BUILD_DIR/src/file1.txt"
touch "$BUILD_DIR/src/file2.txt"
touch "$BUILD_DIR/src/file3.txt"

# Run mainframer that creates "build" result file that should be synced back to local machine.
# shellcheck disable=SC2016
"$MAINFRAMER_EXECUTABLE" 'mkdir build && for ((i=0;i<30;i++)); do dd if=/dev/urandom of=build/buildresult-$i.txt bs=16M count=4 iflag=fullblock; sleep 1; done; ls -la build'

for ((i=0;i<30;i++)); do
    # Make sure files exist on local machine after sync.
    localFileMustMatchRemote "build/buildresult-$i.txt" "(sync problem)"
done

printTestEnded
