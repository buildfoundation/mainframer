#!/bin/bash
set -e

echo "Start time: $( date )"
BUILD_START_TIME=`date +%s`

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_DIR=$DIR

# Read config variables from local.properties.
REMOTE_BUILD_MACHINE=$(awk -F "=" '/remote_build.machine/ {print $2}' "$PROJECT_DIR/local.properties")
LOCAL_GZIP_LEVEL=$(awk -F "=" '/remote_build.local_gzip_level/ {print $2}' "$PROJECT_DIR/local.properties")
REMOTE_GZIP_LEVEL=$(awk -F "=" '/remote_build.remote_gzip_level/ {print $2}' "$PROJECT_DIR/local.properties")

if [ -z "$LOCAL_GZIP_LEVEL" ]; then
	LOCAL_GZIP_LEVEL=1
fi

if [ -z "$REMOTE_GZIP_LEVEL" ]; then
	REMOTE_GZIP_LEVEL=1
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

# Create build folder in case if it does not exist.
mkdir -p "$PROJECT_DIR"/build

# Remove previous archives of the project.
rm -f "$PROJECT_DIR"/build/project_for_remote_build.tar "$PROJECT_DIR"/build/remotely_built_project.tar

# Archive project.
pushd "$PROJECT_DIR"
GZIP=-"$LOCAL_GZIP_LEVEL" tar -cz \
--exclude='build/project_for_remote_build.tar' \
--exclude='local.properties' \
--exclude='.gradle' \
--exclude='.idea' \
--exclude='.git' \
--exclude='artifacts' \
--exclude='captures' \
--exclude='build' \
--exclude='*/build' \
-f build/project_for_remote_build.tar .
popd

# Transfer archive to remote machine.
scp "$PROJECT_DIR/build/project_for_remote_build.tar" $REMOTE_BUILD_MACHINE:~/

# Build project on a remove machine.
ssh $REMOTE_BUILD_MACHINE \
"set -xe && \
printenv && \
cd ~ && \
mkdir -p android-project-remote-build && \
rm -rf android-project-remote-build/build/remotely_built_project.tar android-project-remote-build/*/src && \
tar -xzf project_for_remote_build.tar -C android-project-remote-build && \
cd android-project-remote-build && \
$BUILD_COMMAND && \
GZIP=-$REMOTE_GZIP_LEVEL tar -cz \
--exclude='kotlin' \
--exclude='tmp' \
-f remotely_built_project.tar build/ */build"

# Clean local build dirs.
rm -rf "$PROJECT_DIR"/build "$PROJECT_DIR"/*/build

# Copy results back.
mkdir -p "$PROJECT_DIR/build/"
scp "$REMOTE_BUILD_MACHINE":~/android-project-remote-build/remotely_built_project.tar "$PROJECT_DIR/build/"

# Unzip build results.
pushd "$PROJECT_DIR"
tar -xzf build/remotely_built_project.tar -C ./
popd

BUILD_END_TIME=`date +%s`
echo "End time: $( date )"
echo "Whole process took `expr $BUILD_END_TIME - $BUILD_START_TIME` seconds."
