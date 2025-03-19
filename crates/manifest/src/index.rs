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
use std::{collections::HashMap, path::PathBuf};

/// `IndexManifest` is a struct used to represent an index manifest.
///
/// It uses a nested `HashMap` to map toolchain or target names to their corresponding file paths,
/// making it convenient to manage and load related configuration files.
///
/// example:
/// ```toml
/// [toolchains]
/// solidity = "solidity.toml"
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct IndexManifest(HashMap<String, HashMap<String, PathBuf>>);

impl IndexManifest {
    /// Creates a new, empty `IndexManifest`.
    pub fn new() -> Self {
        IndexManifest(HashMap::new())
    }

    /// Inserts a new entry.
    ///
    /// # Arguments
    /// * `category` - The category name (e.g., "toolchains" or "targets").
    /// * `name` - The name of the toolchain or target (e.g., "solidity" or "evm").
    /// * `path` - The path to the corresponding file.
    pub fn insert(&mut self, category: String, name: String, path: PathBuf) {
        self.0.entry(category).or_default().insert(name, path);
    }

    /// Retrieves the path for a given category and name.
    ///
    /// # Arguments
    /// * `category` - The category name.
    /// * `name` - The name of the toolchain or target.
    ///
    /// # Returns
    /// An `Option` containing the `PathBuf` if found, or `None` otherwise.
    pub fn get(&self, category: &str, name: &str) -> Option<&PathBuf> {
        self.0.get(category)?.get(name)
    }

    /// Removes an entry.
    ///
    /// # Arguments
    /// * `category` - The category name.
    /// * `name` - The name of the toolchain or target.
    ///
    /// # Returns
    /// An `Option` containing the removed `PathBuf` if it existed, or `None` otherwise.
    pub fn remove(&mut self, category: &str, name: &str) -> Option<PathBuf> {
        self.0.get_mut(category)?.remove(name)
    }

    /// Checks if the manifest contains a specific entry.
    ///
    /// # Arguments
    /// * `category` - The category name.
    /// * `name` - The name of the toolchain or target.
    ///
    /// # Returns
    /// `true` if the entry exists, `false` otherwise.
    pub fn contains(&self, category: &str, name: &str) -> bool {
        self.0.get(category).is_some_and(|map| map.contains_key(name))
    }
}

impl Default for IndexManifest {
    fn default() -> Self {
        IndexManifest::new()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let manifest = IndexManifest::new();
        assert!(manifest.0.is_empty());
    }

    #[test]
    fn test_insert_and_get() {
        let mut manifest = IndexManifest::new();
        let category = "toolchains".to_string();
        let name = "solidity".to_string();
        let path = PathBuf::from("solidity.toml");

        manifest.insert(category.clone(), name.clone(), path.clone());
        assert_eq!(manifest.get(&category, &name), Some(&path));
    }

    #[test]
    fn test_remove() {
        let mut manifest = IndexManifest::new();
        let category = "toolchains".to_string();
        let name = "solidity".to_string();
        let path = PathBuf::from("solidity.toml");

        manifest.insert(category.clone(), name.clone(), path.clone());
        let removed = manifest.remove(&category, &name);
        assert_eq!(removed, Some(path));
        assert!(manifest.get(&category, &name).is_none());
    }

    #[test]
    fn test_contains() {
        let mut manifest = IndexManifest::new();
        let category = "toolchains".to_string();
        let name = "solidity".to_string();
        let path = PathBuf::from("solidity.toml");

        manifest.insert(category.clone(), name.clone(), path);
        assert!(manifest.contains(&category, &name));
        assert!(!manifest.contains(&category, "nonexistent"));
    }

    #[test]
    fn test_empty_manifest() {
        let manifest = IndexManifest::new();
        assert!(!manifest.contains("toolchains", "solidity"));
        assert!(manifest.get("toolchains", "solidity").is_none());
    }

    #[test]
    fn test_remove_nonexistent_entry() {
        let mut manifest = IndexManifest::new();
        assert!(manifest.remove("toolchains", "solidity").is_none());
    }

    #[test]
    fn test_multiple_categories() {
        let mut manifest = IndexManifest::new();
        let category1 = "toolchains".to_string();
        let category2 = "targets".to_string();
        let name1 = "solidity".to_string();
        let name2 = "evm".to_string();
        let path1 = PathBuf::from("solidity.toml");
        let path2 = PathBuf::from("evm.toml");

        manifest.insert(category1.clone(), name1.clone(), path1.clone());
        manifest.insert(category2.clone(), name2.clone(), path2.clone());

        assert_eq!(manifest.get(&category1, &name1), Some(&path1));
        assert_eq!(manifest.get(&category2, &name2), Some(&path2));
        assert!(manifest.contains(&category1, &name1));
        assert!(manifest.contains(&category2, &name2));
    }
}
