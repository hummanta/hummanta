// Copyright (c) The Hummanta Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Unified command-line interface for the detection tool.

use std::path::PathBuf;

use clap::Parser;

use crate::{DetectContext, Detector};

#[derive(Parser, Debug)]
pub struct Arguments {
    /// The path to the file or directory to detect.
    #[clap(long, env = "DETECT_PATH")]
    pub path: Option<String>,
}

/// Runs a detector and prints the result as JSON.
pub fn run<T: Detector>(detector: T) {
    let args = Arguments::parse();

    let path = args.path.unwrap_or_else(|| {
        eprintln!("No path provided. Use --path <path> or set DETECT_PATH env variable.");
        std::process::exit(1);
    });

    let context = DetectContext::new(PathBuf::from(path));
    let result = detector.detect(&context);

    // Print the result as JSON, or an error message if serialization fails.
    if let Ok(json) = serde_json::to_string(&result) {
        println!("{}", json);
    } else {
        eprintln!("Failed to serialize the detection result.");
    }
}

#[cfg(test)]
mod tests {
    use crate::{command::run, DetectContext, DetectResult, Detector};

    #[test]
    fn test_run_with_env() {
        // Create a dummy detector that always returns the same result.
        struct DummyDetector;

        impl Detector for DummyDetector {
            fn detect(&self, _context: &DetectContext) -> DetectResult {
                DetectResult::pass("Rust".to_string())
            }
        }

        // Provide a test path
        std::env::set_var("DETECT_PATH", "dummy_path.rs");

        // Run the detector
        run(DummyDetector);

        // Unset the environment variable
        std::env::remove_var("DETECT_PATH");
    }
}
