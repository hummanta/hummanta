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
use tracing::{debug, info, warn};

use crate::{context::Context, errors::Result, utils};

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
        let languages = self.detect(&detectors, &path).await?;

        match languages.len() {
            0 => warn!("No supported language detected in this directory"),
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
    async fn detect(
        &self,
        detectors: &Vec<PackageEntry>,
        path: &Path,
    ) -> Result<Vec<(String, String)>> {
        let mut languages = HashSet::new();

        for detector in detectors {
            let cmd = utils::command(
                &detector.entry.path,
                &["--path", path.to_str().context("Path contains invalid UTF-8")?],
            )
            .await?;

            if !cmd.status.success() {
                continue;
            }

            let output_str = String::from_utf8(cmd.stdout)?;
            let detector_output = DetectResult::from_str(&output_str)?;
            if !detector_output.pass {
                continue;
            }

            let language =
                detector_output.language.context("Detector did not return a language")?;
            let extension =
                detector_output.extension.context("Detector did not return an extension")?;

            debug!("Detected language: {} using detector {}", language, detector.name);
            languages.insert((language, extension));
        }

        Ok(languages.into_iter().collect())
    }

    /// Prompt user to select from multiple matching languages
    fn prompt_user_selection(&self, matches: &[(String, String)]) -> Result<(String, String)> {
        println!("\nMultiple language detectors matched this project:");
        for (i, (language, _)) in matches.iter().enumerate() {
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
    fn write_config(&self, (language, extension): (String, String)) -> Result<()> {
        let project = Project::new(&language, &extension);
        let manifest = ProjectManifest::new(project);

        manifest.save("hummanta.toml")?;
        info!("Successfully initialized project with language: {}", language);

        Ok(())
    }
}
