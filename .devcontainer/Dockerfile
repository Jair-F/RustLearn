FROM mcr.microsoft.com/vscode/devcontainers/base:ubuntu-24.04
# installing uv
COPY --from=ghcr.io/astral-sh/uv:latest /uv /uvx /bin/

ENV DEBIAN_FRONTEND=noninteractive
USER root
WORKDIR /workspace

RUN apt update && apt upgrade -y
RUN apt update && apt-get install -y --no-install-recommends \
    build-essential \
    ca-certificates \
    curl \
    git \
    libssl-dev \
    pkg-config \
    lldb

ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:$PATH

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path \
    && chmod -R a+w $RUSTUP_HOME $CARGO_HOME \
    && rustup component add rust-analyzer rust-src

RUN cargo install cargo-audit
RUN apt install -y python3
