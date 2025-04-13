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

/// `PackageManifest` describes a specific released version of a package.
///
/// This structure holds detailed information about a released version of the package,
/// including version-specific metadata, target platforms, and artifact details.
///
/// Example:
/// ```toml
/// [package]
/// name = "solidity-detector-foundry"
/// version = "1.2.0"
/// description = "Solidity detector for Foundry projects"
/// language = "solidity"
/// kind = "detector"
///
/// targets = [
///   "x86_64-apple-darwin",
///   "aarch64-apple-darwin",
///   "x86_64-unknown-linux-gnu"
/// ]
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
pub struct PackageManifest {
    /// General information about the package.
    pub package: PackageInfo,

    /// The list of target platforms for which the package is built.
    pub targets: Vec<String>,

    /// A mapping of target platforms to their corresponding artifacts.
    pub artifacts: HashMap<String, Artifact>,
}

impl PackageManifest {
    /// Creates a new `PackageManifest` with the given package info, targets, and artifacts.
    pub fn new(
        package: PackageInfo,
        targets: Vec<String>,
        artifacts: HashMap<String, Artifact>,
    ) -> Self {
        PackageManifest { package, targets, artifacts }
    }

    /// Adds a target platform to the package.
    ///
    /// # Arguments
    /// * `target` - The target platform to add.
    pub fn add_target(&mut self, target: String) {
        self.targets.push(target);
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
        self.targets.contains(&target.to_string())
    }
}

/// `PackageInfo` contains detailed information about the package itself.
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageInfo {
    /// The name of the package.
    pub name: String,

    /// The version of the package.
    pub version: String,

    /// An optional description of the package.
    pub description: Option<String>,

    /// The programming language used for the package.
    pub language: String,

    /// The kind of the package (e.g., "detector", "compiler").
    pub kind: String,
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

    fn create_test_package_meta() -> PackageMeta {
        PackageMeta {
            name: String::from("test-package"),
            language: String::from("Rust"),
            kind: String::from("detector"),
            description: Some(String::from("A test package")),
        }
    }

    fn create_test_package_info() -> PackageInfo {
        PackageInfo {
            name: String::from("test-package"),
            version: String::from("1.0.0"),
            description: Some(String::from("A test package")),
            language: String::from("Rust"),
            kind: String::from("detector"),
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

    #[test]
    fn test_package_info_creation() {
        let package_info = create_test_package_info();

        assert_eq!(package_info.name, "test-package");
        assert_eq!(package_info.version, "1.0.0");
        assert_eq!(package_info.description, Some(String::from("A test package")));
        assert_eq!(package_info.language, "Rust");
        assert_eq!(package_info.kind, "detector");
    }

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
    fn test_package_manifest_creation() {
        let package_info = create_test_package_info();
        let targets = vec![String::from("x86_64-unknown-linux-gnu")];
        let artifacts = HashMap::new();
        let package_manifest = PackageManifest::new(package_info, targets.clone(), artifacts);
        assert_eq!(package_manifest.package.name, "test-package");
        assert_eq!(package_manifest.targets, targets);
    }

    #[test]
    fn test_add_target() {
        let mut package_manifest =
            PackageManifest::new(create_test_package_info(), vec![], HashMap::new());

        package_manifest.add_target(String::from("x86_64-unknown-linux-gnu"));
        assert!(package_manifest.targets.contains(&String::from("x86_64-unknown-linux-gnu")));
    }

    #[test]
    fn test_add_artifact() {
        let mut package_manifest =
            PackageManifest::new(create_test_package_info(), vec![], HashMap::new());

        let artifact = Artifact {
            url: String::from("https://example.com/artifact"),
            hash: String::from("abc123"),
        };

        package_manifest.add_artifact(String::from("x86_64-unknown-linux-gnu"), artifact);
        assert!(package_manifest.artifacts.contains_key("x86_64-unknown-linux-gnu"));
    }

    #[test]
    fn test_get_artifact() {
        let mut package_manifest =
            PackageManifest::new(create_test_package_info(), vec![], HashMap::new());

        let artifact = Artifact {
            url: String::from("https://example.com/artifact"),
            hash: String::from("abc123"),
        };

        package_manifest.add_artifact(String::from("x86_64-unknown-linux-gnu"), artifact);

        let retrieved_artifact = package_manifest.get_artifact("x86_64-unknown-linux-gnu");
        assert!(retrieved_artifact.is_some());
        assert_eq!(retrieved_artifact.unwrap().url, "https://example.com/artifact");
    }

    #[test]
    fn test_supports_target() {
        let package_manifest = PackageManifest::new(
            create_test_package_info(),
            vec![String::from("x86_64-unknown-linux-gnu")],
            HashMap::new(),
        );

        assert!(package_manifest.supports_target("x86_64-unknown-linux-gnu"));
        assert!(!package_manifest.supports_target("aarch64-unknown-linux-gnu"));
    }
}
