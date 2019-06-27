#!/bin/bash
set -e

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

TEST_COUNTER=1
TEST_RUN_SUCCESS="false"

function printTestResults {
	echo ""
	if [ "$TEST_RUN_SUCCESS" == "true" ]; then
		echo "Test run SUCCESS, $TEST_COUNTER test(s)."
	else
		echo "Test run FAILED, $TEST_COUNTER test(s)."
		echo "To log each step: export DEBUG_MODE_FOR_ALL_TESTS=true"
	fi
}

# Hook to exit happened either because of success or error.
trap printTestResults EXIT

pushd "$DIR/../" > /dev/null

"$DIR/build_and_unit_tests.sh"
TEST_COUNTER=$((TEST_COUNTER+1))

"$DIR/clippy.sh"
TEST_COUNTER=$((TEST_COUNTER+1))

popd > /dev/null


echo "Running integration tests…"

# Print stacktrace for debug build in case it panics at runtime.
export RUST_BACKTRACE=1

# Run all tests.
for test_ in "$DIR"/test_*; do
	TEST_COUNTER=$((TEST_COUNTER+1))
	TEST_PULL_MODE="serial" "$test_"

	TEST_COUNTER=$((TEST_COUNTER+1))
	TEST_PULL_MODE="parallel" "$test_"
done

TEST_RUN_SUCCESS="true"
