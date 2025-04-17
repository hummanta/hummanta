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

mod error;
mod index;
mod installed;
mod package;
mod project;
mod release;

use serde::Serialize;
use std::{io::Read, path::Path, str::FromStr};

// Re-exports.
pub use error::*;
pub use index::*;
pub use installed::*;
pub use package::*;
pub use project::*;
pub use release::*;

/// `ManifestFile` trait provides common file operations for manifest files.
pub trait ManifestFile: FromStr<Err = ManifestError> + Serialize {
    /// Load the manifest from a file.
    fn load<P: AsRef<Path>>(path: P) -> ManifestResult<Self> {
        let mut file = std::fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Self::from_str(&contents)
    }

    /// Save the manifest to a file.
    fn save<P: AsRef<Path>>(&self, path: P) -> ManifestResult<()> {
        let toml_string = toml::to_string_pretty(&self)?;
        std::fs::write(path, toml_string)?;

        Ok(())
    }
}
