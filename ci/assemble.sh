#!/bin/bash
set -eux

rustup self update
rustup update
rustup component add clippy-preview

cargo clippy -- -D warnings
cargo build
cargo build --release
cargo test
