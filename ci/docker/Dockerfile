FROM ubuntu:16.04

MAINTAINER Mainframer Team

# "sudo": switch user in entrypoint.
# "openssh-server": testing.
# "openjdk-8-jdk": build sample Gradle, Maven, Buck projects.
# "golang": build sample Go project.
# "clang": build sample Clang project.
# "build-essential": build sample GCC project.
# "lib32stdc++6 lib32z1 unzip": build sample Android project.
# "ant python git": build sample Buck project.
# "curl rustup": build sample Rust project.
RUN apt-get update --quiet && \
    apt-get --assume-yes --quiet install sudo openssh-server openjdk-8-jdk \
    golang clang build-essential lib32stdc++6 lib32z1 unzip ant python git curl && \
    curl -sf -L https://static.rust-lang.org/rustup.sh | sh && \
    cd /root && git clone https://github.com/facebook/buck.git && cd buck && git checkout v2016.11.11.01 && ant

# Install Android SDK.
ENV ANDROID_SDK_FILE_NAME tools_r25.2.3-linux.zip
ENV ANDROID_HOME /opt/android-sdk-linux
ENV PATH ${PATH}:${ANDROID_HOME}/tools:${ANDROID_HOME}/tools/bin:${ANDROID_HOME}/platform-tools
ENV ANDROID_SDK_INSTALL_COMPONENT "echo \"y\" | \"$ANDROID_HOME\"/tools/bin/sdkmanager --verbose"

RUN mkdir -p $ANDROID_HOME && \
	curl https://dl.google.com/android/repository/$ANDROID_SDK_FILE_NAME --progress-bar --location --output $ANDROID_SDK_FILE_NAME && \
	unzip $ANDROID_SDK_FILE_NAME -d $ANDROID_HOME && \
	rm $ANDROID_SDK_FILE_NAME && \
	eval $ANDROID_SDK_INSTALL_COMPONENT '"tools"' && \
    eval $ANDROID_SDK_INSTALL_COMPONENT '"platform-tools"' && \
    eval $ANDROID_SDK_INSTALL_COMPONENT '"build-tools;25.0.2"' && \
    eval $ANDROID_SDK_INSTALL_COMPONENT '"platforms;android-25"'

# Entrypoint script will allow us run as non-root in the container.
COPY ci/docker/entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
