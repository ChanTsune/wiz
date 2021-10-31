#!/bin/sh

WIZ_HOME=${WIZ_HOME:-"$HOME/.wiz"}
BIN_DIR="$WIZ_HOME/bin"
LIB_DIR="$WIZ_HOME/lib"

echo "WIZ_HOME=$WIZ_HOME"
echo "BIN_DIR=$BIN_DIR"

main() {
    mkdir -p "$BIN_DIR"
    build_install "wiz"
    build_install "wizc"

    install_builtin_lib

    echo "Installation completed at $BIN_DIR"
    echo "Add $BIN_DIR to your PATH"
    echo 'export WIZ_HOME=$HOME/.wiz'
    echo 'PATH="$WIZ_HOME/bin:$PATH"'
}

build_install() {
    pushd .
    cd "wiz/$1"
    cargo build --release
    cp "target/release/$1" "$BIN_DIR/$1"
    popd
}

install_builtin_lib() {
    mkdir -p "$LIB_DIR/src"
    copy_lib_src builtin
    copy_lib_src std
}

copy_lib_src() {
    cp -r "$1" "$LIB_DIR/src/$1"
}

main
