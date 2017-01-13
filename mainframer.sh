#!/bin/bash

# Copyright 2016 Juno, Inc.

# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at

# http://www.apache.org/licenses/LICENSE-2.0

# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

set -e

echo ":: mainframer v2.0.0-dev"
echo ""

BUILD_START_TIME=`date +%s`

PROJECT_DIR="`pwd`"
PROJECT_DIR_NAME="$( basename "$PROJECT_DIR")"
MAINFRAMER_DIR="$PROJECT_DIR/.mainframer"

PROJECT_DIR_ON_REMOTE_MACHINE="~/mainframer/$PROJECT_DIR_NAME"
PERSONAL_CONFIG_FILE="$MAINFRAMER_DIR/personalconfig"
LOCAL_IGNORE_FILE="$MAINFRAMER_DIR/localignore"
REMOTE_IGNORE_FILE="$MAINFRAMER_DIR/remoteignore"
COMMON_IGNORE_FILE="$MAINFRAMER_DIR/ignore"

function property {
    grep "^${1}=" "$PERSONAL_CONFIG_FILE" | cut -d'=' -f2
}

# Config properties.
REMOTE_MACHINE_PROPERTY="remote_machine"
LOCAL_COMPRESS_LEVEL_PROPERTY="local_compression_level"
REMOTE_COMPRESS_LEVEL_PROPERTY="remote_compression_level"

# Read config properties.
REMOTE_BUILD_MACHINE=$(property "$REMOTE_MACHINE_PROPERTY")
LOCAL_COMPRESS_LEVEL=$(property "$LOCAL_COMPRESS_LEVEL_PROPERTY")
REMOTE_COMPRESS_LEVEL=$(property "$REMOTE_COMPRESS_LEVEL_PROPERTY")

if [ -z "$LOCAL_COMPRESS_LEVEL" ]; then
	LOCAL_COMPRESS_LEVEL=1
fi

if [ -z "$REMOTE_COMPRESS_LEVEL" ]; then
	REMOTE_COMPRESS_LEVEL=1
fi

if [ -z "$REMOTE_BUILD_MACHINE" ]; then
	echo "Please specify \"$REMOTE_MACHINE_PROPERTY\" in $PERSONAL_CONFIG_FILE"
	exit 1
fi

BUILD_COMMAND="$@"

if [ -z "$BUILD_COMMAND" ]; then
	echo "Please pass build command."
	exit 1
fi

function syncBeforeBuild {
	echo "Sync local → remote machine..."
	startTime=`date +%s`

	COMMAND="rsync --archive --delete --rsync-path=\"mkdir -p \"$PROJECT_DIR_ON_REMOTE_MACHINE\" && rsync\" --compress-level=$LOCAL_COMPRESS_LEVEL "

	if [ -f "$LOCAL_IGNORE_FILE" ]; then
		COMMAND+="--exclude-from='$LOCAL_IGNORE_FILE' "
	fi

	if [ -f "$COMMON_IGNORE_FILE" ]; then
		COMMAND+="--exclude-from='$COMMON_IGNORE_FILE' "
	fi

	COMMAND+="--rsh ssh ./ $REMOTE_BUILD_MACHINE:$PROJECT_DIR_ON_REMOTE_MACHINE"

	eval "$COMMAND"

	endTime=`date +%s`
	echo "Sync done: took `expr $endTime - $startTime` seconds."
	echo ""
}

function buildProjectOnRemoteMachine {
	echo "Executing build on remote machine…"
	echo ""
	startTime=`date +%s`

	ssh $REMOTE_BUILD_MACHINE "echo 'set -e && cd $PROJECT_DIR_ON_REMOTE_MACHINE && $BUILD_COMMAND' | bash"

	endTime=`date +%s`
	echo ""
	echo "Execution done: took `expr $endTime - $startTime` seconds."
	echo ""
}

function syncAfterBuild {
	echo "Sync remote → local machine…"
	startTime=`date +%s`

	COMMAND="rsync --archive --delete --compress-level=$REMOTE_COMPRESS_LEVEL "

	if [ -f "$REMOTE_IGNORE_FILE" ]; then
		COMMAND+="--exclude-from='$REMOTE_IGNORE_FILE' "
	fi

	if [ -f "$COMMON_IGNORE_FILE" ]; then
		COMMAND+="--exclude-from='$COMMON_IGNORE_FILE' "
	fi

	COMMAND+="--rsh ssh $REMOTE_BUILD_MACHINE:$PROJECT_DIR_ON_REMOTE_MACHINE/ ./"
	eval "$COMMAND"

	endTime=`date +%s`
	echo "Sync done: took `expr $endTime - $startTime` seconds."
}

pushd "$PROJECT_DIR" > /dev/null

syncBeforeBuild

set +e
buildProjectOnRemoteMachine
set -e

syncAfterBuild

popd > /dev/null

BUILD_END_TIME=`date +%s`
echo ""
echo "Done: took `expr $BUILD_END_TIME - $BUILD_START_TIME` seconds."
