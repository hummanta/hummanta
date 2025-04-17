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

use clap::Args;
use hmt_registry::{manager::ToolchainManager, traits::PackageManager, RegistryClient};

use crate::{context::Context, errors::Result};

/// Installs the specified language's toolchain.
#[derive(Args, Debug)]
pub struct Command {
    /// The language to install the toolchain for.
    language: String,
}

impl Command {
    pub async fn exec(&self, ctx: Arc<Context>) -> Result<()> {
        let registry = RegistryClient::new(&ctx.registry(None));
        let mut manager = ToolchainManager::new(registry, ctx.home_dir());

        manager.add(&self.language).await?;
        println!("Successfully installed {} toolchains", self.language);

        Ok(())
    }
}
