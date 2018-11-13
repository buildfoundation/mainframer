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

popd > /dev/null

TEST_COUNTER=$((TEST_COUNTER+1))

echo "Running integration tests…"

# Run all tests.
for test_ in "$DIR"/test_*; do
	TEST_COUNTER=$((TEST_COUNTER+1))
	"$test_"
done

if [ "$1" == "--run-samples" ]; then
	# Run all samples.
	for sample_ in "$DIR"/sample_*; do
		TEST_COUNTER=$((TEST_COUNTER+1))
		"$sample_"
	done
fi

TEST_RUN_SUCCESS="true"
