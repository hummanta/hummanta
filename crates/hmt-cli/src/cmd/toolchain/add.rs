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

use hmt_fetcher::{FetchContext, Fetcher};
use hmt_manifest::{TargetInfo, Toolchain, ToolchainManifest};
use hmt_utils::archive;

use crate::{context::Context, errors::Result};

/// Installs the specified language's toolchain.
#[derive(Args, Debug)]
pub struct Command {
    /// The language to install the toolchain for.
    language: String,
}

impl Command {
    pub async fn exec(&self, ctx: Arc<Context>) -> Result<()> {
        let version = ctx.version();

        // Load manifest for this language and version
        let manifest_path = ctx
            .manifests_dir()
            .join(&version)
            .join("toolchains")
            .join(format!("{}.toml", self.language));

        if !manifest_path.exists() {
            return Err(anyhow::anyhow!(
                "Manifest not found for {} in version {} at {}",
                self.language,
                version,
                manifest_path.display()
            ));
        }

        // Create toolchain directory
        let toolchain_dir = ctx.toolchains_dir().join(&version).join(&self.language);
        fs::create_dir_all(&toolchain_dir).await.context("Failed to create toolchain directory")?;

        let manifest = ToolchainManifest::read(manifest_path)?;
        self.installs(&manifest, &toolchain_dir).await?;

        println!(
            "Successfully installed {} toolchain (version: {}) at {}",
            self.language,
            version,
            toolchain_dir.display()
        );
        Ok(())
    }

    async fn installs(&self, manifest: &ToolchainManifest, target_dir: &Path) -> Result<()> {
        let current_target = target_triple::TARGET;
        let mut handles = Vec::new();

        manifest.values().for_each(|tools| {
            tools
                .iter()
                .filter_map(|(name, toolchain)| match toolchain {
                    Toolchain::Release(release) => Some((name, release)),
                    _ => None,
                })
                .filter_map(|(name, release)| {
                    release
                        .get_target_info(current_target)
                        .map(|target| (name.to_string(), target.clone()))
                })
                .for_each(|(_, target)| {
                    let target = target.clone();
                    let target_dir = target_dir.to_path_buf();
                    handles.push(tokio::spawn(async move { install(&target, &target_dir).await }));
                });
        });

        for handle in handles {
            handle.await.context("Failed to join task")??;
        }

        Ok(())
    }
}

async fn install(target: &TargetInfo, target_dir: &Path) -> Result<()> {
    // Fetch and verify the checksum
    let context = FetchContext::new(&target.url).checksum(&target.hash);
    let data = Fetcher::default().fetch(&context).await?;

    // Unpack the file and extract its contents to the target directory
    archive::unpack(&data, target_dir)?;

    Ok(())
}
