#!/bin/bash

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

echo ":: syncmainframer v2.1.0"
echo ""

PROJECT_DIR="$(pwd)"
PROJECT_DIR_NAME="$( basename "$PROJECT_DIR" )"
PROJECT_DIR_ON_REMOTE_MACHINE="~/mainframer/$PROJECT_DIR_NAME"

CONFIG_DIR="$PROJECT_DIR/.mainframer"
CONFIG_FILE="$CONFIG_DIR/config"
COMMON_IGNORE_FILE="$CONFIG_DIR/ignore"
LOCAL_IGNORE_FILE="$CONFIG_DIR/localignore"

function readConfigProperty {
	grep "^${1}=" "$CONFIG_FILE" | cut -d'=' -f2
}

REMOTE_MACHINE_CONFIG_PROPERTY="remote_machine"
LOCAL_COMPRESS_LEVEL_CONFIG_PROPERTY="local_compression_level"
REMOTE_COMPRESS_LEVEL_CONFIG_PROPERTY="remote_compression_level"

if [ ! -f "$CONFIG_FILE" ]; then
	echo "Please create and fill $CONFIG_FILE."
	exit 1
fi

REMOTE_MACHINE=$(readConfigProperty "$REMOTE_MACHINE_CONFIG_PROPERTY")
LOCAL_COMPRESS_LEVEL=$(readConfigProperty "$LOCAL_COMPRESS_LEVEL_CONFIG_PROPERTY")

if [ -z "$REMOTE_MACHINE" ]; then
	echo "Please specify \"$REMOTE_MACHINE_CONFIG_PROPERTY\" in $CONFIG_FILE."
	exit 1
fi

if [ -z "$LOCAL_COMPRESS_LEVEL" ]; then
	LOCAL_COMPRESS_LEVEL=1
fi

function verifyRequiredTools {
    if ! type fswatch &> /dev/null; then
        echo "Please install fswatch from https://github.com/emcrisostomo/fswatch"
        exit 1
    fi
}

function monitorAndSyncChangesContinuously {
	echo "Continuous sync local → remote machine..."
    echo "Press ctrl+c to terminate."

	RSYNC_COMMAND+="rsync --quiet --archive --delete --rsync-path=\"mkdir -p \"$PROJECT_DIR_ON_REMOTE_MACHINE\" && rsync\" --compress-level=$LOCAL_COMPRESS_LEVEL "

	if [ -f "$COMMON_IGNORE_FILE" ]; then
		RSYNC_COMMAND+="--exclude-from=\"$COMMON_IGNORE_FILE\" "
	fi

	if [ -f "$LOCAL_IGNORE_FILE" ]; then
		RSYNC_COMMAND+="--exclude-from=\"$LOCAL_IGNORE_FILE\" "
	fi

	RSYNC_COMMAND+="--rsh ssh ./ $REMOTE_MACHINE:\"$PROJECT_DIR_ON_REMOTE_MACHINE\""

    FSWATCH_COMMAND="fswatch --latency 5 --batch-marker "

    while IFS= read -r line
    do
        if [[ ${line:0:1} != '#' ]]; then
            [ -z "$line" ] && continue
            FSWATCH_COMMAND+="--exclude $line "
        fi
    done < <(paste -d "\n" "$COMMON_IGNORE_FILE" "$LOCAL_IGNORE_FILE")

    FSWATCH_COMMAND+=" '$PROJECT_DIR'"

    COMMAND="$FSWATCH_COMMAND | xargs -n1 -I{} $RSYNC_COMMAND"

    echo "..."
    
	eval "$COMMAND"
}

pushd "$PROJECT_DIR" > /dev/null

verifyRequiredTools
monitorAndSyncChangesContinuously

popd > /dev/null

