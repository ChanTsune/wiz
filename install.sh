#!/bin/sh

WIZ_HOME=${WIZ_HOME:-"$HOME/.wiz"}
BIN_DIR="$WIZ_HOME/bin"
LIB_DIR="$WIZ_HOME/lib"

echo "WIZ_HOME=$WIZ_HOME"
echo "BIN_DIR=$BIN_DIR"
echo "LIB_DIR=$LIB_DIR"

main() {
    mkdir -p "$BIN_DIR"
    build_install "wiz"
    build_install "wizc"

    install_builtin_lib

    install_shell_env

    echo "Installation completed at $BIN_DIR"
    ENV_SCRIPT=". \"\$HOME/.wiz/env\""
    case "$(cat ~/.zshrc)" in
        *"$ENV_SCRIPT"*)
        ;;
        *)
        echo "$ENV_SCRIPT" >> ~/.zshrc
    esac
}

build_install() {
    TMP="$(pwd)"
    cd "wiz/$1"
    cargo build --release
    cp "target/release/$1" "$BIN_DIR/$1"
    cd "$TMP"
}

install_builtin_lib() {
    mkdir -p "$LIB_DIR/src"
    copy_lib_src core
    copy_lib_src std
}

install_shell_env() {
    cp env "$WIZ_HOME"
}

copy_lib_src() {
    cp -r "$1" "$LIB_DIR/src/$1"
}

main
