# List all available commands
default:
    just --list

# Build the project
build profile="dev" target="":
    cargo build --workspace --all-features --all-targets \
        --profile {{profile}} {{ if target != "" { "--target " + target } else { "" } }}

# Clean the build artifacts
clean:
    cargo clean --verbose

# Linting
clippy:
   cargo clippy --workspace --all-features --all-targets -- -D warnings

# Check formatting
fmt:
    cargo +nightly fmt --all -- --check

# Test the project
test:
    cargo test --workspace --all-features --all-targets

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

# Build and install all binaries for testing
install:
    cargo install --path crates/hmt-cli
    cargo install --path crates/hmt-manifest
    cargo install --path crates/hmt-packager
