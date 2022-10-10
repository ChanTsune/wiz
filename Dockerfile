FROM ubuntu:22.04 as llvm

RUN apt-get update && apt-get upgrade -y

RUN apt-get install -y git curl gcc lzma-dev

RUN apt-get install -y llvm-13 clang-13

RUN ln -s $(which clang-12) /usr/bin/clang

FROM llvm as rust

RUN apt-get install -y zsh

ENV LLVM_SYS_120_PREFIX="/usr/lib/llvm-12"

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup-init && \
    chmod +x rustup-init && \
    ./rustup-init -y; \
    . $HOME/.cargo/env; \
    rustup --version; \
    cargo --version; \
    rustc --version;

RUN $HOME/.cargo/bin/cargo install cargo-build-deps

FROM rust as wiz

COPY ./wiz ./wiz
COPY ./libraries ./libraries
COPY ./install.sh ./install.sh
COPY ./env ./env

RUN zsh ./install.sh
