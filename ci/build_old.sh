#!/bin/bash
set -e

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_DIR="$DIR/.."

pushd "$PROJECT_DIR"

# Files created in mounted volume by container should have same owner as host machine user to prevent chmod problems.
USER_ID=`id -u $USER`

# BUILD_COMMAND will run INSIDE Docker container.
BUILD_COMMAND="set -xe && "
BUILD_COMMAND+="apt-get update --quiet && "

# Testing. 
BUILD_COMMAND+="apt-get --assume-yes --quiet install openssh-server && "

# Tool to find issues in shell scripts.
BUILD_COMMAND+="apt-get --assume-yes --quiet install shellcheck && "

# Install dependencies for sample projects.
## Gradle, Maven, Buck.
BUILD_COMMAND+="apt-get --assume-yes --quiet install openjdk-8-jdk && "

## Rust.
BUILD_COMMAND+="curl -sf -L https://static.rust-lang.org/rustup.sh | sh && "

## Go.
BUILD_COMMAND+="apt-get --assume-yes --quiet install golang && "

## Clang.
BUILD_COMMAND+="apt-get --assume-yes --quiet install clang && "

## GCC.
BUILD_COMMAND+="apt-get --assume-yes --quiet install build-essential && "

## Android SDK.
BUILD_COMMAND+="apt-get --assume-yes --quiet install lib32stdc++6 lib32z1 && "

## Buck.
BUILD_COMMAND+="apt-get --assume-yes --quiet install ant python git && "

if [ "$USER_ID" == "0" ]; then
    echo "Warning: running as r00t."
else
    BUILD_COMMAND+="apt-get --assume-yes install sudo && "
    BUILD_COMMAND+="groupadd --gid $USER_ID build_user && "
    BUILD_COMMAND+="useradd --shell /bin/bash --uid $USER_ID --gid $USER_ID --create-home build_user && "
    BUILD_COMMAND+="sudo --set-home --preserve-env -u build_user "
fi

# Setup ssh for tests.
BUILD_COMMAND+="eval \"mkdir -p ~/.ssh/\" && "
BUILD_COMMAND+="eval \"chmod u+rwx,go= ~/.ssh/\" && "
BUILD_COMMAND+="eval \"ssh-keygen -b 2048 -t rsa -f ~/.ssh/id_rsa -q -N \"\"\" && "
BUILD_COMMAND+="eval \"cp ~/.ssh/id_rsa.pub ~/.ssh/authorized_keys\" && "
BUILD_COMMAND+="eval \"chmod u+rw,go= ~/.ssh/authorized_keys\" && "
BUILD_COMMAND+="eval \"ssh-keyscan -t rsa localhost > ~/.ssh/known_hosts\" && "
BUILD_COMMAND+="eval \"chmod u+rw,go= ~/.ssh/known_hosts\" && "

# Setup Go env.
BUILD_COMMAND+="eval \"mv ~/.bashrc ~/.bashrc_original && echo -e \"export GOROOT=$GOROOT\n\" > ~/.bashrc && cat ~/.bashrc_original >> ~/.bashrc && rm ~/.bashrc_original\" && "

# Setup Android SDK.
BUILD_COMMAND+="export ANDROID_HOME=/opt/android-sdk-linux && "
BUILD_COMMAND+="export ANDROID_SDK_FILE_NAME=android-sdk_r24.4.1-linux.tgz && "
BUILD_COMMAND+="eval \"export ANDROID_SDK_INSTALL_COMPONENT=\"echo \"y\" | \"$ANDROID_HOME\"/tools/android update sdk --no-ui --all --filter\"\" && "
BUILD_COMMAND+="eval \"mkdir -p \"$ANDROID_HOME\"\" && "
BUILD_COMMAND+="eval \"curl https://dl.google.com/android/$ANDROID_SDK_FILE_NAME --progress-bar --location --output $ANDROID_SDK_FILE_NAME\" && "
BUILD_COMMAND+="eval \"tar -xzf $ANDROID_SDK_FILE_NAME --directory /opt/\" && "
BUILD_COMMAND+="eval \"$ANDROID_HOME\"/tools/android list sdk --all --no-ui --extended && "
BUILD_COMMAND+="eval \"eval $ANDROID_SDK_INSTALL_COMPONENT \"tools\"\" && "
BUILD_COMMAND+="eval \"eval $ANDROID_SDK_INSTALL_COMPONENT \"platform-tools\"\" && "
BUILD_COMMAND+="eval \"eval $ANDROID_SDK_INSTALL_COMPONENT \"build-tools-25.0.2\"\" && "
BUILD_COMMAND+="eval \"eval $ANDROID_SDK_INSTALL_COMPONENT \"android-25\"\" && "
BUILD_COMMAND+="eval \"mv ~/.bashrc ~/.bashrc_original\" && "
BUILD_COMMAND+="eval \"echo -e \"export ANDROID_HOME=$ANDROID_HOME\nJAVA_HOME=$JAVA_HOME\" > ~/.bashrc\" && "
BUILD_COMMAND+="eval \"cat ~/.bashrc_original >> ~/.bashrc\" && "
BUILD_COMMAND+="eval \"rm ~/.bashrc_original\" && "

# Setup Buck.
BUILD_COMMAND+="git clone https://github.com/facebook/buck.git && "
BUILD_COMMAND+="cd buck && "
BUILD_COMMAND+="git checkout v2016.11.11.01 && "
BUILD_COMMAND+="ant && "
BUILD_COMMAND+="eval \"mv ~/.bashrc ~/.bashrc_original\" && "
BUILD_COMMAND+="eval \"echo -e \"export PATH=$PATH:`pwd`/bin\n\" > ~/.bashrc\" && "
BUILD_COMMAND+="eval \"cat ~/.bashrc_original >> ~/.bashrc\" && "
BUILD_COMMAND+="eval \"rm ~/.bashrc_original\" && "
BUILD_COMMAND+="cd .. && "

# Run tests and samples.
BUILD_COMMAND+="bash /opt/project/test/test.sh --run-samples"

docker run \
--volume `"pwd"`:/opt/project \
ubuntu:16.04 \
bash -c "$BUILD_COMMAND"

popd
