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
use std::collections::HashMap;

/// `PackageIndex` keeps track of all versions of a component package.
///
/// This structure represents an index of all versions for a given package,
/// including metadata (e.g., name, language, kind) and version information.
///
/// Example:
/// ```toml
/// [meta]
/// name = "solidity-detector-foundry"
/// language = "solidity"
/// kind = "detector"
/// description = "Solidity detector for Foundry projects"
///
/// latest = "1.2.0"
///
/// [versions."1.2.0"]
/// manifest = "https://github.com/hummanta/solidity-detector-foundry/releases/download/v1.2.0/manifest.toml"
///
/// [versions."1.1.0"]
/// manifest = "https://github.com/hummanta/solidity-detector-foundry/releases/download/v1.1.0/manifest.toml"
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageIndex {
    /// Metadata for the package, such as name, language, and kind.
    pub meta: PackageMeta,

    /// The latest version of the package.
    pub latest: String,

    /// A mapping of all versions to their manifest URLs.
    pub versions: HashMap<String, VersionInfo>,
}

impl PackageIndex {
    /// Create a new PackageIndex instance.
    pub fn new(meta: PackageMeta, latest: String) -> Self {
        PackageIndex { meta, latest, versions: HashMap::new() }
    }

    /// Add a version information to the PackageIndex.
    ///
    /// # Arguments
    /// * `version` - The version number.
    /// * `manifest_url` - The manifest file URL for this version.
    pub fn add_version(&mut self, version: String, manifest_url: String) {
        let version_info = VersionInfo { manifest: manifest_url };
        self.versions.insert(version.clone(), version_info);
        self.latest = version; // Update the latest version
    }

    /// Get information for a specified version.
    ///
    /// # Arguments
    /// * `version` - The version number.
    ///
    /// # Returns
    /// `Option<&VersionInfo>` - Returns the corresponding VersionInfo if the
    /// version exists, otherwise returns None.
    pub fn get_version(&self, version: &str) -> Option<&VersionInfo> {
        self.versions.get(version)
    }

    /// List all versions.
    ///
    /// # Returns
    /// Vec<String> - A list of all versions.
    pub fn list_versions(&self) -> Vec<String> {
        self.versions.keys().cloned().collect()
    }
}

/// `PackageMeta` contains general metadata for a package.
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageMeta {
    /// The name of the package.
    pub name: String,

    /// The programming language used for the package.
    pub language: String,

    /// The kind of the package (e.g., "detector", "compiler").
    pub kind: String,

    /// A description of the package (optional).
    pub description: Option<String>,
}

/// `VersionInfo` contains information about a specific version of the package.
#[derive(Debug, Serialize, Deserialize)]
pub struct VersionInfo {
    /// The URL to the manifest file for the specific version.
    pub manifest: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_package_meta() -> PackageMeta {
        PackageMeta {
            name: String::from("test-package"),
            language: String::from("Rust"),
            kind: String::from("detector"),
            description: Some(String::from("A test package")),
        }
    }

    #[test]
    fn test_package_index_creation() {
        let meta = create_test_package_meta();
        let package_index = PackageIndex::new(meta, String::from("1.0.0"));

        assert_eq!(package_index.latest, "1.0.0");
        assert!(package_index.versions.is_empty());
    }

    #[test]
    fn test_add_version() {
        let meta = create_test_package_meta();
        let mut package_index = PackageIndex::new(meta, String::from("1.0.0"));

        package_index.add_version(
            String::from("1.1.0"),
            String::from("https://example.com/1.1.0/manifest.toml"),
        );

        assert_eq!(package_index.latest, "1.1.0");
        assert!(package_index.versions.contains_key("1.1.0"));
    }

    #[test]
    fn test_get_version() {
        let meta = create_test_package_meta();
        let mut package_index = PackageIndex::new(meta, String::from("1.0.0"));

        package_index.add_version(
            String::from("1.1.0"),
            String::from("https://example.com/1.1.0/manifest.toml"),
        );

        let version_info = package_index.get_version("1.1.0");
        assert!(version_info.is_some());
        assert_eq!(version_info.unwrap().manifest, "https://example.com/1.1.0/manifest.toml");
    }

    #[test]
    fn test_get_nonexistent_version() {
        let meta = create_test_package_meta();
        let package_index = PackageIndex::new(meta, String::from("1.0.0"));

        let version_info = package_index.get_version("1.1.0");
        assert!(version_info.is_none());
    }
}
