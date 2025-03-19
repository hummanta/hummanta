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

/// `ToolchainManifest` represents the structure of a toolchain manifest file.
#[derive(Debug, Serialize, Deserialize)]
pub struct ToolchainManifest {
    /// Used to specify tools for detecting specific configurations.
    #[serde(default)]
    pub detector: Vec<Toolchain>,
    /// Used to specify compiler tools.
    #[serde(default)]
    pub compiler: Vec<Toolchain>,
}

impl ToolchainManifest {
    /// Creates a new, empty `ToolchainManifest`.
    pub fn new() -> Self {
        ToolchainManifest { detector: Vec::new(), compiler: Vec::new() }
    }

    /// Adds a toolchain to the detector list.
    ///
    /// # Arguments
    /// * `toolchain` - The toolchain to add.
    pub fn add_detector(&mut self, toolchain: Toolchain) {
        self.detector.push(toolchain);
    }

    /// Adds a toolchain to the compiler list.
    ///
    /// # Arguments
    /// * `toolchain` - The toolchain to add.
    pub fn add_compiler(&mut self, toolchain: Toolchain) {
        self.compiler.push(toolchain);
    }

    /// Retrieves all detector toolchains.
    ///
    /// # Returns
    /// A reference to the list of detector toolchains.
    pub fn get_detectors(&self) -> &Vec<Toolchain> {
        &self.detector
    }

    /// Retrieves all compiler toolchains.
    ///
    /// # Returns
    /// A reference to the list of compiler toolchains.
    pub fn get_compilers(&self) -> &Vec<Toolchain> {
        &self.compiler
    }

    /// Checks if a specific toolchain exists in the detector list.
    ///
    /// # Arguments
    /// * `name` - The name of the toolchain to check.
    ///
    /// # Returns
    /// `true` if the toolchain exists, `false` otherwise.
    pub fn contains_detector(&self, name: &str) -> bool {
        self.detector.iter().any(|t| match t {
            Toolchain::Package(pkg) => pkg.name == name,
            Toolchain::Release(rel) => rel.name == name,
        })
    }

    /// Checks if a specific toolchain exists in the compiler list.
    ///
    /// # Arguments
    /// * `name` - The name of the toolchain to check.
    ///
    /// # Returns
    /// `true` if the toolchain exists, `false` otherwise.
    pub fn contains_compiler(&self, name: &str) -> bool {
        self.compiler.iter().any(|t| match t {
            Toolchain::Package(pkg) => pkg.name == name,
            Toolchain::Release(rel) => rel.name == name,
        })
    }
}

impl Default for ToolchainManifest {
    fn default() -> Self {
        ToolchainManifest::new()
    }
}

/// `Toolchain` is an enum that represents a toolchain configuration.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Toolchain {
    Package(PackageToolchain),
    Release(ReleaseToolchain),
}

impl Toolchain {
    /// Retrieves the name of the toolchain.
    pub fn name(&self) -> &str {
        match self {
            Toolchain::Package(pkg) => &pkg.name,
            Toolchain::Release(rel) => &rel.name,
        }
    }
}

/// `PackageToolchain` represents a toolchain defined by a package.
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageToolchain {
    /// The name of the toolchain.
    pub name: String,
    /// The package name associated with the toolchain.
    pub package: String,
    /// An optional field specifying the binary name of the toolchain.
    pub bin: Option<String>,
}

impl PackageToolchain {
    /// Creates a new `PackageToolchain`.
    pub fn new(name: String, package: String, bin: Option<String>) -> Self {
        PackageToolchain { name, package, bin }
    }
}

/// `ReleaseToolchain` represents a toolchain defined by a specific release version.
#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseToolchain {
    /// The name of the toolchain.
    pub name: String,
    /// The version of the toolchain.
    pub version: String,
    /// A map of target platforms to their respective `TargetInfo`.
    pub target: HashMap<String, TargetInfo>,
}

impl ReleaseToolchain {
    /// Creates a new `ReleaseToolchain`.
    pub fn new(name: String, version: String, target: HashMap<String, TargetInfo>) -> Self {
        ReleaseToolchain { name, version, target }
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
        assert!(manifest.detector.is_empty());
        assert!(manifest.compiler.is_empty());
    }

    #[test]
    fn test_add_detector() {
        let mut manifest = ToolchainManifest::new();
        let toolchain = Toolchain::Package(PackageToolchain::new(
            "detector1".to_string(),
            "package1".to_string(),
            Some("bin1".to_string()),
        ));
        manifest.add_detector(toolchain);
        assert_eq!(manifest.detector.len(), 1);
        assert_eq!(manifest.detector[0].name(), "detector1");
    }

    #[test]
    fn test_add_compiler() {
        let mut manifest = ToolchainManifest::new();
        let toolchain = Toolchain::Release(ReleaseToolchain::new(
            "compiler1".to_string(),
            "1.0.0".to_string(),
            HashMap::new(),
        ));
        manifest.add_compiler(toolchain);
        assert_eq!(manifest.compiler.len(), 1);
        assert_eq!(manifest.compiler[0].name(), "compiler1");
    }

    #[test]
    fn test_contains_detector() {
        let mut manifest = ToolchainManifest::new();
        let toolchain = Toolchain::Package(PackageToolchain::new(
            "detector1".to_string(),
            "package1".to_string(),
            None,
        ));
        manifest.add_detector(toolchain);
        assert!(manifest.contains_detector("detector1"));
        assert!(!manifest.contains_detector("nonexistent"));
    }

    #[test]
    fn test_contains_compiler() {
        let mut manifest = ToolchainManifest::new();
        let toolchain = Toolchain::Release(ReleaseToolchain::new(
            "compiler1".to_string(),
            "1.0.0".to_string(),
            HashMap::new(),
        ));
        manifest.add_compiler(toolchain);
        assert!(manifest.contains_compiler("compiler1"));
        assert!(!manifest.contains_compiler("nonexistent"));
    }

    #[test]
    fn test_package_toolchain_new() {
        let toolchain = PackageToolchain::new(
            "toolchain1".to_string(),
            "package1".to_string(),
            Some("bin1".to_string()),
        );
        assert_eq!(toolchain.name, "toolchain1");
        assert_eq!(toolchain.package, "package1");
        assert_eq!(toolchain.bin, Some("bin1".to_string()));
    }

    #[test]
    fn test_release_toolchain_new() {
        let targets = HashMap::new();
        let toolchain =
            ReleaseToolchain::new("toolchain1".to_string(), "1.0.0".to_string(), targets.clone());
        assert_eq!(toolchain.name, "toolchain1");
        assert_eq!(toolchain.version, "1.0.0");
        assert_eq!(toolchain.target, targets);
    }

    #[test]
    fn test_release_toolchain_get_target_info() {
        let mut targets = HashMap::new();
        targets.insert(
            "x86_64".to_string(),
            TargetInfo::new("http://example.com".to_string(), "hash123".to_string()),
        );
        let toolchain =
            ReleaseToolchain::new("toolchain1".to_string(), "1.0.0".to_string(), targets.clone());
        let target_info = toolchain.get_target_info("x86_64");
        assert!(target_info.is_some());
        assert_eq!(target_info.unwrap().url, "http://example.com");
        assert_eq!(target_info.unwrap().hash, "hash123");
    }

    #[test]
    fn test_target_info_new() {
        let target_info = TargetInfo::new("http://example.com".to_string(), "hash123".to_string());
        assert_eq!(target_info.url, "http://example.com");
        assert_eq!(target_info.hash, "hash123");
    }
}
