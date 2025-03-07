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
    cargo fmt --all -- --check

# Test the project
test:
    RUST_BACKTRACE=1 cargo test --workspace --all-features --verbose
