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

mod build;
mod compile;
mod init;
mod target;
mod toolchain;

use std::sync::Arc;

use clap::{Parser, Subcommand};

use crate::{context::Context, errors::Result};

#[derive(Parser)]
#[command(arg_required_else_help = true, disable_help_subcommand = false)]
pub struct Command {
    #[command(subcommand)]
    command: Commands,

    /// Override the registry URL.
    #[arg(long, global = true, env = "HUMMANTA_REGISTRY")]
    pub registry: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    Build(build::Command),
    Compile(compile::Command),
    Init(init::Command),
    Target(target::Command),
    Toolchain(toolchain::Command),
}

impl Command {
    pub async fn exec(&self, ctx: Arc<Context>) -> Result<()> {
        match &self.command {
            Commands::Build(cmd) => cmd.exec(ctx),
            Commands::Compile(cmd) => cmd.exec(ctx),
            Commands::Init(cmd) => cmd.exec(ctx).await,
            Commands::Target(cmd) => cmd.exec(ctx).await,
            Commands::Toolchain(cmd) => cmd.exec(ctx).await,
        }
    }
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    #[test]
    fn verify_command() {
        super::Command::command().debug_assert();
    }
}
