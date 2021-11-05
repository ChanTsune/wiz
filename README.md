# Wiz programming language

![wiz](./icon.svg)

Programming is modern-day magic


## Getting start

### Requirments

- Rust 1.52 or later
- llvm-11

### Setup

1. Install Rust

Install Rust from https://www.rust-lang.org/tools/install

2. Install LLVM

We recomended use `llvmenv` for install llvm.

```bash
cargo install llvmenv
```

```bash
llvmenv init
llvmenv build-entry 11
llvmenv global 11
```

3. Build and Install wiz

```bash
./build.sh
```

4. Enable wiz

```bash
source ($HOME/.wiz/env) 
```
