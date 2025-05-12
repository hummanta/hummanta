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

mod args;
mod package;
mod release;

use anyhow::{anyhow, Context, Result};
use args::Args;
use clap::Parser;

use hmt_manifest::{ManifestFile, Package};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let version = &args.version;

    // load package configuration
    let package = Package::load(&args.package)
        .context(format!("Failed to read package config from file: {}", args.package.display()))?;

    if !args.artifacts_dir.exists() {
        return Err(anyhow!("Artifacts dir does not exist: {}", args.artifacts_dir.display()));
    }

    // Create output directory if it doesn't exist
    std::fs::create_dir_all(&args.output_dir)?;

    // Generate release manifest and save to path
    let release = release::generate(&package, &args.artifacts_dir, version)?;
    release.save(args.output_dir.join(format!("release-{}.toml", version)))?;

    // Update or create package manifest
    let index_path = args.output_dir.join("index.toml");
    if index_path.exists() {
        package::update(&package, &index_path, version)?;
    } else {
        package::create(&package, &index_path, version)?;
    }

    info!("Manifests generated successfully!");
    Ok(())
}
