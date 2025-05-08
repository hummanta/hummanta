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

use serde::{Deserialize, Serialize};

/// The result of the detection.
#[derive(Serialize, Deserialize, Debug)]
pub struct DetectResult {
    /// Whether the detection was successful
    pub pass: bool,

    /// The detected source code type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// File extension for the programming language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
}

impl DetectResult {
    /// Shortcut to create a successful detection result.
    #[inline]
    pub fn pass(language: String, extension: String) -> Self {
        Self { pass: true, language: Some(language), extension: Some(extension) }
    }

    /// Shortcut to create a failed detection result.
    #[inline]
    pub fn fail() -> Self {
        Self { pass: false, language: None, extension: None }
    }
}

impl std::str::FromStr for DetectResult {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl std::fmt::Display for DetectResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).expect("Failed to serialize DetectResult"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pass() {
        let result = DetectResult::pass("Rust".to_string(), "rs".to_string());
        assert!(result.pass);
        assert_eq!(result.language, Some("Rust".to_string()));
        assert_eq!(result.extension, Some("rs".to_string()));
    }

    #[test]
    fn test_fail() {
        let result = DetectResult::fail();
        assert!(!result.pass);
        assert_eq!(result.language, None);
        assert_eq!(result.extension, None)
    }
}
