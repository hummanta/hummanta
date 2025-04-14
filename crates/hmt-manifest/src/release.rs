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

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// `ReleaseManifest` describes a specific released version of a package.
///
/// This structure holds detailed information about a released version of the package,
/// including version information and artifact details.
///
/// Example:
/// ```toml
/// [release]
/// version = "v1.2.0"
///
/// [artifacts.x86_64-apple-darwin]
/// url = "https://github.com/hummanta/solidity-detector-foundry/releases/download/v1.2.0/solidity-detector-foundry-x86_64-apple-darwin.tar.gz"
/// hash = "a80a0dd7425173064ce6d1a4ba04b18a967484d6f0d19080170843229065c006"
///
/// [artifacts.aarch64-apple-darwin]
/// url = "..."
/// hash = "..."
///
/// [artifacts.x86_64-unknown-linux-gnu]
/// url = "..."
/// hash = "..."
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseManifest {
    /// The version of the release.
    pub version: String,

    /// A mapping of target platforms to their corresponding artifacts.
    pub artifacts: HashMap<String, Artifact>,
}

impl ReleaseManifest {
    /// Creates a new `ReleaseManifest` with the given version and artifacts.
    pub fn new(version: String, artifacts: HashMap<String, Artifact>) -> Self {
        ReleaseManifest { version, artifacts }
    }

    /// Adds an artifact for a specific target platform.
    ///
    /// # Arguments
    /// * `target` - The target platform for which the artifact is being added.
    /// * `artifact` - The artifact to add.
    pub fn add_artifact(&mut self, target: String, artifact: Artifact) {
        self.artifacts.insert(target, artifact);
    }

    /// Retrieves the artifact for a specific target platform.
    ///
    /// # Arguments
    /// * `target` - The target platform for which to retrieve the artifact.
    ///
    /// # Returns
    /// An `Option` containing the `Artifact` if found, or `None` otherwise.
    pub fn get_artifact(&self, target: &str) -> Option<&Artifact> {
        self.artifacts.get(target)
    }

    /// Checks if the package supports a specific target platform.
    ///
    /// # Arguments
    /// * `target` - The target platform to check.
    ///
    /// # Returns
    /// `true` if the target is supported, `false` otherwise.
    pub fn supports_target(&self, target: &str) -> bool {
        self.artifacts.contains_key(target)
    }
}

/// `Artifact` contains the URL and hash for a specific artifact of a target platform.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Artifact {
    /// The URL to download the artifact from.
    pub url: String,

    /// The hash of the artifact file, used for integrity checking.
    pub hash: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_artifact_creation() {
        let artifact = Artifact {
            url: String::from("https://example.com/artifact"),
            hash: String::from("abc123"),
        };

        assert_eq!(artifact.url, "https://example.com/artifact");
        assert_eq!(artifact.hash, "abc123");
    }

    #[test]
    fn test_release_manifest_creation() {
        let artifacts = HashMap::new();
        let release_manifest = ReleaseManifest::new(String::from("v1.0.0"), artifacts);
        assert_eq!(release_manifest.version, "v1.0.0");
    }

    #[test]
    fn test_add_artifact() {
        let mut release_manifest = ReleaseManifest::new(String::from("v1.0.0"), HashMap::new());

        let artifact = Artifact {
            url: String::from("https://example.com/artifact"),
            hash: String::from("abc123"),
        };

        release_manifest.add_artifact(String::from("x86_64-unknown-linux-gnu"), artifact);
        assert!(release_manifest.artifacts.contains_key("x86_64-unknown-linux-gnu"));
    }

    #[test]
    fn test_get_artifact() {
        let mut release_manifest = ReleaseManifest::new(String::from("v1.0.0"), HashMap::new());

        let artifact = Artifact {
            url: String::from("https://example.com/artifact"),
            hash: String::from("abc123"),
        };

        release_manifest.add_artifact(String::from("x86_64-unknown-linux-gnu"), artifact);

        let retrieved_artifact = release_manifest.get_artifact("x86_64-unknown-linux-gnu");
        assert!(retrieved_artifact.is_some());
        assert_eq!(retrieved_artifact.unwrap().url, "https://example.com/artifact");
    }

    #[test]
    fn test_supports_target() {
        let mut release_manifest = ReleaseManifest::new(String::from("v1.0.0"), HashMap::new());

        let artifact = Artifact {
            url: String::from("https://example.com/artifact"),
            hash: String::from("abc123"),
        };

        release_manifest.add_artifact(String::from("x86_64-unknown-linux-gnu"), artifact);

        assert!(release_manifest.supports_target("x86_64-unknown-linux-gnu"));
        assert!(!release_manifest.supports_target("aarch64-unknown-linux-gnu"));
    }
}
