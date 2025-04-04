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
    collections::HashSet,
    io::{self, Write},
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
};

use anyhow::Context as _;
use clap::Args;

use hummanta_detection::DetectResult;
use hummanta_manifest::{IndexManifest, ProjectManifest, Toolchain, ToolchainManifest};

use crate::{context::Context, errors::Result};

/// Initializes the workspace
#[derive(Args, Debug)]
pub struct Command {}

impl Command {
    pub fn exec(&self, ctx: Arc<Context>) -> Result<()> {
        // Get all detectors
        let detectors = self.get_detectors(&ctx)?;

        // Execute detectors and find matching languages
        let path = std::env::current_dir()?;
        let languages = self.detect(detectors, &path)?;

        match languages.len() {
            0 => println!("No supported language detected in this directory"),
            1 => self.write_config(languages[0].clone())?,
            _ => {
                // Multiple matches - let user choose
                let selected = self.prompt_user_selection(&languages)?;
                self.write_config(selected)?
            }
        }

        Ok(())
    }

    /// Get all available detectors for the current version
    fn get_detectors(&self, ctx: &Context) -> Result<Vec<(String, PathBuf, String)>> {
        let version = ctx.version();
        let manifest_path =
            ctx.manifests_dir().join(&version).join("toolchains").join("index.toml");

        if !manifest_path.exists() {
            return Err(anyhow::anyhow!(
                "Manifest not found in version {} at {}",
                &version,
                manifest_path.display()
            ));
        }

        let index = IndexManifest::read(&manifest_path)?;
        let mut detectors = Vec::new();

        for (toolchain_name, toolchain_file) in index.iter() {
            let toolchain_manifest_path =
                ctx.manifests_dir().join(&version).join("toolchains").join(toolchain_file);

            if !toolchain_manifest_path.exists() {
                continue;
            }

            let toolchain_manifest = ToolchainManifest::read(&toolchain_manifest_path)?;

            if let Some(detector_toolchains) = toolchain_manifest.by_category("detector") {
                for (detector_name, detector) in detector_toolchains {
                    let binary_name = match detector {
                        Toolchain::Package(pkg) => pkg.name().to_string(),
                        Toolchain::Release(_) => detector_name.clone(),
                    };

                    let binary_path =
                        ctx.toolchains_dir().join(&version).join(toolchain_name).join(&binary_name);

                    if binary_path.exists() {
                        detectors.push((
                            detector_name.clone(),
                            binary_path,
                            toolchain_name.clone(),
                        ));
                    }
                }
            }
        }

        Ok(detectors)
    }

    /// Execute all detectors and return all matching languages
    fn detect(
        &self,
        detectors: Vec<(String, PathBuf, String)>,
        path: &Path,
    ) -> Result<Vec<String>> {
        let mut languages = HashSet::new();

        for (detector_name, binary_path, toolchain_name) in detectors {
            let output = std::process::Command::new(&binary_path)
                .arg("--path")
                .arg(path)
                .output()
                .with_context(|| {
                    format!(
                        "Failed to execute detector {} from toolchain {} at {:?}",
                        detector_name, toolchain_name, binary_path
                    )
                })?;

            if !output.status.success() {
                continue;
            }

            let output_str = String::from_utf8(output.stdout)?;
            let detector_output = DetectResult::from_str(&output_str)?;
            if !detector_output.pass {
                continue;
            }

            let language = detector_output.language.unwrap();
            println!("Detected language: {} using detector {}", language, detector_name);
            languages.insert(language);
        }

        Ok(languages.into_iter().collect())
    }

    /// Prompt user to select from multiple matching languages
    fn prompt_user_selection(&self, matches: &[String]) -> Result<String> {
        println!("\nMultiple language detectors matched this project:");
        for (i, language) in matches.iter().enumerate() {
            println!("{}. {}", i + 1, language);
        }

        loop {
            print!("\nPlease select a language (1-{}): ", matches.len());
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            if let Ok(choice) = input.trim().parse::<usize>() {
                if choice >= 1 && choice <= matches.len() {
                    return Ok(matches[choice - 1].clone());
                }
            }

            println!("Invalid selection. Please enter a number between 1 and {}", matches.len());
        }
    }

    /// Write the detected language to hummanta.toml
    fn write_config(&self, language: String) -> Result<()> {
        let manifest = ProjectManifest::new(language);
        manifest.write("hummanta.toml")?;

        println!("\nSuccessfully initialized project with language: {}", manifest.language);
        Ok(())
    }
}
