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
mod config;
mod context;
mod errors;
mod utils;

use std::sync::Arc;

use clap::Parser;
use cmd::Command;
use context::Context;
use errors::Result;
use tracing::error;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time() // Removes the timestamp
        .with_target(false) // remove the target (hummanta)
        .init();

    let cmd = Command::parse();
    let ctx = Context::new(&cmd.registry)?;

    if let Err(err) = cmd.exec(Arc::new(ctx)).await {
        error!("{}", err);
        std::process::exit(1);
    }

    Ok(())
}
