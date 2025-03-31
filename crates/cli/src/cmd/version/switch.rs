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

use clap::Args;
use std::sync::Arc;

use crate::{context::Context, errors::Result};

/// Change active version
#[derive(Args, Debug)]
pub struct Command {
    /// Target version to activate
    pub version: String,
}

impl Command {
    pub fn exec(&self, ctx: Arc<Context>) -> Result<()> {
        let version = self.version.trim();
        if !version.starts_with('v') {
            anyhow::bail!("Version must start with 'v'");
        }

        // Validate version exists
        let manifests_path = ctx.manifests_dir().join(version);
        if !manifests_path.exists() {
            anyhow::bail!("Version {} is not installed (missing manifests)", version);
        }

        // Update config
        let mut config = ctx.config.clone();
        config.active_version = Some(version.to_string());
        config.save(&ctx.config_path)?;

        println!("Switched to version {}", version);
        Ok(())
    }
}
