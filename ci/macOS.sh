#!/bin/bash
set -e

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Note that we only run basic build and unit tests on macOS as it's much harder to configure for integration tests.
# We run extensive testing in Docker container with Linux, see ci/build.sh.

# shellcheck disable=SC1090
source "$DIR/prepare_build_dir.sh"

# Install Rust. TODO: use common declaration for Rust version (duplicated in ci/docker/Dockerfile).
curl --silent --fail --location https://static.rust-lang.org/rustup.sh | sh -s -- --revision=1.24.0
rustc --version
cargo --version

pushd "$BUILD_DIR" > /dev/null

echo "Building debug version of Mainframer..."
cargo build

echo "Building release version of Mainframer..."
cargo build --release

echo "Running unit tests..."
cargo test

popd > /dev/null
