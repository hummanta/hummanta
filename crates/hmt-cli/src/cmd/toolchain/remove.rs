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

use anyhow::Ok;
use clap::Args;
use hmt_registry::{manager::ToolchainManager, traits::PackageManager, RegistryClient};
use std::sync::Arc;

use crate::{context::Context, errors::Result, utils::confirm};

/// Removes the toolchain for the specified language.
#[derive(Args, Debug)]
pub struct Command {
    /// The language to remove the toolchain for.
    language: String,

    /// Skip confirmation prompt
    #[arg(short, long)]
    force: bool,
}

impl Command {
    pub fn exec(&self, ctx: Arc<Context>) -> Result<()> {
        // Confirm removal with user (unless force flag is set)
        if !self.force && !confirm("Are you sure you want to continue? [y/N]")? {
            println!("Removal cancelled");
            return Ok(());
        }

        let registry = RegistryClient::new(&ctx.registry(None));
        let mut manager = ToolchainManager::new(registry, ctx.home_dir());

        // Execute the removal
        manager.remove(&self.language)?;

        Ok(())
    }
}
