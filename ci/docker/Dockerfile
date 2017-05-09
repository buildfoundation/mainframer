FROM ubuntu:16.04

MAINTAINER Mainframer Team

# "sudo": switch user in entrypoint.
# "openssh-server": testing.
# "shellcheck": find issues in shell scripts.
# "openjdk-8-jdk": build sample Gradle, Maven, Buck projects.
# "golang": build sample Go project.
# "clang": build sample Clang project.
# "build-essential": build sample GCC project.
# "lib32stdc++6 lib32z1": build sample Android project.
# "ant python git": build sample Buck project.
# "curl rustup": build sample Rust project.
RUN apt-get update --quiet && \
    apt-get --assume-yes --quiet install sudo openssh-server shellcheck openjdk-8-jdk \
    golang clang build-essential lib32stdc++6 lib32z1 ant python git curl && \
    curl -sf -L https://static.rust-lang.org/rustup.sh | sh
    
# Entrypoint script will allow us run as non-root in the container.
COPY ci/docker/entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
