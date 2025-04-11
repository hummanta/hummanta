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

use hmt_detection::{command, DetectContext, DetectResult, Detector};
use walkdir::WalkDir;

pub struct SolidityFileDetector;

/// Implements the Detector trait for SolidityFileDetector.
///
/// Detects Solidity files by verifying if any file in the specified path has a
/// ".sol" extension.
impl Detector for SolidityFileDetector {
    fn detect(&self, context: &DetectContext) -> DetectResult {
        if WalkDir::new(&context.path)
            .into_iter()
            .filter_map(Result::ok)
            .any(|entry| entry.path().extension().is_some_and(|ext| ext == "sol"))
        {
            return DetectResult::pass("Solidity".to_string());
        }

        DetectResult::fail()
    }
}

/// Run the Solidity file detector.
fn main() {
    command::run(SolidityFileDetector);
}
