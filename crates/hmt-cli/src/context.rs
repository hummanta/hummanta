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

use std::{path::PathBuf, sync::Arc};

use anyhow::Context as _;
use hmt_registry::{manager::ToolchainManager, RegistryClient};
use tokio::sync::{OnceCell, RwLock};

use crate::{config::Config, errors::Result};

/// Holds the state of the application.
pub struct Context {
    /// The configuration for the application.
    pub config: Config,

    /// The path to the configuration.
    pub config_path: PathBuf,

    /// Overridden registry URL
    registry: Option<String>,

    /// Lazily initialized toolchain manager
    toolchain_manager: OnceCell<Arc<RwLock<ToolchainManager>>>,
}

impl Context {
    /// Creates a new context with loaded configuration
    pub fn new(registry: &Option<String>) -> Result<Self> {
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

        Ok(Self {
            config,
            config_path,
            registry: registry.clone(),
            toolchain_manager: OnceCell::new(),
        })
    }

    /// Gets the path to the Hummanta home directory.
    pub fn home_dir(&self) -> PathBuf {
        self.config_path.parent().unwrap().to_path_buf()
    }

    /// Computes the final registry URL based on the priority:
    /// CLI > Environment > Config > Default.
    fn registry(&self) -> String {
        self.registry
            .clone()
            .or_else(|| std::env::var("HUMMANTA_REGISTRY").ok())
            .unwrap_or_else(|| self.config.registry.clone())
    }

    /// Gets the toolchain manager, initializing it if necessary
    pub async fn toolchains(&self) -> Result<Arc<RwLock<ToolchainManager>>> {
        self.toolchain_manager
            .get_or_try_init(|| async {
                let registry = RegistryClient::new(&self.registry());
                Ok(Arc::new(RwLock::new(ToolchainManager::new(registry, self.home_dir()))))
            })
            .await
            .cloned()
    }
}
