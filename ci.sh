#!/bin/bash
set -xe

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_DIR=$DIR
BUILD_DIR="$PROJECT_DIR/ci"

function fileMustExist {
	if [ ! -f "$1" ]; then
		echo "$1 does not exist $2"
		exit 1
	fi
}

# Clean previous build directory.
rm -rf "$BUILD_DIR"

# Create build directory.
mkdir "$BUILD_DIR"

# Copy mainframer.sh into build directory.
cp "$PROJECT_DIR/mainframer.sh" "$BUILD_DIR/"

# Create several files that should be synced to remote machine.
mkdir "$BUILD_DIR/src"
touch "$BUILD_DIR/src/file1.txt"
touch "$BUILD_DIR/src/file2.txt"
touch "$BUILD_DIR/src/file3.txt"

# Create local.properties that sets localhost as a "remote" build machine 
# (this is how we test, localhost should have sshd running on port 22 and ssh key of current user allowed).
echo "remote_build.machine=locahost" > "$BUILD_DIR/local.properties"

# Run mainframer.sh that'll create a file as "build result".
bash "$BUILD_DIR/mainframer.sh" mkdir build && touch build/buildresult.txt

# Make sure build result file was synced back to "local" machine.
fileMustExist "$BUILD_DIR/build/buildresult.txt" "(sync or remote build execution problem)"

# Make sure local src files not deleted after execution.
fileMustExist "$BUILD_DIR/src/file1.txt" "(removed after sync)"
fileMustExist "$BUILD_DIR/src/file2.txt" "(removed after sync)"
fileMustExist "$BUILD_DIR/src/file3.txt" "(removed after sync)"
