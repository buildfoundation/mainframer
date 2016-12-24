#!/bin/bash
set -e

echo "mainframer v1.0.2"

echo "Start time: $( date )"
BUILD_START_TIME=`date +%s`

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_DIR=$DIR
PROJECT_DIR_NAME="$( basename "$PROJECT_DIR")"

# Read config variables from local.properties.
REMOTE_BUILD_MACHINE=$(awk -F "=" '/remote_build.machine/ {print $2}' "$PROJECT_DIR/local.properties")

if [ -z "$REMOTE_BUILD_MACHINE" ]; then
	echo "Please specify remote build machine in local.properties"
	exit 1
fi

BUILD_COMMAND="$@"

if [ -z "$BUILD_COMMAND" ]; then
	echo "Please pass build command."
	exit 1
fi

pushd "$PROJECT_DIR"
# Sync project to remote machine
rsync -a --delete \
--exclude='.gradle' \
--exclude='.idea' \
--exclude='.git' \
--exclude='artifacts' \
--exclude='captures' \
-e "ssh" ./ "$REMOTE_BUILD_MACHINE:~/$PROJECT_DIR_NAME"


# Build project on a remote machine
ssh $REMOTE_BUILD_MACHINE \
"set -xe && \
cd ~/mainframer/ && \
$BUILD_COMMAND"

# Sync project back to local machine
echo Syncing back to local machine
rsync --delete -a \
-e "ssh" "$REMOTE_BUILD_MACHINE:~/$PROJECT_DIR_NAME/" ./
popd

BUILD_END_TIME=`date +%s`
echo "End time: $( date )"
echo "Whole process took `expr $BUILD_END_TIME - $BUILD_START_TIME` seconds."
