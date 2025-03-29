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
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Context as _;
use clap::Args;

use crate::{context::Context, errors::Result};

/// Removes the toolchain for the specified language.
#[derive(Args, Debug)]
pub struct Command {
    /// The language to remove the toolchain for.
    language: String,

    /// Specific version to remove (removes all versions if not specified)
    #[arg(short, long)]
    version: Option<String>,

    /// Skip confirmation prompt
    #[arg(short, long)]
    force: bool,
}

impl Command {
    pub fn exec(&self, _ctx: Arc<Context>) -> Result<()> {
        let toolchains_dir =
            dirs::home_dir().context("Failed to get home directory")?.join(".hummanta/toolchains");

        // Find matching toolchains to remove
        let toolchains = find_remove_toolchains(&toolchains_dir, &self.language, &self.version)?;

        if toolchains.is_empty() {
            println!("No matching toolchains found to remove");
            return Ok(());
        }

        // Confirm removal with user (unless force flag is set)
        if !confirm_removal(&toolchains, self.force)? {
            println!("Aborted");
            return Ok(());
        }

        // Execute the removal
        remove_toolchains(toolchains)
    }
}

/// Finds all toolchain directories matching the removal criteria
fn find_remove_toolchains(
    base_dir: &Path,
    language: &str,
    version: &Option<String>,
) -> Result<Vec<PathBuf>> {
    let mut toolchains = Vec::new();

    for version_entry in fs::read_dir(base_dir)? {
        let version_entry = version_entry?;
        let version_path = version_entry.path();

        // Filter by version if specified
        if let Some(ver) = version {
            if !version_path
                .file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.contains(ver))
                .unwrap_or(false)
            {
                continue;
            }
        }

        // Check if language directory exists
        let toolchain_path = version_path.join(language);
        if toolchain_path.exists() && toolchain_path.is_dir() {
            toolchains.push(toolchain_path);
        }
    }

    Ok(toolchains)
}

/// Prompts user for confirmation before removal
fn confirm_removal(targets: &[PathBuf], force: bool) -> Result<bool> {
    println!("The following toolchains will be removed:");
    for path in targets {
        println!("- {}", path.display());
    }

    if !force {
        println!("Are you sure you want to continue? [y/N]");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        return Ok(input.trim().eq_ignore_ascii_case("y"));
    }

    Ok(true)
}

/// Performs the actual directory removal
fn remove_toolchains(targets: Vec<PathBuf>) -> Result<()> {
    for path in targets {
        if path.exists() {
            fs::remove_dir_all(&path)?;
            println!("Removed: {}", path.display());

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
