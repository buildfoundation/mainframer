#!/bin/bash
set -e

echo "mainframer v1.1.2"

echo "Start time: $( date )"
BUILD_START_TIME=`date +%s`

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_DIR=$DIR
PROJECT_DIR_NAME="$( basename "$PROJECT_DIR")"

function property {
    grep "^${1}=" $PROJECT_DIR/local.properties | cut -d'=' -f2
}

pushd "$PROJECT_DIR"

# Read config variables from local.properties.
REMOTE_BUILD_MACHINE=$(property 'remote_build.machine')
LOCAL_COMPRESS_LEVEL=$(property 'remote_build.local_gzip_level')
REMOTE_COMPRESS_LEVEL=$(property 'remote_build.local_gzip_level')

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

BUILD_COMMAND="$@"

if [ -z "$BUILD_COMMAND" ]; then
	echo "Please pass build command."
	exit 1
fi

# Read local exclude rules.
LOCAL_EXCLUDE_FILE=".mainframerignorelocal"
LOCAL_EXCLUDE=""

if [ -f "$LOCAL_EXCLUDE_FILE" ]; then
	while read -r line
	do
		LOCAL_EXCLUDE+="--exclude='$line' "
	done < "$LOCAL_EXCLUDE_FILE"
fi

# Read remote exclude rules.
REMOTE_EXCLUDE_FILE=".mainframerignoreremote"
REMOTE_EXCLUDE=""

if [ -f "$REMOTE_EXCLUDE_FILE" ]; then
	while read -r line
	do
		REMOTE_EXCLUDE+="--exclude='$line' "
	done < "$REMOTE_EXCLUDE_FILE"
fi

# Sync project to remote machine.
eval "rsync --archive --delete --compress-level=$LOCAL_COMPRESS_LEVEL $LOCAL_EXCLUDE --rsh ssh ./ $REMOTE_BUILD_MACHINE:~/$PROJECT_DIR_NAME"

# Build project on a remote machine.
ssh $REMOTE_BUILD_MACHINE "echo 'set -xe && cd ~/$PROJECT_DIR_NAME/ && $BUILD_COMMAND' | bash"

# Sync project back to local machine.
eval "rsync --archive --delete --compress-level=$REMOTE_COMPRESS_LEVEL $REMOTE_EXCLUDE --rsh ssh $REMOTE_BUILD_MACHINE:~/$PROJECT_DIR_NAME/ ./"

BUILD_END_TIME=`date +%s`
echo "End time: $( date )"
echo "Whole process took `expr $BUILD_END_TIME - $BUILD_START_TIME` seconds."

popd