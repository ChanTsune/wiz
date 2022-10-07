FROM ubuntu:22.04 as llvm

RUN apt-get update

RUN apt update && apt upgrade -y

RUN apt install -y curl gcc lzma-dev

RUN apt install -y llvm-13

RUN apt install -y clang-13

RUN ln -s $(which clang-13) /usr/bin/clang

FROM llvm as rust

RUN apt install -y zsh

ENV LLVM_SYS_130_PREFIX="/usr/lib/llvm-13"

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup-init && \
    chmod +x rustup-init && \
    ./rustup-init -y; \
    . $HOME/.cargo/env; \
    rustup --version; \
    cargo --version; \
    rustc --version;


FROM rust as wiz

COPY ./wiz ./wiz
COPY ./libraries ./libraries
COPY ./install.sh ./install.sh
COPY ./env ./env

RUN zsh ./install.sh
