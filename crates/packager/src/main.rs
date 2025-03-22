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

mod archive;
mod args;
mod checksum;
mod package;
mod utils;

use anyhow::Result;
use clap::Parser;

use self::{args::Arguments, package::package};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Arguments::parse();

    // prepare the output directory
    let output_dir = args.output_dir();
    if !output_dir.exists() {
        eprintln!("Error: output directory {:?} does not exist.", output_dir);
        std::process::exit(1);
    }

    let target = args.target();
    let version = args.version();

    println!("Creating archives and checksums for executables in {:?}:\n", output_dir);

    // Call the package function to handle processing
    if let Err(e) = package(&output_dir, &version, &target).await {
        eprintln!("Error: Failed to package files: {}", e);
        std::process::exit(1);
    }

    println!("Done!");
    Ok(())
}
