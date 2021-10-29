#!/bin/sh

WIZ_HOME=${WIZ_HOME:-"$HOME/.wiz"}
INSTALL_DIR="$WIZ_HOME/bin"

echo "WIZ_HOME=$WIZ_HOME"
echo "INSTALL_DIR=$INSTALL_DIR"

main() {
    mkdir -p $INSTALL_DIR
    build_install "wiz"
    build_install "wizc"
    echo "Installation completed at $INSTALL_DIR"
    echo "Add $INSTALL_DIR to your PATH"
    echo 'export WIZ_HOME=$HOME/.wiz'
    echo 'PATH="$WIZ_HOME/bin:$PATH"'
}

build_install() {
    pushd .
    cd "wiz/$1"
    cargo build --release
    cp "target/release/$1" "$INSTALL_DIR/$1"
    popd    
}

main
