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

use std::{path::Path, sync::Arc};

use anyhow::Context as _;
use clap::Args;
use tokio::fs;

use crate::{context::Context, errors::Result, utils::confirm};

/// Remove a version
#[derive(Args, Debug)]
pub struct Command {
    /// Specific version to remove
    pub version: String,

    /// Skip confirmation prompt
    #[clap(short, long)]
    pub force: bool,
}

impl Command {
    pub async fn exec(&self, ctx: Arc<Context>) -> Result<()> {
        let version = &self.version;

        // Validate version exists
        let manifests_path =
            ctx.manifests_dir().context("Failed to get manifests directory")?.join(version);
        let toolchains_path =
            ctx.toolchains_dir().context("Failed to get toolchains directory")?.join(version);

        if !manifests_path.exists() && !toolchains_path.exists() {
            anyhow::bail!("Version {} does not exist", version);
        }

        // Show removal preview
        println!("The following directories will be removed:");
        if manifests_path.exists() {
            println!("- {}", manifests_path.display());
        }
        if toolchains_path.exists() {
            println!("- {}", toolchains_path.display());
        }

        // Confirm removal
        let prompt = format!("Are you sure you want to remove version {}? [y/N]", version);
        if !self.force && !confirm(&prompt)? {
            println!("Removal cancelled");
            return Ok(());
        }

        // Perform removal
        remove(&manifests_path, &toolchains_path).await?;
        println!("Successfully removed version {}", version);

        Ok(())
    }
}

async fn remove(manifests_path: &Path, toolchains_path: &Path) -> Result<()> {
    // Remove manifests (non-critical if missing)
    if manifests_path.exists() {
        fs::remove_dir_all(manifests_path).await.context("Failed to remove manifests directory")?;
    }

    // Remove toolchains (non-critical if missing)
    if toolchains_path.exists() {
        fs::remove_dir_all(toolchains_path)
            .await
            .context("Failed to remove toolchains directory")?;
    }

    Ok(())
}
