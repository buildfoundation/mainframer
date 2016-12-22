#!/bin/bash
set -e

echo "mainframer v1.0.3"

echo "Start time: $( date )"
BUILD_START_TIME=`date +%s`

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_DIR=$DIR
PROJECT_DIR_NAME="$( basename "$PROJECT_DIR")"

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

# Archiver.

DETECT_ARCHIVER="if type 'pigz' > /dev/null; then echo 'pigz'; else echo 'gzip'; fi"
LOCAL_ARCHIVER=`eval $DETECT_ARCHIVER`

# Archive project.
pushd "$PROJECT_DIR"
LOCAL_ARCHIVE_COMMAND="tar \
-c \
--exclude='build/project_for_remote_build.tar' \
--exclude='local.properties' \
--exclude='.gradle' \
--exclude='.idea' \
--exclude='.git' \
--exclude='artifacts' \
--exclude='captures' \
--exclude='build' \
--exclude='*/build' \
."

if [ $LOCAL_GZIP_LEVEL = "0" ]; then
	LOCAL_ARCHIVE_COMMAND+=" > build/project_for_remote_build.tar"
	REMOTE_UNARCHIVE_COMMAND="tar -xf project_for_remote_build.tar -C $PROJECT_DIR_NAME"
else
	LOCAL_ARCHIVE_COMMAND+=" | $LOCAL_ARCHIVER -$LOCAL_GZIP_LEVEL > build/project_for_remote_build.tar"
	REMOTE_UNARCHIVE_COMMAND="\$REMOTE_ARCHIVER -d < project_for_remote_build.tar | tar -xf - -C $PROJECT_DIR_NAME"
fi

eval $LOCAL_ARCHIVE_COMMAND
popd

# Prepare remote archive and local unarchive commands.
REMOTE_ARCHIVE_COMMAND="tar \
-c \
--exclude='kotlin' \
--exclude='tmp' \
build/ */build"

if [ $REMOTE_GZIP_LEVEL = "0" ]; then
	REMOTE_ARCHIVE_COMMAND+=" > remotely_built_project.tar"
	LOCAL_UNARCHIVE_COMMAND="tar -xf build/remotely_built_project.tar -C ./"
else
	REMOTE_ARCHIVE_COMMAND+=" | \$REMOTE_ARCHIVER -$REMOTE_GZIP_LEVEL > remotely_built_project.tar"
	LOCAL_UNARCHIVE_COMMAND="$LOCAL_ARCHIVER -d < build/remotely_built_project.tar | tar -xf - -C ./"
fi

# Transfer archive to remote machine.
scp "$PROJECT_DIR/build/project_for_remote_build.tar" $REMOTE_BUILD_MACHINE:~/

# Build project on a remote machine and then archive it.
ssh $REMOTE_BUILD_MACHINE \
"set -xe && \
export REMOTE_ARCHIVER=\`eval \"$DETECT_ARCHIVER\"\` && \
cd ~ && \
mkdir -p $PROJECT_DIR_NAME && \
rm -rf $PROJECT_DIR_NAME/build/remotely_built_project.tar $PROJECT_DIR_NAME/*/src && \
$REMOTE_UNARCHIVE_COMMAND && \
cd $PROJECT_DIR_NAME && \
$BUILD_COMMAND && \
$REMOTE_ARCHIVE_COMMAND"

# Clean local build dirs.
rm -rf "$PROJECT_DIR"/build "$PROJECT_DIR"/*/build
mkdir -p "$PROJECT_DIR/build/"

# Copy build results from remote machine to local.
scp "$REMOTE_BUILD_MACHINE":~/"$PROJECT_DIR_NAME"/remotely_built_project.tar "$PROJECT_DIR/build/"

# Unarchive build results.
pushd "$PROJECT_DIR"
eval "$LOCAL_UNARCHIVE_COMMAND"
popd

BUILD_END_TIME=`date +%s`
echo "End time: $( date )"
echo "Whole process took `expr $BUILD_END_TIME - $BUILD_START_TIME` seconds."
