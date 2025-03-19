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

/// `ToolchainManifest` represents the structure of a toolchain manifest file.
///
/// It is structured as a nested `HashMap`, where the outer map groups toolchains by category,
/// and the inner map associates toolchain names with their respective configurations.
///
/// example:
/// ```toml
/// [detector.detector1]
///     package = "package1"
///     bin = "bin1"
/// #
/// [compiler.compiler1]
///     version = "1.0.0"
/// #
// [compiler.compiler1.target.x86_64-unknown-linux-gnu]
///     url = "http://example.com"
///     hash = "hash123"
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct ToolchainManifest(HashMap<String, HashMap<String, Toolchain>>);

impl ToolchainManifest {
    /// Creates a new, empty `ToolchainManifest`.
    pub fn new() -> Self {
        ToolchainManifest(HashMap::new())
    }

    /// Inserts a new entry.
    ///
    /// # Arguments
    /// * `category` - The category name (e.g., "detector" or "compiler").
    /// * `name` - The name of the toolchain.
    /// * `toolchain` - The toolchain to insert.
    pub fn insert(&mut self, category: String, name: String, toolchain: Toolchain) {
        self.0.entry(category).or_default().insert(name, toolchain);
    }

    /// Retrieves a toolchain for a given category and name.
    ///
    /// # Arguments
    /// * `category` - The category name.
    /// * `name` - The name of the toolchain.
    ///
    /// # Returns
    /// An `Option` containing the `Toolchain` if found, or `None` otherwise.
    pub fn get(&self, category: &str, name: &str) -> Option<&Toolchain> {
        self.0.get(category)?.get(name)
    }

    /// Removes a toolchain entry.
    ///
    /// # Arguments
    /// * `category` - The category name.
    /// * `name` - The name of the toolchain.
    ///
    /// # Returns
    /// An `Option` containing the removed `Toolchain` if it existed, or `None` otherwise.
    pub fn remove(&mut self, category: &str, name: &str) -> Option<Toolchain> {
        self.0.get_mut(category)?.remove(name)
    }

    /// Checks if the manifest contains a specific toolchain entry.
    ///
    /// # Arguments
    /// * `category` - The category name.
    /// * `name` - The name of the toolchain.
    ///
    /// # Returns
    /// `true` if the entry exists, `false` otherwise.
    pub fn contains(&self, category: &str, name: &str) -> bool {
        self.0.get(category).is_some_and(|map| map.contains_key(name))
    }
}

impl Default for ToolchainManifest {
    fn default() -> Self {
        ToolchainManifest::new()
    }
}

/// `Toolchain` is an enum that represents a toolchain configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Toolchain {
    Package(PackageToolchain),
    Release(ReleaseToolchain),
}

/// `PackageToolchain` represents a toolchain defined by a package.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageToolchain {
    /// The package name associated with the toolchain.
    pub package: String,
    /// An optional field specifying the binary name of the toolchain.
    pub bin: Option<String>,
}

impl PackageToolchain {
    /// Creates a new `PackageToolchain`.
    pub fn new(package: String, bin: Option<String>) -> Self {
        PackageToolchain { package, bin }
    }
}

/// `ReleaseToolchain` represents a toolchain defined by a specific release version.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseToolchain {
    /// The version of the toolchain.
    pub version: String,
    /// A map of target platforms to their respective `TargetInfo`.
    pub target: HashMap<String, TargetInfo>,
}

impl ReleaseToolchain {
    /// Creates a new `ReleaseToolchain`.
    pub fn new(version: String, target: HashMap<String, TargetInfo>) -> Self {
        ReleaseToolchain { version, target }
    }

    /// Retrieves the `TargetInfo` for a specific target platform.
    pub fn get_target_info(&self, platform: &str) -> Option<&TargetInfo> {
        self.target.get(platform)
    }
}

/// `TargetInfo` represents the information for a specific target platform.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TargetInfo {
    /// The URL to download the toolchain for the target platform.
    pub url: String,
    /// The hash of the toolchain file for verification purposes.
    pub hash: String,
}

impl TargetInfo {
    /// Creates a new `TargetInfo`.
    pub fn new(url: String, hash: String) -> Self {
        Self { url, hash }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toolchain_manifest_new() {
        let manifest = ToolchainManifest::new();
        assert!(manifest.0.is_empty());
    }

    #[test]
    fn test_toolchain_manifest_insert_and_get() {
        let mut manifest = ToolchainManifest::new();
        let toolchain = Toolchain::Package(PackageToolchain::new(
            "package1".to_string(),
            Some("bin1".to_string()),
        ));

        manifest.insert("detector".to_string(), "detector1".to_string(), toolchain.clone());
        let retrieved = manifest.get("detector", "detector1");

        assert!(retrieved.is_some());
    }

    #[test]
    fn test_toolchain_manifest_remove() {
        let mut manifest = ToolchainManifest::new();
        let toolchain = Toolchain::Package(PackageToolchain::new(
            "package1".to_string(),
            Some("bin1".to_string()),
        ));

        manifest.insert("detector".to_string(), "detector1".to_string(), toolchain.clone());
        let removed = manifest.remove("detector", "detector1");

        assert!(removed.is_some());
        assert!(manifest.get("detector", "detector1").is_none());
    }

    #[test]
    fn test_toolchain_manifest_contains() {
        let mut manifest = ToolchainManifest::new();
        let toolchain = Toolchain::Package(PackageToolchain::new(
            "package1".to_string(),
            Some("bin1".to_string()),
        ));

        manifest.insert("detector".to_string(), "detector1".to_string(), toolchain);
        assert!(manifest.contains("detector", "detector1"));
        assert!(!manifest.contains("detector", "nonexistent"));
    }

    #[test]
    fn test_release_toolchain_get_target_info() {
        let mut target_info = HashMap::new();
        target_info.insert(
            "x86_64-unknown-linux-gnu".to_string(),
            TargetInfo::new("http://example.com".to_string(), "hash123".to_string()),
        );

        let release_toolchain = ReleaseToolchain::new("1.0.0".to_string(), target_info.clone());

        let target = release_toolchain.get_target_info("x86_64-unknown-linux-gnu");
        assert!(target.is_some());
        assert_eq!(target.unwrap(), target_info.get("x86_64-unknown-linux-gnu").unwrap());

        let nonexistent_target = release_toolchain.get_target_info("nonexistent");
        assert!(nonexistent_target.is_none());
    }

    #[test]
    fn test_package_toolchain_creation() {
        let package_toolchain =
            PackageToolchain::new("package1".to_string(), Some("bin1".to_string()));

        assert_eq!(package_toolchain.package, "package1");
        assert_eq!(package_toolchain.bin, Some("bin1".to_string()));
    }

    #[test]
    fn test_target_info_creation() {
        let target_info = TargetInfo::new("http://example.com".to_string(), "hash123".to_string());

        assert_eq!(target_info.url, "http://example.com");
        assert_eq!(target_info.hash, "hash123");
    }
}
