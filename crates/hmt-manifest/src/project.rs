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
use std::{io::Read, path::Path, str::FromStr};

use crate::{error::ManifestResult, ManifestError};

/// `ProjectManifest` is a struct used to represent a project-specific settings.
///
/// Example:
/// ```toml
/// language = "Solidity"
/// ```
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProjectManifest {
    /// The programming language used for the source code in this project.
    pub language: String,
}

impl ProjectManifest {
    /// Creates a new instance with the specified language.
    pub fn new(language: String) -> Self {
        ProjectManifest { language }
    }
}

impl ProjectManifest
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
        let toml_string = toml::to_string(&self)?;
        std::fs::write(path, toml_string)?;

        Ok(())
    }
}

impl std::str::FromStr for ProjectManifest {
    type Err = ManifestError;

    fn from_str(s: &str) -> ManifestResult<Self> {
        toml::from_str(s).map_err(ManifestError::from)
    }
}
