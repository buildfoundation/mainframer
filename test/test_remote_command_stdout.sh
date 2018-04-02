#!/bin/bash
set -e

#
# Test checks that stdout of remote command gets piped into Mainframer output.
#

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Execute common pre-setup, include test functions.
# shellcheck disable=SC1090
source "$DIR/common.sh"

printTestStarted

# Run Mainframer that outputs multiple lines of text to stdout.
OUTPUT=$("$MAINFRAMER_EXECUTABLE" "echo 'hello' && echo 'world'")
echo "$OUTPUT"

# Greping strings with \n is really hard, we replace \n with `|`.
EXPECTED="hello|world"
OUTPUT_FOR_GREP=$(echo "$OUTPUT" | tr '\r\n' '|')

if ! echo "$OUTPUT_FOR_GREP" | grep "$EXPECTED"; then
    echo "stdout output of Mainframer doesn't contain expected text, expected = '$EXPECTED', actual = '$OUTPUT_FOR_GREP'"
    exit 1
fi

printTestEnded
