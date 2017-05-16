#!/bin/bash
set -xe

# See https://denibertovic.com/posts/handling-permissions-with-docker-volumes/

# Add local user.
# Either use the LOCAL_USER_ID if passed in at runtime or fallback to 9001.
USER_ID=${LOCAL_USER_ID:-9001}

echo "Starting with UID : $USER_ID"
groupadd --gid $USER_ID build_user
useradd --shell /bin/bash --uid $USER_ID --gid $USER_ID --comment "User for container" --create-home build_user

# Grant build_user access to Android SDK.
chown -R build_user:build_user $ANDROID_HOME

# Start ssh server for tests.
service ssh start

# Run original docker run command as build_user.
sudo --set-home --preserve-env -u build_user "$@"
