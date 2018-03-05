#!/bin/bash
set -e

echo "Building debug version of Mainframer..."
cargo build

echo "Building release version of Mainframer..."
cargo build --release

echo "Running unit tests..."
cargo test
