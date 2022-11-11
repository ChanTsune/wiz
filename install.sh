#!/bin/sh

set -eu

WIZ_HOME=${WIZ_HOME:-"$HOME/.wiz"}
BIN_DIR="$WIZ_HOME/bin"
LIB_DIR="$WIZ_HOME/lib"
VERSION="0.0.0"

SCRIPT_DIR="$(dirname $0)"

echo "WIZ_HOME=$WIZ_HOME"
echo "BIN_DIR=$BIN_DIR"
echo "LIB_DIR=$LIB_DIR"

check_commands() {
    need_cmd mkdir
    need_cmd touch
    need_cmd cat
    need_cmd cp
    need_cmd echo
    need_cmd cargo
    need_cmd clang
}

main() {
    check_commands

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
    FROM="$SCRIPT_DIR/wiz/target/release/$1"
    if [ ! -e "$FROM" ]; then
        cargo build --bin "$1" --release --manifest-path wiz/Cargo.toml
    fi
    cp "$FROM" "$BIN_DIR/$1"
}

install_builtin_lib() {
    mkdir -p "$LIB_DIR/src"
    copy_lib_src core "$VERSION"
    copy_lib_src std "$VERSION"
    copy_lib_src libc "$VERSION"
}

install_shell_env() {
    cp env "$WIZ_HOME"
}

copy_lib_src() {
    cp -r "$SCRIPT_DIR/libraries/$1" "$LIB_DIR/src/$1"
    cp -r "$SCRIPT_DIR/libraries/$1" "$LIB_DIR/src/$1/$2"
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

main "$@"
