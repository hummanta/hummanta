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

use std::{io::Read, path::Path, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::{ManifestError, ManifestResult};

/// `PackageManifest` keeps track of all versions of a component package.
///
/// This structure represents a manifest for a given package,
/// including metadata (e.g., name, language, kind) and release information.
///
/// Example:
/// ```toml
/// [package]
/// name = "solidity-detector-foundry"
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
    pub package: PackageMeta,

    /// Supported target platforms.
    pub targets: Vec<String>,

    /// The latest version of the package.
    pub latest: String,

    /// A list of all release manifest files.
    pub releases: Vec<String>,
}

impl PackageManifest {
    /// Create a new PackageManifest instance.
    pub fn new(package: PackageMeta, targets: Vec<String>, latest: String) -> Self {
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

impl PackageManifest
where
    Self: FromStr,
{
    /// Read the project manifest from a file.
    pub fn read<P: AsRef<Path>>(path: P) -> ManifestResult<Self> {
        let mut file = std::fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Self::from_str(&contents)
    }

    /// Write the project manifest to a file.
    pub fn write<P: AsRef<Path>>(&self, path: P) -> ManifestResult<()> {
        let toml_string = toml::to_string_pretty(&self)?;
        std::fs::write(path, toml_string)?;

        Ok(())
    }
}

impl FromStr for PackageManifest {
    type Err = ManifestError;

    fn from_str(s: &str) -> ManifestResult<Self> {
        toml::from_str(s).map_err(ManifestError::from)
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

/// Convert PackageConfig to PackageMeta.
impl From<&PackageConfig> for PackageMeta {
    fn from(config: &PackageConfig) -> Self {
        PackageMeta {
            name: config.name.clone(),
            language: config.language.clone(),
            kind: config.kind.clone(),
            description: Some(config.description.clone()),
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
    /// The name of the component package (also used as the artifact prefix).
    pub name: String,

    /// The GitHub repository URL (used to construct release download links).
    pub repository: String,

    /// The programming language supported by this component (e.g., "solidity", "rust").
    pub language: String,

    /// The kind of component (e.g., "detector", "compiler").
    pub kind: String,

    /// A short description of the component package.
    pub description: String,

    /// A list of supported platform targets (e.g., "x86_64-apple-darwin").
    pub targets: Vec<String>,
}

impl PackageConfig
where
    Self: FromStr,
{
    /// Read the project manifest from a file.
    pub fn read<P: AsRef<Path>>(path: P) -> ManifestResult<Self> {
        let mut file = std::fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Self::from_str(&contents)
    }

    /// Write the project manifest to a file.
    pub fn write<P: AsRef<Path>>(&self, path: P) -> ManifestResult<()> {
        let toml_string = toml::to_string_pretty(&self)?;
        std::fs::write(path, toml_string)?;

        Ok(())
    }
}

impl FromStr for PackageConfig {
    type Err = ManifestError;

    fn from_str(s: &str) -> ManifestResult<Self> {
        toml::from_str(s).map_err(ManifestError::from)
    }
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
    fn test_package_manifest_creation() {
        let meta = create_test_package_meta();
        let targets =
            vec![String::from("x86_64-apple-darwin"), String::from("aarch64-apple-darwin")];
        let package_manifest = PackageManifest::new(meta, targets.clone(), String::from("v1.0.0"));

        assert_eq!(package_manifest.latest, "v1.0.0");
        assert_eq!(package_manifest.targets, targets);
        assert!(package_manifest.releases.is_empty());
    }

    #[test]
    fn test_add_release() {
        let meta = create_test_package_meta();
        let mut package_manifest = PackageManifest::new(
            meta,
            vec![String::from("x86_64-apple-darwin")],
            String::from("v1.0.0"),
        );

        package_manifest.add_release(String::from("release-v1.1.0.toml"));

        assert_eq!(package_manifest.releases.len(), 1);
        assert_eq!(package_manifest.releases[0], "release-v1.1.0.toml");
    }

    #[test]
    fn test_get_releases() {
        let meta = create_test_package_meta();
        let mut package_manifest = PackageManifest::new(
            meta,
            vec![String::from("x86_64-apple-darwin")],
            String::from("v1.0.0"),
        );

        package_manifest.add_release(String::from("release-v1.1.0.toml"));
        package_manifest.add_release(String::from("release-v1.2.0.toml"));

        let releases = package_manifest.get_releases();
        assert_eq!(releases.len(), 2);
        assert_eq!(releases[0], "release-v1.1.0.toml");
        assert_eq!(releases[1], "release-v1.2.0.toml");
    }
}
