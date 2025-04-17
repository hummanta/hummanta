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

use anyhow::Context as _;

use crate::{config::Config, errors::Result};

/// Holds the state of the application.
pub struct Context {
    /// The configuration for the application.
    pub config: Config,

    /// The path to the configuration.
    pub config_path: PathBuf,
}

impl Context {
    /// Creates a new context with loaded configuration
    pub fn new() -> Result<Self> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?
            .join(".hummanta");

        // Ensure home directory exists
        if !home_dir.exists() {
            std::fs::create_dir_all(&home_dir)
                .context("Failed to create Hummanta home directory")?;
        }

        let config_path = home_dir.join("config.toml");
        let config = Config::load(&config_path)?;

        Ok(Self { config, config_path })
    }

    /// Gets currently active version.
    pub fn version(&self) -> String {
        format!("v{}", env!("CARGO_PKG_VERSION"))
    }

    /// Gets the path to the Hummanta home directory.
    pub fn home_dir(&self) -> PathBuf {
        self.config_path.parent().unwrap().to_path_buf()
    }

    /// Gets the path to the toolchains directory.
    pub fn toolchains_dir(&self) -> PathBuf {
        self.home_dir().join("toolchains")
    }

    /// Gets the path to the manifests directory.
    pub fn manifests_dir(&self) -> PathBuf {
        self.home_dir().join("manifests")
    }

    #[allow(dead_code)]
    /// Computes the final registry URL based on the priority:
    /// CLI > Environment > Config > Default.
    pub fn registry(&self, registry: Option<String>) -> String {
        registry
            .or_else(|| std::env::var("HUMMANTA_REGISTRY").ok())
            .unwrap_or_else(|| self.config.registry.clone()) // 使用配置文件中的 registry
    }
}
