#!/bin/bash
set -xe

##
## Test checks basic functions of mainframer:
##   1. Sync files from local to remote machine.
##   2. Run build command on remote machine. 
##   3. Sync results from remote machine to local machine.
##

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Execute common pre-setup, include test functions.
source "$DIR/common.sh"

# Create several files that should be synced to remote machine.
mkdir "$BUILD_DIR/src"
touch "$BUILD_DIR/src/file1.txt"
touch "$BUILD_DIR/src/file2.txt"
touch "$BUILD_DIR/src/file3.txt"

# TODO: Remove these files once they'll become non-required and create separate tests for exclude strategies.
touch "$BUILD_DIR/.mainframerignorelocal"
touch "$BUILD_DIR/.mainframerignoreremote"

# Run mainframer.sh that'll create a file as a "build result".
bash "$BUILD_DIR"/mainframer.sh 'mkdir build && touch build/buildresult.txt'

# Make sure build result file was synced back to "local" machine.
fileMustExist "$BUILD_DIR/build/buildresult.txt" "(sync or remote build execution problem)"

# Make sure local src files not deleted after execution.
fileMustExist "$BUILD_DIR/src/file1.txt" "(removed by sync)"
fileMustExist "$BUILD_DIR/src/file2.txt" "(removed by sync)"
fileMustExist "$BUILD_DIR/src/file3.txt" "(removed by sync)"
