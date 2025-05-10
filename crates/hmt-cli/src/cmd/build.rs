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

use anyhow::{anyhow, bail, Context as _};
use clap::Args;
use once_cell::sync::OnceCell;
use tracing::info;
use walkdir::WalkDir;

use hmt_manifest::{ManifestFile, ProjectManifest};
use hmt_registry::traits::Query;

use crate::{context::Context, errors::Result, utils};

/// Builds the entire workspace
#[derive(Args, Debug)]
pub struct Command {
    /// The target platform to build for
    #[arg(long)]
    target: Option<String>,

    /// The resolved target platform, determined by CLI or manifest
    #[clap(skip)]
    resolved_target: OnceCell<String>,
}

impl Command {
    pub async fn exec(&self, ctx: Arc<Context>) -> Result<()> {
        let manifest = ProjectManifest::load("hummanta.toml")
            .with_context(|| "Could not find 'hummanta.toml'. Please run `hummanta init` first.")?;

        let target = self.target(&manifest)?;
        let target_dir = self.target_dir(target)?;

        // Execute the complete build pipeline
        self.compile(ctx.clone(), &manifest, &target_dir).await?;
        self.emit(ctx.clone(), &manifest, &target_dir).await?;

        info!("Build completed for target '{}'", target);
        Ok(())
    }

    /// Resolve target with clear precedence: CLI arg > manifest > error
    fn target(&self, manifest: &ProjectManifest) -> Result<&str> {
        self.resolved_target.get_or_try_init(|| {
            if let Some(cli_target) = &self.target {
                if !cli_target.is_empty() {
                    return Ok(cli_target.to_owned());
                }
                bail!("Empty target specified in command line");
            }

            if let Some(manifest_target) = &manifest.project.target {
                if !manifest_target.is_empty() {
                    return Ok(manifest_target.to_owned());
                }
                bail!("Empty target specified in manifest");
            }

            bail!("No target specified. Either set 'target' in hummanta.toml or use --target flag")
        }).map(|s| s.as_str())
    }

    /// Prepares and validates the build output directory
    fn target_dir(&self, target: &str) -> Result<PathBuf> {
        let target_dir = Path::new("target").join(target);
        if !target_dir.exists() {
            fs::create_dir_all(&target_dir) //
                .context("Failed to create target directory")?;
        }
        Ok(target_dir)
    }

    /// Compiles source code to intermediate representation (CLIF)
    async fn compile(
        &self,
        ctx: Arc<Context>,
        manifest: &ProjectManifest,
        target_dir: &Path,
    ) -> Result<()> {
        // Acquires the toolchain manager.
        let manager = ctx.toolchains().await?;
        let manager = manager.read().await;

        let language = &manifest.project.language;
        let extension = manifest.project.extension.as_str();

        // Get the appropriate frontend compiler
        let packages = manager.get_package(language, "frontend");
        let package = packages
            .first()
            .ok_or_else(|| anyhow!("Frontend compiler for '{}' not found", language))?;
        let compiler_path = &package.entry.path;

        // Process all source files with the matching language extension
        for entry in WalkDir::new(".")
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.path().extension().is_some_and(|ext| ext == extension))
        {
            let input = entry.path();
            let file_stem = input
                .file_stem()
                .ok_or_else(|| anyhow!("Source file has no valid name: {}", input.display()))?;
            let output = target_dir.join(file_stem).with_extension("clif");

            let cmd = utils::command(
                compiler_path,
                &[
                    "--input",
                    input.to_str().context("Invalid input path")?,
                    "--output",
                    output.to_str().context("Invalid output path")?,
                ],
            )
            .await?;

            if !cmd.status.success() {
                let stderr = String::from_utf8_lossy(&cmd.stderr);
                bail!("Compilation failed with status {}:\n{}", cmd.status, stderr.trim());
            }

            info!("Compiled: {} → {}", input.display(), output.display());
        }

        Ok(())
    }

    /// Compiles intermediate representation (CLIF) to target machine code
    async fn emit(
        &self,
        ctx: Arc<Context>,
        manifest: &ProjectManifest,
        target_dir: &PathBuf,
    ) -> Result<()> {
        let manager = ctx.targets().await?;
        let manager = manager.read().await;

        let target = self.target(manifest)?;

        // Get the appropriate backend compiler
        let packages = manager.get_package(target, "backend");
        let package =
            packages.first().ok_or(anyhow!("Backend compiler for '{}' not found", target))?;
        let compiler_path = &package.entry.path;

        // Process all intermediate .clif files
        for entry in fs::read_dir(target_dir)?
            .filter_map(Result::ok)
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "clif"))
        {
            let input = entry.path();
            let output = input.with_extension("o");

            let cmd = utils::command(
                compiler_path,
                &[
                    "--input",
                    input.to_str().context("Invalid input path")?,
                    "--output",
                    output.to_str().context("Invalid output path")?,
                ],
            )
            .await?;

            if !cmd.status.success() {
                let stderr = String::from_utf8_lossy(&cmd.stderr);
                bail!("Compilation failed with status {}:\n{}", cmd.status, stderr.trim());
            }

            info!("Compiled: {} → {}", input.display(), output.display());
        }

        Ok(())
    }
}
