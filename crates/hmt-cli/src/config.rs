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

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::errors::Result;

const DEFAULT_REGISTRY: &str = "https://hummanta.github.io/registry";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// The currently active version.
    pub active_version: Option<String>,

    /// The URL of the registry to use.
    ///
    /// This can be overridden by the CLI argument `--registry`,
    /// the environment variable `HUMMANTA_REGISTRY`,
    /// or left as the default.
    pub registry: String,
}

impl Default for Config {
    fn default() -> Self {
        Self { active_version: None, registry: DEFAULT_REGISTRY.to_string() }
    }
}

impl Config {
    pub fn load(path: &PathBuf) -> Result<Self> {
        if path.exists() {
            let content = std::fs::read_to_string(path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self, path: &PathBuf) -> Result<()> {
        let content = toml::to_string(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
