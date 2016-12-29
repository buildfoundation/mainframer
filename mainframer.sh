#!/bin/bash
set -e

echo "mainframer v1.1.0"

echo "Start time: $( date )"
BUILD_START_TIME=`date +%s`

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_DIR=$DIR
PROJECT_DIR_NAME="$( basename "$PROJECT_DIR")"

# Read config variables from local.properties.
REMOTE_BUILD_MACHINE=$(awk -F "=" '/remote_build.machine/ {print $2}' "$PROJECT_DIR/local.properties")
LOCAL_COMPRESS_LEVEL=$(awk -F "=" '/remote_build.local_gzip_level/ {print $2}' "$PROJECT_DIR/local.properties")
REMOTE_COMPRESS_LEVEL=$(awk -F "=" '/remote_build.local_gzip_level/ {print $2}' "$PROJECT_DIR/local.properties")
REMOTE_SSH_PORT=$(awk -F "=" '/remote_build.ssh_port/ {print $2}' "$PROJECT_DIR/local.properties")

if [ -z "$LOCAL_COMPRESS_LEVEL" ]; then
	LOCAL_COMPRESS_LEVEL=1
fi

if [ -z "$REMOTE_COMPRESS_LEVEL" ]; then
	REMOTE_COMPRESS_LEVEL=1
fi

if [ -z "$REMOTE_BUILD_MACHINE" ]; then
	echo "Please specify remote build machine in local.properties"
	exit 1
fi

if [ -z "$REMOTE_SSH_PORT" ]; then
	REMOTE_SSH_PORT=22
fi

BUILD_COMMAND="$@"

if [ -z "$BUILD_COMMAND" ]; then
	echo "Please pass build command."
	exit 1
fi

pushd "$PROJECT_DIR"
# Sync project to remote machine.
rsync --archive --delete --compress-level=$LOCAL_COMPRESS_LEVEL \
--exclude='.gradle' \
--exclude='.idea' \
--exclude='**/.git/' \
--exclude='artifacts' \
--exclude='captures' \
--exclude='**/build' \
--exclude='**/local.properties' \
--rsh "ssh -p$REMOTE_SSH_PORT" ./ "$REMOTE_BUILD_MACHINE:~/$PROJECT_DIR_NAME"

# Build project on a remote machine.
ssh $REMOTE_BUILD_MACHINE -p $REMOTE_SSH_PORT \
"set -xe && \
cd ~/$PROJECT_DIR_NAME/ && \
$BUILD_COMMAND"

# Sync project back to local machine.
rsync --archive --delete --compress-level=$REMOTE_COMPRESS_LEVEL \
--exclude='.gradle' \
--exclude='.idea' \
--exclude='**/.git/' \
--exclude='artifacts' \
--exclude='captures' \
--exclude='**/local.properties' \
--rsh "ssh -p$REMOTE_SSH_PORT" "$REMOTE_BUILD_MACHINE:~/$PROJECT_DIR_NAME/" ./
popd

BUILD_END_TIME=`date +%s`
echo "End time: $( date )"
echo "Whole process took `expr $BUILD_END_TIME - $BUILD_START_TIME` seconds."
