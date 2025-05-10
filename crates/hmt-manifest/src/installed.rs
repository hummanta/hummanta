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

use hmt_utils::bytes::FromSlice;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf, str::FromStr};

use crate::{ManifestError, ManifestFile};

/// Represents a single installed package entry with version and optional description.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    /// The version of the package.
    pub version: String,
    /// An optional description of the package.
    pub description: Option<String>,
    /// The file path where the package is located.
    pub path: PathBuf,
}

impl Entry {
    /// Create a new, empty Entry.
    pub fn new(version: String, description: Option<String>, path: PathBuf) -> Self {
        Self { version, description, path }
    }
}

/// Represents a package entry with associated domain, name, and metadata.
#[derive(Debug, Clone)]
pub struct PackageEntry {
    /// The name of the package.
    pub name: String,
    /// The metadata associated with the package entry.
    pub entry: Entry,
}

impl PackageEntry {
    /// Creates a new PackageEntry from the given name, and entry.
    pub fn new(name: String, entry: Entry) -> Self {
        Self { name, entry }
    }
}

impl From<(&String, &Entry)> for PackageEntry {
    fn from((name, entry): (&String, &Entry)) -> Self {
        Self::new(name.clone(), entry.clone())
    }
}

/// Maps a package name (e.g., "solidity-detector-foundry") to its metadata.
pub type PackageMap = HashMap<String, Entry>;

/// Maps category names (e.g., "detector", "compiler") to packages.
pub type CategoryMap = HashMap<String, PackageMap>;

/// Maps domain names (e.g., "solidity", "move") to category maps.
pub type DomainMap = HashMap<String, CategoryMap>;

/// Maps kind names (e.g., "toolchains", "targets") to domain maps.
pub type KindMap = HashMap<String, DomainMap>;

/// Represents the full set of installed toolchains and targets.
///
/// Example TOML:
/// ```toml
/// [toolchains.solidity.detector]
/// solidity-detector-foundry = { version = "v1.2.0", description = "Solidity detector for Foundry projects" }
///
/// [targets.evm.runtime]
/// evm-runtime = { version = "v0.3.1", description = "EVM runtime for aarch64-apple-darwin" }
/// ```

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstalledManifest(KindMap);

impl InstalledManifest {
    /// Create a new, empty InstalledManifest.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Get a reference to the inner map.
    pub fn as_map(&self) -> &KindMap {
        &self.0
    }

    /// Get a mutable reference to the inner map.
    pub fn as_map_mut(&mut self) -> &mut KindMap {
        &mut self.0
    }

    /// Insert a new entry
    pub fn insert(&mut self, kind: &str, domain: &str, cat: &str, pkg: &str, entry: Entry) {
        self.0
            .entry(kind.to_string())
            .or_default()
            .entry(domain.to_string())
            .or_default()
            .entry(cat.to_string())
            .or_default()
            .insert(pkg.to_string(), entry);
    }

    /// Remove a package entry
    pub fn remove(&mut self, kind: &str, domain: &str, cat: &str, pkg: &str) -> Option<Entry> {
        self.0.get_mut(kind)?.get_mut(domain)?.get_mut(cat)?.remove(pkg)
    }

    /// Check if a package exists
    pub fn contains(&self, kind: &str, domain: &str, cat: &str, pkg: &str) -> bool {
        self.0
            .get(kind)
            .and_then(|d| d.get(domain))
            .and_then(|t| t.get(cat))
            .is_some_and(|p| p.contains_key(pkg))
    }

    /// Get the entire domain map under a kind (e.g., "toolchains")
    pub fn get_domain(&self, kind: &str) -> Option<&DomainMap> {
        self.0.get(kind)
    }

    /// Get a category map under a specific kind and domain.
    ///  (e.g., "toolchains" -> "solidity")
    pub fn get_category(&self, kind: &str, domain: &str) -> Option<&CategoryMap> {
        self.0.get(kind)?.get(domain)
    }

    /// Get the package map under a specific kind, domain, and type
    /// (e.g., "toolchains" -> "solidity" -> "detector")
    pub fn get_package(&self, kind: &str, domain: &str, cat: &str) -> Option<&PackageMap> {
        self.0.get(kind)?.get(domain)?.get(cat)
    }

    /// Remove all packages under a specific kind and domain.
    pub fn remove_domain(&mut self, kind: &str, domain: &str) {
        if let Some(kind_map) = self.0.get_mut(kind) {
            kind_map.remove(domain);
        }
    }

    /// Get all package maps under the given kind and category across all domains.
    pub fn by_category(&self, kind: &str, category: &str) -> Vec<&PackageMap> {
        self.get_domain(kind)
            .iter()
            .flat_map(|domain_map| domain_map.values())
            .filter_map(|cat_map| cat_map.get(category))
            .collect()
    }
}

/// Implement load from file and save to file
impl ManifestFile for InstalledManifest {}

impl FromStr for InstalledManifest {
    type Err = ManifestError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s).map_err(ManifestError::from)
    }
}

impl FromSlice for InstalledManifest {
    type Err = ManifestError;

    fn from_slice(v: &[u8]) -> Result<Self, Self::Err> {
        let s = std::str::from_utf8(v)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        toml::from_str(s).map_err(ManifestError::from)
    }
}
