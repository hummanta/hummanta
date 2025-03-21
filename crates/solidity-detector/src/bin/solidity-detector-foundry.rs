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

use hummanta_detection::{command, DetectContext, DetectResult, Detector};
use walkdir::WalkDir;

pub struct SolidityFoundryDetector;

/// Implements the Detector trait for SolidityFoundryDetector.
///
/// Detects Solidity projects by verifying if a "foundry.toml" file exists in
/// the specified path and if there is at least one ".sol" file in the
/// directory.
impl Detector for SolidityFoundryDetector {
    fn detect(&self, context: &DetectContext) -> DetectResult {
        let path = &context.path;

        let foundry_toml_exists = path.join("foundry.toml").exists();
        let has_sol_file = WalkDir::new(path)
            .into_iter()
            .filter_map(Result::ok)
            .any(|entry| entry.path().extension().is_some_and(|ext| ext == "sol"));

        if foundry_toml_exists && has_sol_file {
            DetectResult::pass("Solidity".to_string())
        } else {
            DetectResult::fail()
        }
    }
}

/// Run the Solidity foundry detector.
fn main() {
    command::run(SolidityFoundryDetector);
}
