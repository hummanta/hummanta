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

mod args;
mod checksum;
mod package;
mod utils;

use anyhow::Result;
use clap::Parser;
use std::fs;

use self::{args::Arguments, package::package};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Arguments::parse();

    // prepare the bin directory
    let input_path = args.target_dir();
    if !input_path.exists() {
        eprintln!("Error: input directory {:?} does not exist.", input_path);
        std::process::exit(1);
    }

    // prepare the output directory
    let output_path = args.output_dir();
    if !output_path.exists() {
        fs::create_dir_all(&output_path).expect("Failed to create output directory");
    }

    let target = args.target();
    let version = args.version();

    println!("Creating archives and checksums for executables in {:?}:\n", input_path);

    // Call the package function to handle processing
    if let Err(e) = package(&input_path, &output_path, &target, &version).await {
        eprintln!("Error: Failed to package files: {}", e);
        std::process::exit(1);
    }

    println!("Done!");
    Ok(())
}
