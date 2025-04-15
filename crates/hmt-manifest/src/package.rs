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

use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{ManifestError, ManifestFile, ManifestResult};

/// `PackageManifest` keeps track of all versions of a component package.
///
/// This structure represents a manifest for a given package,
/// including metadata (e.g., name, language, kind) and release information.
///
/// Example:
/// ```toml
/// [package]
/// name = "solidity-detector-foundry"
/// repository = "https://github.com/hummanta/solidity-detector-foundry"
/// language = "solidity"
/// kind = "detector"
/// description = "Solidity detector for Foundry projects"
///
/// targets = [
///   "x86_64-apple-darwin",
///   "aarch64-apple-darwin",
///   "x86_64-unknown-linux-gnu"
/// ]
///
/// latest = "v1.2.0"
///
/// releases = [
///     "release-v1.2.0.toml",
///     "release-v1.1.0.toml"
/// ]
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageManifest {
    /// Metadata for the package, such as name, language, and kind.
    pub package: Package,

    /// Supported target platforms.
    pub targets: Vec<String>,

    /// The latest version of the package.
    pub latest: String,

    /// A list of all release manifest files.
    pub releases: Vec<String>,
}

impl PackageManifest {
    /// Create a new PackageManifest instance.
    pub fn new(package: Package, targets: Vec<String>, latest: String) -> Self {
        PackageManifest { package, targets, latest, releases: Vec::new() }
    }

    /// Add a release to the PackageManifest.
    ///
    /// # Arguments
    /// * `release` - The release manifest file name.
    pub fn add_release(&mut self, release: String) {
        self.releases.push(release);
    }

    /// Get all releases.
    ///
    /// # Returns
    /// &Vec<String> - A reference to the list of all releases.
    pub fn get_releases(&self) -> &Vec<String> {
        &self.releases
    }
}

/// Implement load from file and save to file
impl ManifestFile for PackageManifest {}

impl FromStr for PackageManifest {
    type Err = ManifestError;

    fn from_str(s: &str) -> ManifestResult<Self> {
        toml::from_str(s).map_err(ManifestError::from)
    }
}

/// `Package` contains general metadata for a package.
#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    /// The name of the package.
    pub name: String,

    /// The GitHub repository URL.
    pub repository: String,

    /// The programming language used for the package.
    pub language: String,

    /// The kind of the package (e.g., "detector", "compiler").
    pub kind: String,

    /// A description of the package (optional).
    pub description: Option<String>,
}

/// Convert PackageConfig to Meta.
impl From<&PackageConfig> for Package {
    fn from(config: &PackageConfig) -> Self {
        Package {
            name: config.package.name.clone(),
            repository: config.package.repository.clone(),
            language: config.package.language.clone(),
            kind: config.package.kind.clone(),
            description: config.package.description.clone(),
        }
    }
}

/// `PackageConfig` represents the metadata defined in `hmt-package.toml`.
///
/// It serves as the source configuration for generating manifest files,
/// containing essential information about the component package.
///
/// Example:
/// ```toml
/// [package]
/// name = "solidity-detector-foundry"
/// repository = "https://github.com/hummanta/solidity-detector-foundry"
/// language = "solidity"
/// kind = "detector"
/// description = "Solidity detector for Foundry projects"
///
/// targets = [
///   "x86_64-apple-darwin",
///   "aarch64-apple-darwin",
///   "x86_64-unknown-linux-gnu"
/// ]
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageConfig {
    /// Metadata for the package, such as name, language, and kind.
    pub package: Package,

    /// A list of supported platform targets (e.g., "x86_64-apple-darwin").
    pub targets: Vec<String>,
}

/// Implement load from file and save to file
impl ManifestFile for PackageConfig {}

impl FromStr for PackageConfig {
    type Err = ManifestError;

    fn from_str(s: &str) -> ManifestResult<Self> {
        toml::from_str(s).map_err(ManifestError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_package() -> Package {
        Package {
            name: String::from("test-package"),
            repository: String::from("https://github.com/hummanta/solidity-detector-foundry"),
            language: String::from("Rust"),
            kind: String::from("detector"),
            description: Some(String::from("A test package")),
        }
    }

    #[test]
    fn test_package_manifest_creation() {
        let package = create_test_package();
        let targets =
            vec![String::from("x86_64-apple-darwin"), String::from("aarch64-apple-darwin")];
        let manifest = PackageManifest::new(package, targets.clone(), String::from("v1.0.0"));

        assert_eq!(manifest.latest, "v1.0.0");
        assert_eq!(manifest.targets, targets);
        assert!(manifest.releases.is_empty());
    }

    #[test]
    fn test_add_release() {
        let package = create_test_package();
        let mut manifest = PackageManifest::new(
            package,
            vec![String::from("x86_64-apple-darwin")],
            String::from("v1.0.0"),
        );

        manifest.add_release(String::from("release-v1.1.0.toml"));

        assert_eq!(manifest.releases.len(), 1);
        assert_eq!(manifest.releases[0], "release-v1.1.0.toml");
    }

    #[test]
    fn test_get_releases() {
        let package = create_test_package();
        let mut manifest = PackageManifest::new(
            package,
            vec![String::from("x86_64-apple-darwin")],
            String::from("v1.0.0"),
        );

        manifest.add_release(String::from("release-v1.1.0.toml"));
        manifest.add_release(String::from("release-v1.2.0.toml"));

        let releases = manifest.get_releases();
        assert_eq!(releases.len(), 2);
        assert_eq!(releases[0], "release-v1.1.0.toml");
        assert_eq!(releases[1], "release-v1.2.0.toml");
    }
}
