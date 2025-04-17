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
    path::Path,
    str::FromStr,
    sync::Arc,
};

use anyhow::Context as _;
use clap::Args;

use hmt_detection::DetectResult;
use hmt_manifest::{ManifestFile, PackageEntry, Project, ProjectManifest};
use hmt_registry::traits::Query;

use crate::{context::Context, errors::Result};

/// Initializes the workspace
#[derive(Args, Debug)]
pub struct Command {}

impl Command {
    pub async fn exec(&self, ctx: Arc<Context>) -> Result<()> {
        // Acquires the toolchain manager.
        let manager = ctx.toolchains().await?;
        let manager = manager.read().await;

        // Get all detectors
        let detectors = manager.by_category("detector");

        // Execute detectors and find matching languages
        let path = std::env::current_dir()?;
        let languages = self.detect(&detectors, &path)?;

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

    /// Execute all detectors and return all matching languages
    fn detect(&self, detectors: &Vec<PackageEntry>, path: &Path) -> Result<Vec<String>> {
        let mut languages = HashSet::new();

        for detector in detectors {
            let binary_path = &detector.entry.path;
            let output = std::process::Command::new(binary_path)
                .arg("--path")
                .arg(path)
                .output()
                .with_context(|| {
                format!(
                    "Failed to execute detector {} from toolchain {} at {:?}",
                    detector.name, detector.domain, binary_path
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
            println!("Detected language: {} using detector {}", language, detector.name);
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
        let project = Project { language: language.clone() };
        let manifest = ProjectManifest::new(project);

        manifest.save("hummanta.toml")?;
        println!("\nSuccessfully initialized project with language: {}", language);

        Ok(())
    }
}
