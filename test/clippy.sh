#!/bin/bash
set -e

echo "Running Clippy linter..."

# Warnings as errors.
cargo clippy -- -D warnings
