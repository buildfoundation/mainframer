#!/bin/bash
set -euo pipefail

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

rustc --version
cargo --version

# Override Mainframer version if needed.
"$DIR/mainframer_version.sh"

"$DIR/../test/build_and_unit_tests.sh"

mkdir -p artifacts
cp target/release/mainframer "artifacts/mainframer-macOS"
