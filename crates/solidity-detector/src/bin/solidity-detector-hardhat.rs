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

pub struct SolidityHardhatDetector;

/// Implements the Detector trait for SolidityHardhatDetector.
///
/// Detects Solidity projects by verifying if a "package.json" file exists in
/// the specified path and if there is at least one ".sol" file in the
/// directory.
impl Detector for SolidityHardhatDetector {
    fn detect(&self, context: &DetectContext) -> DetectResult {
        let path = &context.path;

        let package_json_exists = path.join("package.json").exists();
        let has_sol_file = WalkDir::new(path)
            .into_iter()
            .filter_map(Result::ok)
            .any(|entry| entry.path().extension().is_some_and(|ext| ext == "sol"));

        if package_json_exists && has_sol_file {
            DetectResult::pass("Solidity".to_string())
        } else {
            DetectResult::fail()
        }
    }
}

/// Run the Solidity hardhat detector.
fn main() {
    command::run(SolidityHardhatDetector);
}
