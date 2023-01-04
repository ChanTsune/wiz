FROM ubuntu:22.04 as llvm

RUN apt-get update && apt-get upgrade -y

RUN apt-get install -y git curl gcc lzma-dev libssl-dev pkg-config

RUN apt-get install -y llvm-14 clang-14

RUN ln -s $(which clang-14) /usr/bin/clang

FROM llvm as rust

RUN apt-get install -y zsh

ENV LLVM_SYS_140_PREFIX="/usr/lib/llvm-14"

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup-init && \
    chmod +x rustup-init && \
    ./rustup-init -y; \
    . $HOME/.cargo/env; \
    rustup --version; \
    cargo --version; \
    rustc --version;


FROM rust as wiz

ENV WIZ_HOME="/wiz/"
ENV WIZ_VERSION="0.0.0"

COPY ./wiz/ ${WIZ_HOME}source/
COPY ./libraries/core ${WIZ_HOME}lib/src/core/${WIZ_VERSION}
COPY ./libraries/libc ${WIZ_HOME}lib/src/libc/${WIZ_VERSION}
COPY ./libraries/std ${WIZ_HOME}lib/src/std/${WIZ_VERSION}

RUN . $HOME/.cargo/env; cargo install --path ${WIZ_HOME}source/wiz --root ${WIZ_HOME}
RUN . $HOME/.cargo/env; cargo install --path ${WIZ_HOME}source/wizc --root ${WIZ_HOME}
ENV PATH="$PATH:/wiz/bin"
