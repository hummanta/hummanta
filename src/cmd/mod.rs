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

use clap::Parser;

use crate::context::Context;
use crate::errors::Result;

#[derive(Parser)]
#[command(arg_required_else_help = true, disable_help_subcommand = false)]
pub struct Cli {}

impl Cli {
    pub fn exec(&self, _ctx: Arc<Context>) -> Result<()> {
        println!("Hello, world!");

        Ok(())
    }
}
