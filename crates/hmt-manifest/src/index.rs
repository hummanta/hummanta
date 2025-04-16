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
use std::{collections::HashMap, str::FromStr};

use crate::{ManifestError, ManifestFile};

/// `IndexManifest` is a struct used to represent an index manifest.
///
/// example:
/// ```toml
/// [toolchains]
/// move = "toolchains/move.toml"
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct IndexManifest(HashMap<String, HashMap<String, String>>);

impl IndexManifest {
    /// Creates a new, empty `IndexManifest`.
    pub fn new() -> Self {
        IndexManifest(HashMap::new())
    }

    /// Inserts a new entry.
    ///
    /// # Arguments
    /// * `section` - The section of the manifest.
    /// * `key` - The key within the section.
    /// * `value` - The value associated with the key.
    pub fn insert(&mut self, section: String, key: String, value: String) {
        self.0.entry(section).or_default().insert(key, value);
    }

    /// Retrieves the value for a given section and key.
    ///
    /// # Arguments
    /// * `section` - The section of the manifest.
    /// * `key` - The key within the section.
    ///
    /// # Returns
    /// An `Option` containing the `String` if found, or `None` otherwise.
    pub fn get(&self, section: &str, key: &str) -> Option<&String> {
        self.0.get(section).and_then(|keys| keys.get(key))
    }

    /// Removes an entry.
    ///
    /// # Arguments
    /// * `section` - The section of the manifest.
    /// * `key` - The key within the section.
    ///
    /// # Returns
    /// An `Option` containing the removed `String` if it existed, or `None` otherwise.
    pub fn remove(&mut self, section: &str, key: &str) -> Option<String> {
        self.0.get_mut(section).and_then(|keys| keys.remove(key))
    }

    /// Checks if the manifest contains a specific section.
    ///
    /// # Arguments
    /// * `section` - The section of the manifest.
    ///
    /// # Returns
    /// `true` if the section exists, `false` otherwise.
    pub fn contains_section(&self, section: &str) -> bool {
        self.0.contains_key(section)
    }

    /// Checks if the manifest contains a specific key in a section.
    ///
    /// # Arguments
    /// * `section` - The section of the manifest.
    /// * `key` - The key within the section.
    ///
    /// # Returns
    /// `true` if the key exists in the section, `false` otherwise.
    pub fn contains_key(&self, section: &str, key: &str) -> bool {
        self.0.get(section).is_some_and(|keys| keys.contains_key(key))
    }

    /// Returns an iterator over the sections in the manifest.
    pub fn sections(&self) -> impl Iterator<Item = &String> {
        self.0.keys()
    }

    /// Returns an iterator over the keys and values in a specific section.
    ///
    /// # Arguments
    /// * `section` - The section to get keys and values for.
    ///
    /// # Returns
    /// An iterator over the keys and values in the section, or an empty
    /// iterator if the section doesn't exist.
    pub fn keys(&self, section: &str) -> Box<dyn Iterator<Item = (&String, &String)> + '_> {
        match self.0.get(section) {
            Some(keys) => Box::new(keys.iter()),
            None => Box::new(std::iter::empty()),
        }
    }

    /// Returns an iterator over all (section, name) entries.
    pub fn entries(&self) -> impl Iterator<Item = (&String, &String)> {
        self.0.iter().flat_map(|(section, map)| map.keys().map(move |key| (section, key)))
    }
}

impl Default for IndexManifest {
    fn default() -> Self {
        IndexManifest::new()
    }
}

/// Implement load from file and save to file
impl ManifestFile for IndexManifest {}

impl FromStr for IndexManifest {
    type Err = ManifestError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s).map_err(ManifestError::from)
    }
}

impl FromSlice for IndexManifest {
    type Err = ManifestError;

    fn from_slice(v: &[u8]) -> Result<Self, Self::Err> {
        let s = std::str::from_utf8(v)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
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
        let section = "toolchains".to_string();
        let key = "move".to_string();
        let value = "toolchains/move.toml".to_string();

        manifest.insert(section.clone(), key.clone(), value.clone());
        assert_eq!(manifest.get(&section, &key), Some(&value));
    }

    #[test]
    fn test_remove() {
        let mut manifest = IndexManifest::new();
        let section = "toolchains".to_string();
        let key = "move".to_string();
        let value = "toolchains/move.toml".to_string();

        manifest.insert(section.clone(), key.clone(), value.clone());
        let removed = manifest.remove(&section, &key);
        assert_eq!(removed, Some(value));
        assert!(manifest.get(&section, &key).is_none());
    }

    #[test]
    fn test_contains() {
        let mut manifest = IndexManifest::new();
        let section = "toolchains".to_string();
        let key = "move".to_string();
        let value = "toolchains/move.toml".to_string();

        manifest.insert(section.clone(), key.clone(), value);
        assert!(manifest.contains_section(&section));
        assert!(manifest.contains_key(&section, &key));
        assert!(!manifest.contains_section("nonexistent"));
        assert!(!manifest.contains_key(&section, "nonexistent"));
    }

    #[test]
    fn test_empty_manifest() {
        let manifest = IndexManifest::new();
        assert!(!manifest.contains_section("toolchains"));
        assert!(manifest.get("toolchains", "move").is_none());
    }

    #[test]
    fn test_remove_nonexistent_entry() {
        let mut manifest = IndexManifest::new();
        assert!(manifest.remove("toolchains", "move").is_none());
    }

    #[test]
    fn test_multiple_entries() {
        let mut manifest = IndexManifest::new();
        let section1 = "toolchains".to_string();
        let section2 = "targets".to_string();
        let key1 = "move".to_string();
        let key2 = "aptos".to_string();
        let value1 = "toolchains/move.toml".to_string();
        let value2 = "https://aptos.dev/toolchain.toml".to_string();
        let value3 = "targets/aptos.toml".to_string();

        manifest.insert(section1.clone(), key1.clone(), value1.clone());
        manifest.insert(section1.clone(), key2.clone(), value2.clone());
        manifest.insert(section2.clone(), "aptos".to_string(), value3.clone());

        assert_eq!(manifest.get(&section1, &key1), Some(&value1));
        assert_eq!(manifest.get(&section1, &key2), Some(&value2));
        assert_eq!(manifest.get(&section2, "aptos"), Some(&value3));
        assert!(manifest.contains_section(&section1));
        assert!(manifest.contains_section(&section2));
        assert!(manifest.contains_key(&section1, &key1));
        assert!(manifest.contains_key(&section1, &key2));
    }
}
