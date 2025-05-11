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

use std::{collections::HashMap, str::FromStr};

use hmt_utils::bytes::FromSlice;
use serde::{Deserialize, Serialize};

use crate::{ManifestError, ManifestFile};

/// `PackageManifest` keeps track of all versions of a component package.
///
/// This structure represents a manifest for a given package,
/// including metadata (e.g., name, language, kind) and release information.
///
/// Example:
/// ```toml
/// name = "solidity-detector-foundry"
/// homepage = "https://hummanta.github.io/solidity-detector-foundry"
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
/// [releases]
/// "v1.2.0" = "release-v1.2.0.toml"
/// "v1.1.0" = "release-v1.1.0.toml"
/// ```
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PackageManifest {
    /// Metadata for the package, such as name, language, and kind.
    #[serde(flatten)]
    pub package: Package,

    /// The latest version of the package.
    pub latest: String,

    /// A mapping of version to their corresponding release file.
    pub releases: HashMap<String, String>,
}

impl PackageManifest {
    /// Create a new PackageManifest instance.
    pub fn new(package: Package, latest: String) -> Self {
        PackageManifest { package, latest, releases: HashMap::new() }
    }

    /// Add a release to the PackageManifest.
    ///
    /// # Arguments
    /// * `version` - The version, eg. v1.0.0
    /// * `release` - The release manifest file name.
    pub fn add_release(&mut self, version: String, release: String) {
        self.releases.insert(version, release);
    }

    /// Get all releases.
    ///
    /// # Returns
    /// &HashMap<String, String> - A reference to the map of all releases.
    pub fn get_releases(&self) -> &HashMap<String, String> {
        &self.releases
    }
}

/// Implement load from file and save to file
impl ManifestFile for PackageManifest {}

impl FromStr for PackageManifest {
    type Err = ManifestError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s).map_err(ManifestError::from)
    }
}

impl FromSlice for PackageManifest {
    type Err = ManifestError;

    fn from_slice(v: &[u8]) -> Result<Self, Self::Err> {
        let s = std::str::from_utf8(v)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        toml::from_str(s).map_err(ManifestError::from)
    }
}

/// `Package` contains general metadata for a package.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Package {
    /// The name of the package.
    pub name: String,

    /// URL of the package homepage.
    pub homepage: String,

    /// The GitHub repository URL.
    pub repository: String,

    /// The programming language used for the package.
    /// Just used for detector and frontend.
    pub language: Option<String>,

    /// The kind of the package (e.g., "detector", "compiler").
    pub kind: String,

    /// A description of the package (optional).
    pub description: Option<String>,

    /// A list of supported platform targets (e.g., "x86_64-apple-darwin").
    pub targets: Vec<String>,
}

/// Implement load from file and save to file
impl ManifestFile for Package {}

impl FromStr for Package {
    type Err = ManifestError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s).map_err(ManifestError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_package() -> Package {
        Package {
            name: String::from("test-package"),
            homepage: String::from("https://hummanta.github.io/solidity-detector-foundry"),
            repository: String::from("https://github.com/hummanta/solidity-detector-foundry"),
            language: Some(String::from("Rust")),
            kind: String::from("detector"),
            description: Some(String::from("A test package")),
            targets: vec![
                String::from("x86_64-apple-darwin"),
                String::from("aarch64-apple-darwin"),
            ],
        }
    }

    #[test]
    fn test_package_manifest_creation() {
        let package = create_test_package();
        let manifest = PackageManifest::new(package, String::from("v1.0.0"));

        assert_eq!(manifest.latest, "v1.0.0");
        assert_eq!(
            manifest.package.targets,
            vec![String::from("x86_64-apple-darwin"), String::from("aarch64-apple-darwin"),]
        );
        assert!(manifest.releases.is_empty());
    }

    #[test]
    fn test_add_release() {
        let package = create_test_package();
        let mut manifest = PackageManifest::new(package, String::from("v1.0.0"));

        manifest.add_release(String::from("v1.0.0"), String::from("release-v1.1.0.toml"));
        assert_eq!(manifest.releases.len(), 1);
        assert_eq!(
            manifest.releases.iter().next(),
            Some((&String::from("v1.0.0"), &String::from("release-v1.1.0.toml")))
        );
    }

    #[test]
    fn test_get_releases() {
        let package = create_test_package();
        let mut manifest = PackageManifest::new(package, String::from("v1.2.0"));

        manifest.add_release(String::from("v1.1.0"), String::from("release-v1.1.0.toml"));
        manifest.add_release(String::from("v1.2.0"), String::from("release-v1.2.0.toml"));

        let releases = manifest.get_releases();
        assert_eq!(releases.len(), 2);
        assert_eq!(releases.get("v1.1.0"), Some(&String::from("release-v1.1.0.toml")));
        assert_eq!(releases.get("v1.2.0"), Some(&String::from("release-v1.2.0.toml")));
    }
}
