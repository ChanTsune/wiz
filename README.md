# Wiz programming language

![wiz](./icon.svg)

Programming is modern-day magic


## Getting start

### Requirments

|Tool|version|
|:-:|:-:|
|Rust|latest|
|llvm|11|

### Setup

1. Install Rust

Install Rust from https://www.rust-lang.org/tools/install

2. Install LLVM

We recomended use `llvmenv` for install llvm.

**Use llvmenv**
```bash
cargo install llvmenv
```

```bash
llvmenv init
llvmenv build-entry 11.0
llvmenv global 11.0
```

**Use Homebrew for Mac**

```bash
brew install llvm@11
```

```bash
export LLVM_SYS_110_PREFIX="$(brew --prefix llvm@11)"
```

**Use apt for Debian Linux**

```bash
sudo apt install clang-11 llvm-11
```

3. Build and Install wiz

```bash
sh install.sh
```

4. Enable wiz

```bash
source "$HOME/.wiz/env"
```
