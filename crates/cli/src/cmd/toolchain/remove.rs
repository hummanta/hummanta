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

use std::{fs, path::PathBuf, sync::Arc};

use anyhow::Context as _;
use clap::Args;

use crate::{context::Context, errors::Result, utils::confirm};

/// Removes the toolchain for the specified language.
#[derive(Args, Debug)]
pub struct Command {
    /// The language to remove the toolchain for.
    language: String,

    /// Specific version to remove (default: current active version)
    #[arg(short, long)]
    version: Option<String>,

    /// Remove all versions of this toolchain
    #[arg(short, long)]
    all: bool,

    /// Skip confirmation prompt
    #[arg(short, long)]
    force: bool,
}

impl Command {
    pub fn exec(&self, ctx: Arc<Context>) -> Result<()> {
        let toolchains_dir =
            ctx.toolchains_dir().context("Failed to determine toolchains directory")?;

        let versions = self.resolve_versions(&ctx)?;
        let mut toolchains = Vec::new();

        // Finds all toolchain directories matching the removal criteria
        for version in versions {
            let toolchain_path = toolchains_dir.join(&version).join(&self.language);
            if toolchain_path.exists() {
                toolchains.push((version, toolchain_path));
            }
        }

        if toolchains.is_empty() {
            println!("No matching toolchains found to remove");
            return Ok(());
        }

        // Show removal preview
        println!("The following toolchains will be removed:");
        for (version, path) in &toolchains {
            println!("- {} (version: {})", path.display(), version);
        }

        // Confirm removal with user (unless force flag is set)
        if !self.force && !confirm("Are you sure you want to continue? [y/N]")? {
            println!("Removal cancelled");
            return Ok(());
        }

        // Execute the removal
        self.remove_toolchains(&toolchains)
    }

    fn resolve_versions(&self, ctx: &Context) -> Result<Vec<String>> {
        match (&self.version, self.all) {
            (Some(ver), _) => Ok(vec![ver.clone()]),
            (None, true) => self.find_all_versions(ctx),
            (None, false) => Ok(vec![ctx.version()]),
        }
    }

    fn find_all_versions(&self, ctx: &Context) -> Result<Vec<String>> {
        let toolchains_dir =
            ctx.toolchains_dir().context("Failed to determine toolchains directory")?;

        let mut versions = Vec::new();
        for entry in fs::read_dir(toolchains_dir)? {
            let path = entry?.path();
            if let Some(name) = path.file_name() {
                versions.push(name.to_string_lossy().into_owned());
            }
        }
        Ok(versions)
    }

    /// Performs the actual directory removal
    fn remove_toolchains(&self, targets: &[(String, PathBuf)]) -> Result<()> {
        for (version, path) in targets {
            if path.exists() {
                fs::remove_dir_all(path)?;
                println!("Removed: {} (version: {})", path.display(), version);

                // Clean up empty version directories
                if let Some(parent) = path.parent() {
                    if fs::read_dir(parent)?.next().is_none() {
                        fs::remove_dir(parent)?;
                    }
                }
            }
        }
        Ok(())
    }
}
