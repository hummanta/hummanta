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
use walkdir::WalkDir;

use hmt_manifest::{ManifestFile, ProjectManifest};
use hmt_registry::traits::Query;

use crate::{context::Context, errors::Result};

/// Builds the entire workspace
#[derive(Args, Debug)]
pub struct Command {
    /// The target platform to build for
    #[arg(long)]
    target: Option<String>,
}

impl Command {
    pub async fn exec(&self, ctx: Arc<Context>) -> Result<()> {
        let manifest = ProjectManifest::load("hummanta.toml")?;

        let language = &manifest.project.language;
        let target = self.target(&manifest)?;
        let target_dir = self.target_dir(&target)?;

        // Execute the complete build pipeline
        self.compile(ctx.clone(), language, &target_dir).await?;
        self.emit(ctx.clone(), &target, &target_dir).await?;

        println!("Build completed for target '{}'", target);
        Ok(())
    }

    /// Resolve target with clear precedence: CLI arg > manifest > error
    fn target(&self, manifest: &ProjectManifest) -> Result<String> {
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
    }

    /// Prepares and validates the build output directory
    fn target_dir(&self, target: &str) -> Result<PathBuf> {
        let targets_dir = Path::new("targets").join(target);
        if !targets_dir.exists() {
            fs::create_dir_all(&targets_dir) //
                .context("Failed to create targets directory")?;
        }
        Ok(targets_dir)
    }

    /// Compiles source code to intermediate representation (CLIF)
    async fn compile(&self, ctx: Arc<Context>, language: &str, target_dir: &Path) -> Result<()> {
        // Acquires the toolchain manager.
        let manager = ctx.toolchains().await?;
        let manager = manager.read().await;

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
            .filter(|e| e.path().extension().is_some_and(|ext| ext == language))
        {
            let input = entry.path();
            let file_stem = input
                .file_stem()
                .ok_or_else(|| anyhow!("Source file has no valid name: {}", input.display()))?;
            let output = target_dir.join(file_stem).with_extension("clif");

            run_command(
                compiler_path,
                &[
                    "--input",
                    input.to_str().context("Invalid input path")?,
                    "--output",
                    input.to_str().context("Invalid output path")?,
                ],
            )?;

            println!("Compiled: {} → {}", input.display(), output.display());
        }

        Ok(())
    }

    /// Compiles intermediate representation (CLIF) to target machine code
    async fn emit(&self, ctx: Arc<Context>, target: &str, target_dir: &PathBuf) -> Result<()> {
        let manager = ctx.targets().await?;
        let manager = manager.read().await;

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

            run_command(
                compiler_path,
                &[
                    "--input",
                    input.to_str().context("Invalid input path")?,
                    "--output",
                    input.to_str().context("Invalid output path")?,
                ],
            )?;

            println!("Compiled: {} → {}", input.display(), output.display());
        }

        Ok(())
    }
}

/// Executes a shell command with proper error handling
fn run_command(program: &Path, args: &[&str]) -> Result<()> {
    let status = std::process::Command::new(program).args(args).status().context(format!(
        "Failed to execute: {} {}",
        program.display(),
        args.join(" ")
    ))?;

    if !status.success() {
        bail!(
            "Command failed with exit code {:?}: {} {}",
            status.code(),
            program.display(),
            args.join(" ")
        );
    }
    Ok(())
}
