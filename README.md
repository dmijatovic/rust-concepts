# RUST programing language

This repo contains basics of rust progamming language. I want to get feeling of the language and where it can be applied. Rust seem to be propper language for creating web assembly applications.

My goal is to lern the basics and see what could be a good use cases for the language in the situations I encounter as programmer.

## Installation

Installation on unix based system is done running shell script [from the website](https://www.rust-lang.org/learn/get-started)

```bash
# download and run script
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# select option 1 for default installation
# restart to load env from .profile file
```

## Visual Studio Code

You need to install rust plugin and optionally TOML file support plugin and crates (for package version assistance). In addition, after open first rs file VSC might advice to install additiona rust tooling. I opted yes, the following is installed using rustup toolchain installer

```bash
#
rustup component add rust-analyses --toolchain stable-x86_64-unknown-linux-gnu
#
rustup component add rust-src --toolchain stable-x86_64-unknown-linux-gnu
```

### Rust-analyzer plugin and debugging

For debugging in VCS I installed rust-analyzer and [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) plugins.

## Basic commands

```bash
## rust compiler versopm
rustc --version
## cargo version (package manager)
cargo --version
## basic build
rustc build hw.rs
## production build
rustc build hw --optimized
```

## Project skaffold

```bash
# create new rust project
cargo init --bin todo-actix

# check for error
cargo check

# run project in dev
cargo run
# run project in release
carg run --release
# build
cargo build --release
```

## Dependencies

The project dependencies are specified in `Cargo.toml` file. You can add dependencies manually. There is a tool [`cargo-edit`](https://github.com/killercup/cargo-edit) you can use to add, remove and update dependencies.

```bash
# install cargo
cargo install cargo-edit

# add dependecy to project as build dependencies
cargo add actix-rt actix-web dotenv --build

# add as dev dependencies
cargo add regex --dev

```

## Evironment variables

Using dotenv module we can define envroment variables using .env file in the root of the project. The variables in the .env file use . to split props. See example here for server variables.

```env
SERVER.HOST=localhost
SERVER.PORT=8080
```
