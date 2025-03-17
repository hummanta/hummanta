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

//! Detect the source code type of the current project and return the detect result.

pub mod command;
mod context;
mod result;

pub use context::DetectContext;
pub use result::DetectResult;

/// A trait for source code detectors.
pub trait Detector {
    fn detect(&self, context: &DetectContext) -> DetectResult;
}

#[cfg(test)]
mod tests {
    use crate::{DetectContext, DetectResult, Detector};

    #[test]
    fn test_dummy_detector() {
        // Create a dummy detector that always returns the same result.
        struct DummyDetector;

        impl Detector for DummyDetector {
            fn detect(&self, _context: &DetectContext) -> DetectResult {
                DetectResult::pass("Rust".to_string())
            }
        }

        // Provide a test path
        let context = DetectContext::new("dummy_path.rs".into());

        // Run detection directly
        let result = DummyDetector.detect(&context);

        // Assert expected outcome
        assert!(result.pass);
        assert_eq!(result.language, Some("Rust".to_string()));
    }
}
