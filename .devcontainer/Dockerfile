# Use the [Choice] comment to indicate option arguments that should appear in VS Code UX. Use a comma separated list.
#
# [Choice] Debian OS version: bullseye, buster
ARG VARIANT="bullseye"
ARG WASM_BINDGEN_VERSION="0.2.83"
# FROM buildpack-deps:${VARIANT}-curl
FROM mcr.microsoft.com/devcontainers/rust:1-${VARIANT}
# Use the [Option] comment to specify true/false arguments that should appear in VS Code UX
#
# [Option] Install zsh
ARG INSTALL_ZSH="true"
# [Option] Upgrade OS packages to their latest versions
ARG UPGRADE_PACKAGES="false"
# Install needed packages and setup non-root user. Use a separate RUN statement to add your own dependencies.
ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID
RUN apt-get install libcurl4-openssl-dev \
    && curl -s https://packagecloud.io/install/repositories/wasmcloud/core/script.deb.sh | bash && apt install wash \
    && which wash
RUN echo $WASM_BINDGEN_VERSION
RUN cargo install -f wasm-bindgen-cli --vers 0.2.83
RUN cargo install -f wasm-snip
RUN cargo install -f cargo-make
RUN rustup target add wasm32-unknown-unknown
RUN apt-get install iputils-ping -y
RUN sudo mkdir -p /home/vscode/.wash && sudo chmod 777 /home/vscode/.wash
