FROM rust:1.43.1

ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# Configure apt and install packages
RUN apt-get update \
    && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends apt-utils dialog 2>&1 \
    # Install lldb, vadimcn.vscode-lldb VSCode extension dependencies
    && apt-get install -y lldb python3-minimal libpython3.7 \
    # Install Rust components
    && rustup update 2>&1 \
    && rustup component add rls rust-analysis rust-src rustfmt clippy 2>&1 \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
    