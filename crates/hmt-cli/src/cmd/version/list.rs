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

use std::sync::Arc;

use anyhow::Context as _;
use clap::Args;
use tokio::fs;

use crate::{context::Context, errors::Result};

/// List all installed versions
#[derive(Args, Debug)]
pub struct Command {}

impl Command {
    pub async fn exec(&self, ctx: Arc<Context>) -> Result<()> {
        let active_version = ctx.version();
        let manifests_dir = ctx.manifests_dir();

        let mut versions = Vec::new();
        let mut entries =
            fs::read_dir(&manifests_dir).await.context("Failed to read manifests directory")?;

        while let Some(entry) = entries.next_entry().await? {
            if entry.file_type().await?.is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    versions.push(name.to_string());
                }
            }
        }

        // Sort versions newest first (reverse order)
        versions.sort_by(|a, b| b.cmp(a));

        // Display versions with active marker
        for version in versions {
            if version == active_version {
                println!("* {} (active)", version);
            } else {
                println!("  {}", version);
            }
        }

        Ok(())
    }
}
