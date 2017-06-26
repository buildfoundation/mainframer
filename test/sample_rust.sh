#!/bin/bash
set -e

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Execute common pre-setup, include test functions.
source "$DIR/common.sh"

printTestStarted

# Copy Rust sample to build dir.
cp -a "$DIR/../samples/rust/." "$BUILD_DIR"

# Overwrite config to work with test remote machine.
setTestRemoteMachineInConfig

# Run mainframer that builds Rust project.
bash "$REPO_DIR"/mainframer 'cargo build'

# Run binary to ensure that it was built fine.
eval "$BUILD_DIR/target/debug/mainframer_rust_sample"

printTestEnded
