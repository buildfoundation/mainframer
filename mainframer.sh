#!/bin/bash
set -e

echo "mainframer v1.1.2"

echo "Start time: $( date )"
BUILD_START_TIME=`date +%s`

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_DIR=$DIR
PROJECT_DIR_NAME="$( basename "$PROJECT_DIR")"
MAINFRAMER_DIR="$PROJECT_DIR/.mainframer"
IGNORE_LOCAL_FILE="$MAINFRAMER_DIR/localignore"
IGNORE_REMOTE_FILE="$MAINFRAMER_DIR/remoteignore"

function property {
    grep "^${1}=" "$MAINFRAMER_DIR"/local.properties | cut -d'=' -f2
}

# Read config variables from local.properties.
REMOTE_BUILD_MACHINE=$(property 'remote_build.machine')
LOCAL_COMPRESS_LEVEL=$(property 'remote_build.local_gzip_level')
REMOTE_COMPRESS_LEVEL=$(property 'remote_build.remote_gzip_level')

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

function syncBeforeBuild {
	echo "Syncing before build…"
	startTime=`date +%s`

	COMMAND="rsync --archive --delete --compress-level=$LOCAL_COMPRESS_LEVEL "

	if [ -f "$IGNORE_LOCAL_FILE" ]; then
		COMMAND+="--exclude-from='$IGNORE_LOCAL_FILE' "
	fi

	COMMAND+="--rsh ssh ./ $REMOTE_BUILD_MACHINE:~/$PROJECT_DIR_NAME"

	eval "$COMMAND"

	endTime=`date +%s`
	echo "Sync done (took `expr $endTime - $startTime` seconds)."
}

function buildProjectOnRemoteMachine {
	echo "Executing build on remote machine…"
	startTime=`date +%s`

	ssh $REMOTE_BUILD_MACHINE "echo 'set -xe && cd ~/$PROJECT_DIR_NAME/ && $BUILD_COMMAND' | bash"

	endTime=`date +%s`
	echo "Execution done (took `expr $endTime - $startTime` seconds)."
}

function syncAfterBuild {
	echo "Syncing after build…"
	startTime=`date +%s`

	COMMAND="rsync --archive --delete --compress-level=$REMOTE_COMPRESS_LEVEL "

	if [ -f "$IGNORE_REMOTE_FILE" ]; then
		COMMAND+="--exclude-from='$IGNORE_REMOTE_FILE' "
	fi

	COMMAND+="--rsh ssh $REMOTE_BUILD_MACHINE:~/$PROJECT_DIR_NAME/ ./"
	eval "$COMMAND"

	endTime=`date +%s`
	echo "Sync done (took `expr $endTime - $startTime` seconds)."
}

pushd "$PROJECT_DIR" > /dev/null

syncBeforeBuild
buildProjectOnRemoteMachine
syncAfterBuild

popd > /dev/null

BUILD_END_TIME=`date +%s`
echo "End time: $( date )"
echo "Whole process took `expr $BUILD_END_TIME - $BUILD_START_TIME` seconds."
