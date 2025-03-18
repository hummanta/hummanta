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

use std::path::Path;

use hummanta_detection::{command, DetectContext, DetectResult, Detector};

pub struct SolidityFileDetector;

/// Implements the Detector trait for SolidityFileDetector.
///
/// Detects Solidity files by checking if the path ends with ".sol" or includes
/// such files in its first level.
impl Detector for SolidityFileDetector {
    fn detect(&self, context: &DetectContext) -> DetectResult {
        let path = &context.path;

        if is_solidity_file(path) {
            return DetectResult::pass("Solidity".to_string());
        }

        if path.is_dir() {
            if let Ok(entries) = path.read_dir() {
                for entry in entries.flatten() {
                    if is_solidity_file(&entry.path()) {
                        return DetectResult::pass("Solidity".to_string());
                    }
                }
            }
        }

        DetectResult::fail()
    }
}

/// Check if the file is a Solidity file.
fn is_solidity_file(path: &Path) -> bool {
    path.extension().is_some_and(|ext| ext == "sol")
}

/// Run the Solidity file detector.
fn main() {
    command::run(SolidityFileDetector);
}
