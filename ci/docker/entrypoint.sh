#!/bin/bash
set -xe

# See https://denibertovic.com/posts/handling-permissions-with-docker-volumes/

if [ "$LOCAL_USER_ID" == "" ]; then
    uid=$("id -u")
else
    uid="$LOCAL_USER_ID"
    groupmod -g "$uid" build_user
    usermod -u "$uid" build_user
    chown -R build_user:build_user "$HOME"
fi

echo "Starting with UID : $uid"

# Start ssh server for tests.
sudo service ssh start

# Run original docker run command as build_user.
exec "$@"
