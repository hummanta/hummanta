# List all available commands
default:
    just --list

# Build the project
build options="":
    RUST_BACKTRACE=1 cargo build {{options}} --workspace --all-features --tests --bins --benches

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

# Generate the manifests
manifest local="true" options="": (build options)
    cargo run {{options}} \
        --package hummanta-manifest-generator -- \
        --path manifests {{ if local == "true" { "--local" } else { "" } }}
