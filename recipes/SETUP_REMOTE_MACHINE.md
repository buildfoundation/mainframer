## Recipe: Setup Remote Machine

Bash script that allows easily add new users with SSH keys to the remote machine.

How to use:

```
$ scp remote_machine_setup.sh root@remote-machine
$ ssh root@remote-machine 'remote_machine_setup.sh new_user_name "new_user_ssh_public_key"'
```

Content of `remote_machine_setup.sh`:

```bash
#!/bin/bash
set -xe

# Params:
# 1 — New user name.
# 2 — Authorized SSH key.

NEW_USER="$1"
AUTHORIZED_SSH_KEY="$2"

if [ -z "$NEW_USER" ]; then
	echo "Please specify new user name."
	exit 1
fi

if [ -z "$AUTHORIZED_SSH_KEY" ]; then
	echo "Please specify authorized ssh key."
	exit 1
fi

# Create user.
useradd "$NEW_USER" --create-home

# Set bash as default shell for new user, so you could place env vars and other things into `~/.bashrc`.
chsh -s /bin/bash "$NEW_USER"

# Set user directory as "working dir".
pushd "/home/$NEW_USER"

# Setup ssh access.
mkdir -p .ssh
chmod u+rwx,go= .ssh
echo "$AUTHORIZED_SSH_KEY" > .ssh/authorized_keys
chmod u+rw,go= .ssh/authorized_keys

# Example of adding env variable to user's ~/.bashrc.
mv .bashrc .bashrc_original
echo -e "export SOME_ENV_VAR_YOU_NEED=value\n" > .bashrc
cat .bashrc_original >> .bashrc
rm .bashrc_original

# TODO (optional): install tools you need to perform remote commands, i.e. packages, SDKs, so on.

# Set correct ownership to all user's files.
chown -R "$NEW_USER":"$NEW_USER" "/home/$NEW_USER/"

echo "New user $NEW_USER was set up correctly."

popd
```
