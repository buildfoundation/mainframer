#!/bin/bash
set -eux

# Update everything and install Clippy.
rustup self update
rustup update
rustup component add clippy-preview

# Run Clippy, debug and release builds, unit tests.
cargo clippy -- -D warnings
cargo build
cargo build --release
cargo test

# Move release build to artifacts.
mkdir -p "artifacts"
mv "target/release/mainframer" "artifacts/mainframer-${TRAVIS_TAG}-$(uname -s)"
