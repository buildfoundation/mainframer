#!/bin/bash
set -eu

rustup self update
rustup update
rustup component add clippy-preview

cargo clippy -- -D warnings
cargo build
cargo build --release
cargo test

if [ -z "${TRAVIS_TAG}" ]; then
    echo "Tag is not detected, artifacts will not be provided."
else
    echo "Tag is detected, providing artifacts...."

    mkdir -p "artifacts"
    mv "target/release/mainframer" "artifacts/mainframer-${TRAVIS_TAG}-$(uname -s)"
fi

