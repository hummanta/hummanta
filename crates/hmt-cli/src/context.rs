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

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::{Context as _, Ok};
use tokio::sync::{OnceCell, RwLock};
use tracing::debug;

use hmt_registry::{
    manager::{TargetManager, ToolchainManager},
    RegistryClient,
};

use crate::{config::Config, errors::Result, utils};

/// Holds the state of the application.
pub struct Context {
    /// The configuration for the application.
    pub config: Config,

    /// The path to the configuration.
    pub config_path: PathBuf,

    /// Overridden registry URL
    registry: Option<String>,

    /// Lazily initialized target manager
    target_manager: OnceCell<Arc<RwLock<TargetManager>>>,

    /// Lazily initialized toolchain manager
    toolchain_manager: OnceCell<Arc<RwLock<ToolchainManager>>>,

    /// The path to the project manifest.
    manifest_path: Option<PathBuf>,
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
        let manifest_path = utils::find("hummanta.toml").ok();

        let context = Self {
            config,
            config_path,
            registry: registry.clone(),
            target_manager: OnceCell::new(),
            toolchain_manager: OnceCell::new(),
            manifest_path,
        };
        debug!("Registry: {}", context.registry());

        Ok(context)
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

    /// Gets the target manager, initializing it if necessary
    pub async fn targets(&self) -> Result<Arc<RwLock<TargetManager>>> {
        self.target_manager
            .get_or_try_init(|| async {
                let registry = RegistryClient::new(&self.registry());
                Ok(Arc::new(RwLock::new(TargetManager::new(registry, self.home_dir()))))
            })
            .await
            .cloned()
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

    /// Gets the path to the Hummanta project manifest.
    pub fn manifest_path(&self) -> Result<&PathBuf> {
        self.manifest_path.as_ref().ok_or_else(|| {
            anyhow::anyhow!("Could not find 'hummanta.toml'. Please run `hummanta init` first.")
        })
    }

    /// Determine project directory from manifest path
    pub fn project_dir(&self) -> Result<&Path> {
        self.manifest_path()?.parent().ok_or_else(|| {
            anyhow::anyhow!("Could not determine project directory from manifest path")
        })
    }
}
