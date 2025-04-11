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
use std::{fs, sync::Arc};

use crate::{context::Context, errors::Result};

/// Lists all toolchains
#[derive(Args, Debug)]
pub struct Command {}

impl Command {
    pub fn exec(&self, ctx: Arc<Context>) -> Result<()> {
        let toolchains_dir = ctx.toolchains_dir();

        let mut pairs = Vec::new();

        for version_entry in fs::read_dir(&toolchains_dir)? {
            let version_entry = version_entry?;
            if !version_entry.path().is_dir() {
                continue;
            }

            for toolchain_entry in fs::read_dir(version_entry.path())? {
                let toolchain_entry = toolchain_entry?;
                if !toolchain_entry.path().is_dir() {
                    continue;
                }

                pairs.push((
                    toolchain_entry.path().file_name().unwrap().to_str().unwrap().to_string(),
                    version_entry.path().file_name().unwrap().to_str().unwrap().to_string(),
                ));
            }
        }

        pairs.sort_by(|a, b| a.0.cmp(&b.0));

        for (toolchain, version) in pairs {
            if version == ctx.version() {
                println!("* {} ({}) (active)", toolchain, version);
            } else {
                println!("  {} ({})", toolchain, version);
            }
        }

        Ok(())
    }
}
