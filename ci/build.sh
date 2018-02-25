#!/bin/bash
set -e

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_DIR="$DIR/.."

cp "$PROJECT_DIR/ci/docker/Dockerfile" "$PROJECT_DIR"
cp "$PROJECT_DIR/ci/docker/.dockerignore" "$PROJECT_DIR"

# Remove Dockerfile from root directory when build exits either successfully or with error code.
trap 'rm "$PROJECT_DIR/Dockerfile" && rm "$PROJECT_DIR/.dockerignore"' EXIT

pushd "$PROJECT_DIR"

# Files created in mounted volume by container should have same owner as host machine user to prevent chmod problems.
USER_ID=`id -u $USER`

if [ "$USER_ID" == "0" ]; then
    echo "Warning: running as r00t."
fi

docker build -t mainframer:latest .

# Command will run inside a container.
BUILD_COMMAND="set -xe && "

# Setup ssh for tests.
BUILD_COMMAND+="mkdir -p ~/.ssh/ && "
BUILD_COMMAND+="chmod u+rwx,go= ~/.ssh/ && "
BUILD_COMMAND+="ssh-keygen -b 2048 -t rsa -f ~/.ssh/id_rsa -q -N '' && "
BUILD_COMMAND+="cp ~/.ssh/id_rsa.pub ~/.ssh/authorized_keys && "
BUILD_COMMAND+="chmod u+rw,go= ~/.ssh/authorized_keys && "
BUILD_COMMAND+="ssh-keyscan -t rsa localhost > ~/.ssh/known_hosts && "
BUILD_COMMAND+="cat ~/.ssh/known_hosts && "
BUILD_COMMAND+="chmod u+rw,go= ~/.ssh/known_hosts && "

# Add ANDROID_HOME to bashrc for ssh sessions.
BUILD_COMMAND+="mv ~/.bashrc ~/.bashrc_original && echo -e 'export ANDROID_HOME=/opt/android-sdk-linux\n' > ~/.bashrc && cat ~/.bashrc_original >> ~/.bashrc && rm ~/.bashrc_original && "

BUILD_COMMAND+="/opt/project/test/test.sh --run-samples"

docker run \
--rm \
--volume `"pwd"`:/opt/project \
--env LOCAL_USER_ID="$USER_ID" \
mainframer:latest \
bash -c "$BUILD_COMMAND"

popd
