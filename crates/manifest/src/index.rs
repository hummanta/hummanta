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
use std::{
    collections::HashMap,
    io::Read,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{error::Result, ManifestError};

/// `IndexManifest` is a struct used to represent an index manifest.
///
/// example:
/// ```toml
/// solidity = "solidity.toml"
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct IndexManifest(HashMap<String, PathBuf>);

impl IndexManifest {
    /// Creates a new, empty `IndexManifest`.
    pub fn new() -> Self {
        IndexManifest(HashMap::new())
    }

    /// Inserts a new entry.
    ///
    /// # Arguments
    /// * `name` - The name of the toolchain or target (e.g., "solidity").
    /// * `path` - The path to the corresponding file.
    pub fn insert(&mut self, name: String, path: PathBuf) {
        self.0.insert(name, path);
    }

    /// Retrieves the path for a given name.
    ///
    /// # Arguments
    /// * `name` - The name of the toolchain or target.
    ///
    /// # Returns
    /// An `Option` containing the `PathBuf` if found, or `None` otherwise.
    pub fn get(&self, name: &str) -> Option<&PathBuf> {
        self.0.get(name)
    }

    /// Removes an entry.
    ///
    /// # Arguments
    /// * `name` - The name of the toolchain or target.
    ///
    /// # Returns
    /// An `Option` containing the removed `PathBuf` if it existed, or `None` otherwise.
    pub fn remove(&mut self, name: &str) -> Option<PathBuf> {
        self.0.remove(name)
    }

    /// Checks if the manifest contains a specific entry.
    ///
    /// # Arguments
    /// * `name` - The name of the toolchain or target.
    ///
    /// # Returns
    /// `true` if the entry exists, `false` otherwise.
    pub fn contains(&self, name: &str) -> bool {
        self.0.contains_key(name)
    }

    /// Returns an iterator over the manifest.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &PathBuf)> {
        self.0.iter()
    }
}

impl Default for IndexManifest {
    fn default() -> Self {
        IndexManifest::new()
    }
}

impl IndexManifest
where
    Self: FromStr,
{
    /// Load the index manifest from a file.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = std::fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Self::from_str(&contents)
    }
}

impl std::str::FromStr for IndexManifest {
    type Err = ManifestError;

    fn from_str(s: &str) -> Result<Self> {
        toml::from_str(s).map_err(ManifestError::from)
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
        let name = "solidity".to_string();
        let path = PathBuf::from("solidity.toml");

        manifest.insert(name.clone(), path.clone());
        assert_eq!(manifest.get(&name), Some(&path));
    }

    #[test]
    fn test_remove() {
        let mut manifest = IndexManifest::new();
        let name = "solidity".to_string();
        let path = PathBuf::from("solidity.toml");

        manifest.insert(name.clone(), path.clone());
        let removed = manifest.remove(&name);
        assert_eq!(removed, Some(path));
        assert!(manifest.get(&name).is_none());
    }

    #[test]
    fn test_contains() {
        let mut manifest = IndexManifest::new();
        let name = "solidity".to_string();
        let path = PathBuf::from("solidity.toml");

        manifest.insert(name.clone(), path);
        assert!(manifest.contains(&name));
        assert!(!manifest.contains("nonexistent"));
    }

    #[test]
    fn test_empty_manifest() {
        let manifest = IndexManifest::new();
        assert!(!manifest.contains("solidity"));
        assert!(manifest.get("solidity").is_none());
    }

    #[test]
    fn test_remove_nonexistent_entry() {
        let mut manifest = IndexManifest::new();
        assert!(manifest.remove("solidity").is_none());
    }

    #[test]
    fn test_multiple_entries() {
        let mut manifest = IndexManifest::new();
        let name1 = "solidity".to_string();
        let name2 = "rust".to_string();
        let path1 = PathBuf::from("solidity.toml");
        let path2 = PathBuf::from("rust.toml");

        manifest.insert(name1.clone(), path1.clone());
        manifest.insert(name2.clone(), path2.clone());

        assert_eq!(manifest.get(&name1), Some(&path1));
        assert_eq!(manifest.get(&name2), Some(&path2));
        assert!(manifest.contains(&name1));
        assert!(manifest.contains(&name2));
    }
}
