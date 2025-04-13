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

/// `ToolchainManifest` maps kind -> name -> PackageIndexRef.
///
/// This structure represents a mapping from toolchain categories (`kind`)
/// to toolchain names (`name`) and their corresponding `PackageIndexRef`.
///
/// Example:
/// ```toml
/// [detector.solidity-detector-foundry]
///     package-index = "https://hummanta.github.io/solidity-detector-foundry/package-index.toml"
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct ToolchainManifest(HashMap<String, HashMap<String, PackageIndexRef>>);

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
    /// * `index_ref` - The `PackageIndexRef` to insert.
    pub fn insert(&mut self, category: String, name: String, index_ref: PackageIndexRef) {
        self.0.entry(category).or_default().insert(name, index_ref);
    }

    /// Retrieves a toolchain for a given category and name.
    ///
    /// # Arguments
    /// * `category` - The category name.
    /// * `name` - The name of the toolchain.
    ///
    /// # Returns
    /// An `Option` containing the `PackageIndexRef` if found, or `None` otherwise.
    pub fn get(&self, category: &str, name: &str) -> Option<&PackageIndexRef> {
        self.0.get(category)?.get(name)
    }

    /// Retrieves all toolchains for a given category.
    ///
    /// # Arguments
    /// * `category` - The category name.
    ///
    /// # Returns
    /// An `Option` containing a reference to the map of toolchains for the specified category,
    /// or `None` if the category does not exist.
    pub fn by_category(&self, category: &str) -> Option<&HashMap<String, PackageIndexRef>> {
        self.0.get(category)
    }

    /// Removes a toolchain entry.
    ///
    /// # Arguments
    /// * `category` - The category name.
    /// * `name` - The name of the toolchain.
    ///
    /// # Returns
    /// An `Option` containing the removed `PackageIndexRef` if it existed, or `None` otherwise.
    pub fn remove(&mut self, category: &str, name: &str) -> Option<PackageIndexRef> {
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

    /// Returns an iterator over the entries in the toolchain manifest.
    ///
    /// This iterator yields tuples where the first element is the category name
    /// and the second element is a reference to the corresponding map of toolchains.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &HashMap<String, PackageIndexRef>)> {
        self.0.iter()
    }

    /// Returns an iterator over the toolchains in the manifest.
    ///
    /// This iterator yields references to the maps of toolchains, where each map
    /// corresponds to a category of toolchains.
    pub fn values(&self) -> impl Iterator<Item = &HashMap<String, PackageIndexRef>> {
        self.0.values()
    }
}

impl Default for ToolchainManifest {
    fn default() -> Self {
        ToolchainManifest::new()
    }
}

/// `PackageIndexRef` holds a reference to a package index file.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PackageIndexRef {
    /// URL to the package index.
    #[serde(rename = "package-index")]
    pub package_index: String,
}

impl PackageIndexRef {
    /// Creates a new `PackageIndexRef`.
    pub fn new(package_index: String) -> Self {
        PackageIndexRef { package_index }
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
        let index_ref = PackageIndexRef::new("https://example.com/package-index.toml".to_string());

        manifest.insert("detector".to_string(), "detector1".to_string(), index_ref);
        let retrieved = manifest.get("detector", "detector1");

        assert!(retrieved.is_some());
    }

    #[test]
    fn test_toolchain_manifest_remove() {
        let mut manifest = ToolchainManifest::new();
        let index_ref = PackageIndexRef::new("https://example.com/package-index.toml".to_string());

        manifest.insert("detector".to_string(), "detector1".to_string(), index_ref);
        let removed = manifest.remove("detector", "detector1");

        assert!(removed.is_some());
        assert!(manifest.get("detector", "detector1").is_none());
    }

    #[test]
    fn test_toolchain_manifest_contains() {
        let mut manifest = ToolchainManifest::new();
        let index_ref = PackageIndexRef::new("https://example.com/package-index.toml".to_string());

        manifest.insert("detector".to_string(), "detector1".to_string(), index_ref);
        assert!(manifest.contains("detector", "detector1"));
        assert!(!manifest.contains("detector", "nonexistent"));
    }

    #[test]
    fn test_index_ref_creation() {
        let index_ref = PackageIndexRef::new("https://example.com/package-index.toml".to_string());

        assert_eq!(index_ref.package_index, "https://example.com/package-index.toml");
    }

    #[test]
    fn test_iter_toolchain() {
        let mut manifest = ToolchainManifest::new();
        let index_ref = PackageIndexRef::new("https://example.com/package-index.toml".to_string());
        manifest.insert("detector".to_string(), "detector1".to_string(), index_ref);

        let mut iter = manifest.iter();
        assert!(iter.next().is_some());
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_values_toolchain() {
        let mut manifest = ToolchainManifest::new();
        let index_ref1 =
            PackageIndexRef::new("https://example.com/package-index1.toml".to_string());
        let index_ref2 =
            PackageIndexRef::new("https://example.com/package-index2.toml".to_string());

        manifest.insert("detector".to_string(), "detector1".to_string(), index_ref1);
        manifest.insert("compiler".to_string(), "compiler1".to_string(), index_ref2);

        let values: Vec<_> = manifest.values().collect();
        assert_eq!(values.len(), 2);

        assert!(values.iter().any(|map| map.contains_key("detector1")));
        assert!(values.iter().any(|map| map.contains_key("compiler1")));
    }
}
