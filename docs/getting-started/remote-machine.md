# Getting Started on Remote Machine

## Dependencies

* SSH server

That’s it. There is no need to install Mainframer on the remote machine.

## Users

We recommend creating a OS-level user per person
and using real names to help with distinguishing people.
For example, a username can be in a format of `<FIRST_NAME>_<LAST_NAME>`.

There are different options, like a Docker container per person,
but we’ll leave it up to you to decide.

### Linux

Be advised that Mainframer runs SSH in non-interactive mode.
Some default Bash `.bashrc` configurations stop evaluation in this mode.
In such cases place custom environment variable declarations
and script invocations before the following line.

```bash
# If not running interactively, don't do anything
```

We’ve built a Bash script which might help with creating Linux users.

Usage:

```console
$ bash create-user.sh "USER_NAME" "USER_SSH_PUBLIC_KEY"
```

`create-user.sh` contents:

```bash
#!/bin/bash
set -eux

USER_NAME="$1"
USER_SSH_PUBLIC_KEY="$2"

if [ -z "${USER_NAME}" ]; then
  echo "Error: user name is not provided."
  exit 1
fi

if [ -z "${USER_SSH_PUBLIC_KEY}" ]; then
  echo "Error: user SSH public key is not provided."
  exit 1
fi

echo ":: Creating user [${USER_NAME}]..."

# Create user.
useradd "${USER_NAME}" --create-home

# Change shell to Bash.
chsh -s /bin/bash "${USER_NAME}"

# Switch to user directory.
pushd "/home/${USER_NAME}"

# Configure SSH access.
SSH_DIR=".ssh"
SSH_KEYS_FILE="${SSH_DIR}"/authorized_keys

mkdir -p "${SSH_DIR}"
touch "${SSH_KEYS_FILE}"

chmod u+rw "${SSH_DIR}"
chmod u+rw "${SSH_KEYS_FILE}"
echo "${USER_SSH_PUBLIC_KEY}" > "${SSH_KEYS_FILE}"

# ATTENTION
# You can install required tools, packages and SDKs in this step.
# The following commented commands are an example of installing Android SDK.

# ANDROID_SDK_FILE="android-sdk.zip"
# ANDROID_SDK_DIR="android-sdk"

# curl --location "https://dl.google.com/android/repository/sdk-tools-linux-4333796.zip" --output "${ANDROID_SDK_FILE}"
# unzip -q "${ANDROID_SDK_FILE}" -d "${ANDROID_SDK_DIR}"
# rm "${ANDROID_SDK_FILE}"

# mv .bashrc .bashrc_original
# echo -e "export ANDROID_HOME=/home/${USER_NAME}/${ANDROID_SDK_DIR}\n" >> .bashrc
# cat .bashrc_original >> .bashrc
# rm .bashrc_original

# Change ownership to all affected files.
chown -R "${USER_NAME}":"${USER_NAME}" "/home/${USER_NAME}/"

echo ":: Created user [${USER_NAME}]!"

# Switch from user directory.
popd
```

