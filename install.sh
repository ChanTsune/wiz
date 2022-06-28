#!/bin/sh

set -eu

WIZ_HOME=${WIZ_HOME:-"$HOME/.wiz"}
BIN_DIR="$WIZ_HOME/bin"
LIB_DIR="$WIZ_HOME/lib"
VERSION="0.0.0"

echo "WIZ_HOME=$WIZ_HOME"
echo "BIN_DIR=$BIN_DIR"
echo "LIB_DIR=$LIB_DIR"

main() {
    need_cmd mkdir
    need_cmd touch
    need_cmd cat
    need_cmd cp
    need_cmd echo
    need_cmd cargo

    mkdir -p "$BIN_DIR"
    build_install "wiz"
    build_install "wizc"

    install_builtin_lib

    install_shell_env

    echo "Installation completed at $BIN_DIR"
    ENV_SCRIPT=". \"\$HOME/.wiz/env\""
    touch ~/.zshrc
    case "$(cat ~/.zshrc)" in
        *"$ENV_SCRIPT"*)
        ;;
        *)
        echo "$ENV_SCRIPT" >> ~/.zshrc
    esac
}

build_install() {
    TMP="$(pwd)"
    cd "wiz"
    cargo build --bin "$1" --release
    cp "target/release/$1" "$BIN_DIR/$1"
    cd "$TMP"
}

install_builtin_lib() {
    mkdir -p "$LIB_DIR/src"
    copy_lib_src core "$VERSION"
    copy_lib_src std "$VERSION"
}

install_shell_env() {
    cp env "$WIZ_HOME"
}

copy_lib_src() {
    cp -r "libraries/$1" "$LIB_DIR/src/$1"
    cp -r "libraries/$1" "$LIB_DIR/src/$1/$2"
}

err() {
    echo "$1" >&2
    exit 1
}

need_cmd() {
    if ! check_cmd "$1"; then
        err "need '$1' (command not found)"
    fi
}

check_cmd() {
    command -v "$1" > /dev/null 2>&1
}

main
