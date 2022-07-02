FROM ubuntu:22.04

RUN apt-get update

RUN apt update && apt upgrade -y

RUN apt install -y curl gcc lzma-dev

RUN apt install -y llvm-12

RUN apt install -y clang-12

RUN ln -s $(which clang-12) /usr/bin/clang

ENV LLVM_SYS_120_PREFIX="/usr/lib/llvm-12"

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup-init && \
    chmod +x rustup-init && \
    ./rustup-init -y; \
    . $HOME/.cargo/env; \
    rustup --version; \
    cargo --version; \
    rustc --version;
