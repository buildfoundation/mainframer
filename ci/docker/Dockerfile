FROM ubuntu:16.04

MAINTAINER Mainframer Team

COPY ci/rust.version /var/rust.version

# "sudo": switch user in entrypoint.
# "curl rustup": build Mainframer and sample Rust project.
# "openssh-server": testing.
# "openjdk-8-jdk": build sample Gradle, Maven, Buck projects.
# "golang": build sample Go project.
# "clang": build sample Clang project.
# "build-essential": build sample GCC project.
# "lib32stdc++6 lib32z1 unzip": build sample Android project.
# "ant python git": build sample Buck project.
RUN apt-get update --quiet > /dev/null && \
    apt-get --assume-yes --quiet install sudo openssh-server openjdk-8-jdk \
    golang clang build-essential lib32stdc++6 lib32z1 unzip ant python git curl

RUN groupadd build_user && \
    useradd --shell /bin/bash --comment "User for container" --create-home -g build_user build_user && \
    usermod -a -G sudo build_user && \
    su build_user && \
    whoami

RUN curl --silent --fail --location curl https://sh.rustup.rs | sh -s -- -y --default-toolchain="$(cat /var/rust.version)" && \
    export PATH="$PATH:$HOME/.cargo/bin" && \
    echo "$PATH" && \
    rustc --version && cargo --version && \
    rustup component add clippy-preview && \
    echo "clippy version " && cargo clippy --version

# Install Android SDK.
ENV ANDROID_SDK_FILE_NAME tools_r25.2.3-linux.zip
ENV ANDROID_HOME /opt/android-sdk-linux
ENV PATH ${PATH}:${ANDROID_HOME}/tools:${ANDROID_HOME}/tools/bin:${ANDROID_HOME}/platform-tools
ENV ANDROID_SDK_INSTALL_COMPONENT "echo \"y\" | \"$ANDROID_HOME\"/tools/bin/sdkmanager --verbose"

# Give all users access to Android SDK (otherwise build_user won't be able to access it).
RUN mkdir -p $ANDROID_HOME && \
    curl https://dl.google.com/android/repository/$ANDROID_SDK_FILE_NAME --progress-bar --location --output $ANDROID_SDK_FILE_NAME && \
    echo "Unzipping Android SDK" && \
    unzip -qq $ANDROID_SDK_FILE_NAME -d $ANDROID_HOME && \
    rm $ANDROID_SDK_FILE_NAME && \
    echo "Installing Android SDK components" && \
    eval $ANDROID_SDK_INSTALL_COMPONENT '"tools"' > /dev/null && \
    eval $ANDROID_SDK_INSTALL_COMPONENT '"platform-tools"' > /dev/null && \
    eval $ANDROID_SDK_INSTALL_COMPONENT '"build-tools;25.0.2"' > /dev/null && \
    eval $ANDROID_SDK_INSTALL_COMPONENT '"platforms;android-25"' > /dev/null && \
    chmod -R a+rwx "$ANDROID_HOME"

# Entrypoint script will allow us run as non-root in the container.
COPY ci/docker/entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
