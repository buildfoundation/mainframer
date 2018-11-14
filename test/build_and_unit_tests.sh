#!/bin/bash
set -e

export RUSTFLAGS="--deny warnings"

echo "Building debug version of Mainframer..."
cargo build

echo "Building release version of Mainframer..."
cargo build --release

echo "Running unit tests..."
cargo test
