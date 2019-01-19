#!/bin/bash
set -eu

# Travis macOS image does not have ShellCheck.
if [[ "$OSTYPE" == "linux-gnu" ]]; then
  for file in $(find test -type f -name "*.sh"); do shellcheck --format=gcc $file; done
fi

# NOTE: local machine and remote machine are fake
# since we are using the same machine as both of them.

# Travis Linux image already has SSH server started, macOS has not.
if [[ "$OSTYPE" == "darwin" ]]; then
  sudo systemsetup -setremotelogin on
fi

# Create a local machine SSH key with blank passphrase.
ssh-keygen -b 2048 -t rsa -f ~/.ssh/id_rsa -N '' -q

# Make the remote machine a known SSH host for the local machine.
ssh-keyscan -t rsa localhost > ~/.ssh/known_hosts

# Add local machine SSH key to the remote machine.
cp ~/.ssh/id_rsa.pub ~/.ssh/authorized_keys

# Run!
bash test/test.sh
