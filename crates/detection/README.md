# hummanta-detection

`hummanta-detection` is a Rust library that provides helper functions for command-line detection tools. It offers a unified interface for writing language-specific detectors and ensures consistent command-line argument parsing and JSON output but does not itself provide CLI capabilities.

## Features
- Provides a standard interface for writing custom detectors.
- Supports command-line argument parsing with `clap`.
- Outputs results in JSON format.
- Supports specifying paths via CLI arguments or environment variables.

## Installation

To use `hummanta-detection` as a library, add the following to your `Cargo.toml`:

```toml
[dependencies]
hummanta-detection = { git = "https://github.com/hummanta/hummanta.git" }
```

## Usage

You can integrate `hummanta-detection` into your Rust project and define custom detectors:

```rust
use hummanta_detection::{DetectContext, DetectResult, Detector};

struct MyDetector;

impl Detector for MyDetector {
    fn detect(&self, context: &DetectContext) -> DetectResult {
        if context.path.ends_with(".sol") {
            DetectResult::pass("Solidity".to_string())
        } else {
            DetectResult::fail()
        }
    }
}
```

Then, in your `main.rs`, use the `run` function to execute it:

```rust
use hummanta_detection::command;

fn main() {
    command::run(MyDetector);
}
```

```bash
cargo run -- --path path/to/solidity-file.sol
```

Example Output

```json
{
  "pass": true,
  "language": "Solidity"
}
```
