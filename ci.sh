#!/bin/bash
set -xe

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Run all tests.
for test_ in "$DIR"/test/test_*; do
	bash "$test_"
done
