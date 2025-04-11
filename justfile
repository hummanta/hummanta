# List all available commands
default:
    just --list

# Build the project
build profile="dev" target="":
    RUST_BACKTRACE=1 cargo build --workspace --all-features --tests --bins --benches \
        --profile {{profile}} {{ if target != "" { "--target " + target } else { "" } }}

# Clean the build artifacts
clean:
    cargo clean --verbose

# Linting
clippy:
   cargo clippy --workspace --all-features --tests --bins --benches -- -D warnings

# Check formatting
fmt:
    cargo +nightly fmt --all -- --check

# Test the project
test:
    RUST_BACKTRACE=1 cargo test --workspace --all-features --verbose

# Run all the checks
check:
    just fmt
    just clippy
    just test

# Package executables and generate checksums
package profile="dev" target="" version="":
    cargo run --package hmt-packager -- \
        --profile={{profile}} --target={{target}} --version={{version}}

# Run all commend in the local environment
all:
    just clean
    just check
    just build dev
    just package dev "" local
