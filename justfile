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

# Generate the manifests
manifest local="true" version="":
    cargo run --package hummanta-manifest -- \
        --path manifests --version={{version}} \
        {{ if local == "true" { "--local" } else { "" } }}

# Package executables and generate checksums
package profile="dev" target="" version="":
    cargo run --package hummanta-packager -- \
        --profile={{profile}} --target={{target}} --version={{version}}

# Release the project in the local environment
release local="true" profile="dev" target="" version="":
    just build {{profile}} {{target}}
    just package {{profile}} {{target}} {{version}}
    just manifest {{local}} {{version}}

# Link local development build as a version
link:
    mkdir -p ~/.hummanta/manifests/local
    cp -r target/artifacts/manifests/* ~/.hummanta/manifests/local

# Run all commend in the local environment
all:
    just check
    just build dev
    just package dev "" local
    just manifest true local
    just link
