#!/bin/bash
set -e

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Note that we only run basic build and unit tests on macOS as it's much harder to configure for integration tests.
# We run extensive testing in Docker container with Linux, see ci/build.sh.

# shellcheck disable=SC1090
source "$DIR/prepare_build_dir.sh"

# Install Rust.
# shellcheck disable=SC1090
MAINFRAMER_RUST_VERSION="$(cat "$DIR"/rust.version)"
curl --silent --fail --location curl https://sh.rustup.rs | sh -s -- -y --default-toolchain="$MAINFRAMER_RUST_VERSION"
# shellcheck disable=SC1090
source "$HOME/.cargo/env"
rustc --version
cargo --version

pushd "$BUILD_DIR" > /dev/null

# Override Mainframer version if needed.
"$DIR/mainframer_version.sh"

"$DIR/../test/build_and_unit_tests.sh"

mkdir -p artifacts
mv target/release/mainframer "artifacts/mainframer-$TRAVIS_TAG-$(uname -s)"

popd > /dev/null
