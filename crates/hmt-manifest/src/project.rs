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

use crate::{error::ManifestResult, ManifestError, ManifestFile};

/// `ProjectManifest` is a struct used to represent a project-specific settings.
///
/// Example:
/// ```toml
/// language = "Solidity"
/// ```
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProjectManifest {
    /// Metadata for the project, such as language and build.
    #[serde(flatten)]
    pub project: Project,
}

impl ProjectManifest {
    /// Creates a new instance with the specified language.
    pub fn new(project: Project) -> Self {
        ProjectManifest { project }
    }
}

/// Implement load from file and save to file
impl ManifestFile for ProjectManifest {}

impl std::str::FromStr for ProjectManifest {
    type Err = ManifestError;

    fn from_str(s: &str) -> ManifestResult<Self> {
        toml::from_str(s).map_err(ManifestError::from)
    }
}

/// `Project` contains general metadata for a project.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Project {
    /// The programming language used for the source code in this project.
    pub language: String,
}
