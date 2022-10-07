<!-- PROJECT SHIELDS -->
[![CI][ci-status-shield]][ci-status-url]
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]

<!-- PROJECT LOGO -->
<div align="center">
    <p><a href="https://github.com/ChanTsune/wiz"><img src="./icon.svg" width="120" hight="120"/></a></p>
    <p><h3>Wiz Programming Language</h3></p>
    <p>Programming is modern-day magic
</p>
</div>


## Getting started

### Requirements

|**Tool**|**Version**|
|:-:|:-:|
|Rust|latest|
|llvm|13|

### Setup

1. Install Rust

Install Rust from https://www.rust-lang.org/tools/install.

2. Install LLVM

We recomended using `llvmenv` to install LLVM.

**Using llvmenv**
```bash
cargo install llvmenv
```

```bash
llvmenv init
llvmenv build-entry 13.0
llvmenv global 13.0.0
```

**Using Homebrew on Mac**

```bash
brew install llvm@13
```

```bash
export LLVM_SYS_130_PREFIX="$(brew --prefix llvm@13)"
```

**Using apt on Debian Linux**

```bash
sudo apt install clang-13 llvm-13
```

3. Build and install wiz

```bash
sh install.sh
```

4. Enable wiz

```bash
source "$HOME/.wiz/env"
```

<!-- MARKDOWN LINKS & IMAGES -->
[ci-status-shield]: https://github.com/ChanTsune/wiz/actions/workflows/test.yml/badge.svg
[ci-status-url]: https://github.com/ChanTsune/wiz/actions/workflows/test.yml
[contributors-shield]: https://img.shields.io/github/contributors/ChanTsune/wiz.svg
[contributors-url]: https://github.com/ChanTsune/wiz/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/ChanTsune/wiz.svg
[forks-url]: https://github.com/ChanTsune/wiz/network/members
[stars-shield]: https://img.shields.io/github/stars/ChanTsune/wiz.svg
[stars-url]: https://github.com/ChanTsune/wiz/stargazers
[issues-shield]: https://img.shields.io/github/issues/ChanTsune/wiz.svg
[issues-url]: https://github.com/ChanTsune/wiz/issues
[license-shield]: https://img.shields.io/github/license/ChanTsune/wiz.svg
[license-url]: https://github.com/ChanTsune/wiz/blob/main/LICENSE
