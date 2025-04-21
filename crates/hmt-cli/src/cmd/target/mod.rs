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

mod add;
mod list;
mod remove;
mod show;

use std::sync::Arc;

use crate::{context::Context, errors::Result};
use clap::{Args, Subcommand};

/// View and manage compilation targets
#[derive(Args, Debug)]
pub struct Command {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add(add::Command),
    Remove(remove::Command),
    Show(show::Command),
    List(list::Command),
}

impl Command {
    pub async fn exec(&self, ctx: Arc<Context>) -> Result<()> {
        match &self.command {
            Commands::Add(cmd) => cmd.exec(ctx).await,
            Commands::Remove(cmd) => cmd.exec(ctx),
            Commands::Show(cmd) => cmd.exec(ctx),
            Commands::List(cmd) => cmd.exec(ctx),
        }
    }
}
