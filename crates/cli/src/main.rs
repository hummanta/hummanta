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

mod cmd;
mod context;
mod errors;

use std::sync::Arc;

use clap::Parser;
use cmd::Command;
use context::Context;
use errors::Result;
use tracing::error;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let ctx = Arc::new(Context::default());
    if let Err(err) = Command::parse().exec(ctx) {
        error!("{}", err);
        std::process::exit(1);
    }

    Ok(())
}
