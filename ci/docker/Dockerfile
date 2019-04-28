FROM ubuntu:16.04

MAINTAINER Mainframer Team

COPY ci/rust.version /var/rust.version

# "sudo": switch user in entrypoint.
# "curl", "build-essential": build Mainframer.
# "openssh-server", "rsync": testing.
RUN apt-get update --quiet > /dev/null && \
    apt-get --assume-yes --quiet install \
    sudo \
    curl \
    build-essential \
    openssh-server \
    rsync && \
    apt-get --assume-yes clean

RUN groupadd build_user && \
    useradd --shell /bin/bash --comment "User for container" --create-home -g build_user build_user && \
    usermod -a -G sudo build_user

RUN su build_user -c 'curl --silent --fail --location curl https://sh.rustup.rs | sh -s -- -y --default-toolchain="$(cat /var/rust.version)" && \
    source "$HOME/.cargo/env" && \
    echo "$PATH" && \
    rustc --version && cargo --version && \
    rustup component add clippy-preview && \
    echo "clippy version " && cargo clippy --version'

# Entrypoint script will allow us run as non-root in the container.
COPY ci/docker/entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
